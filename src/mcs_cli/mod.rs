use colored::Colorize;
use pad::PadStr;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::path::Path;

pub mod clap_app;

#[derive(Debug, Serialize, Deserialize)]
pub struct McsResult {
    pub response: McsResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct McsResponse {
    #[serde(rename = "numFound")]
    pub num_found: u32,
    pub start: u32,
    pub docs: Option<Vec<McsDoc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct McsDoc {
    pub id: String,
    pub g: String,
    pub a: String,
    pub v: Option<String>,
    #[serde(rename = "latestVersion")]
    pub latest_version: Option<String>,
    pub p: String,
    pub timestamp: u64,
}

impl McsDoc {
    pub fn last_updated(&self) -> String {
        let date = chrono::DateTime::from_timestamp(self.timestamp as i64 / 1000, 0).unwrap();
        date.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn get_result_id(&self, format: &str) -> String {
        match format {
            "jbang" => {
                format!(
                    "//DEPS {}:{}",
                    self.id,
                    self.latest_version.clone().unwrap_or("".to_string())
                )
            }
            "sbt" => {
                format!(
                    "\"{}\" % \"{}\" % \"{}\"",
                    self.g,
                    self.a,
                    self.latest_version.clone().unwrap_or("".to_string())
                )
            }
            "leiningen" => {
                format!(
                    "[{} {}/{} \"{}\"]",
                    self.id,
                    self.g,
                    self.a,
                    self.latest_version.clone().unwrap_or("".to_string())
                )
            }
            "ivy" => {
                format!(
                    "<dependency org=\"{}\" name=\"{}\" rev=\"{}\" />",
                    self.g,
                    self.a,
                    self.latest_version.clone().unwrap_or("".to_string())
                )
            }
            "grape" => {
                format!(
                    "@Grab(group='{}', module='{}', version='{}')",
                    self.g,
                    self.a,
                    self.latest_version.clone().unwrap_or("".to_string())
                )
            }
            "maven" => {
                format!(
                    r#"
  <dependency>
    <groupId>{}</groupId>
    <artifactId>{}</artifactId>
    <version>{}</version>
  </dependency>"#,
                    self.g,
                    self.a,
                    self.latest_version.clone().unwrap_or("".to_string())
                )
            }
            &_ => {
                format!(
                    "{}:{}",
                    self.id,
                    self.latest_version.clone().unwrap_or("".to_string())
                )
            }
        }
    }
}

pub fn search(command_matches: &clap::ArgMatches) {
    let query = command_matches.get_one::<String>("query").unwrap();
    let default_format = "gav".to_owned();
    let format = command_matches
        .get_one::<String>("format")
        .unwrap_or(&default_format);
    let limit = command_matches.get_one::<u32>("limit").unwrap_or(&20);
    let url = format!(
        "https://search.maven.org/solrsearch/select?q={}&rows={}&wt=json",
        query, limit
    );
    println!("Searching for containing {}...", query);
    let client = reqwest::blocking::Client::new();
    let result = client
        .get(&url)
        .header(reqwest::header::USER_AGENT, "curl/8.7.1")
        .send()
        .unwrap()
        .json::<McsResult>()
        .unwrap();
    let limit1 = *limit;
    println!(
        "Found {} results (showing {})",
        result.response.num_found, limit1
    );
    if let Some(docs) = &result.response.docs {
        let max_len = docs
            .iter()
            .map(|doc| doc.get_result_id(format).len())
            .max()
            .unwrap();
        if format == "maven" {
            println!("  {}", "Coordinates");
            println!("  {}", "===========");
        } else {
            println!(
                "  {}  {}",
                "Coordinates".pad_to_width(max_len),
                "Last Updated"
            );
            println!(
                "  {}  {}",
                "===========".pad_to_width(max_len),
                "==================="
            );
        }
        for doc in docs {
            let id = doc.get_result_id(format);
            if format == "maven" {
                // multi lines
                println!("  {}\n  {}\n", doc.last_updated(), id.trim());
            } else {
                println!("  {}  {}", id.pad_to_width(max_len), doc.last_updated());
            }
        }
    }
}

pub fn class_search(command_matches: &clap::ArgMatches) {
    let query = command_matches.get_one::<String>("query").unwrap();
    let limit = command_matches.get_one::<u32>("limit").unwrap_or(&20);
    let url = if command_matches.get_flag("full-name") {
        format!(
            "https://search.maven.org/solrsearch/select?q=c:{}&rows={}&wt=json",
            query, limit
        )
    } else {
        format!(
            "https://search.maven.org/solrsearch/select?q=c:{}&rows={}&wt=json",
            query, limit
        )
    };
    println!("Searching for artifacts containing {}...", query);
    let client = reqwest::blocking::Client::new();
    let result = client
        .get(&url)
        .header(reqwest::header::USER_AGENT, "curl/8.7.1")
        .send()
        .unwrap()
        .json::<McsResult>()
        .unwrap();
    let limit1 = *limit;
    println!(
        "Found {} results (showing {})",
        result.response.num_found, limit1
    );
    if let Some(docs) = &result.response.docs {
        let max_len = docs.iter().map(|doc| doc.id.len()).max().unwrap();
        println!(
            "  {} {}",
            "Coordinates".pad_to_width(max_len),
            "Last Updated"
        );
        println!(
            "  {} {}",
            "===========".pad_to_width(max_len),
            "==================="
        );
        for doc in docs {
            println!("  {}  {}", doc.id.pad_to_width(max_len), doc.last_updated());
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Project {
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "artifactId")]
    pub artifact_id: Option<String>,
    pub version: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
}

impl Project {
    pub fn load(url: &str) -> Self {
        let response = reqwest::blocking::get(url).unwrap();
        if !response.status().is_success() {
            panic!("Failed to fetch {}", url);
        }
        let xml_code = response.text().unwrap();
        quick_xml::de::from_str(&xml_code).unwrap()
    }

    pub fn parse(xml_code: &str) -> Self {
        quick_xml::de::from_str(&xml_code).unwrap()
    }

    pub fn gav(&self) -> Option<String> {
        if self.group_id.is_some() && self.artifact_id.is_some() && self.version.is_some() {
            return Some(format!(
                "{}:{}:{}",
                self.group_id.clone().unwrap(),
                self.artifact_id.clone().unwrap(),
                self.version.clone().unwrap()
            ));
        }
        None
    }
}

pub fn info(command_matches: &clap::ArgMatches) {
    let gav = command_matches.get_one::<String>("gav").unwrap();
    if gav.ends_with(".jar") {
        jar_info(gav);
        return;
    }
    let parts = gav.split(':').collect::<Vec<&str>>();
    let url = format!(
        "https://repo1.maven.org/maven2/{}/{}/{}/{}-{}.pom",
        parts[0].replace('.', "/"),
        parts[1],
        parts[2],
        parts[1],
        parts[2]
    );
    let project = Project::load(&url);
    if let Some(name) = &project.name {
        println!("{}: {}", "Name".bold(), name);
    }
    if let Some(description) = &project.description {
        println!("{}: {}", "Description".bold(), description);
    }
    if let Some(url) = &project.url {
        println!("{}: {}", "URL".bold(), url);
    }
    let artifact_url = format!(
        "https://repo1.maven.org/maven2/{}/{}/{}/",
        parts[0].replace('.', "/"),
        parts[1],
        parts[2],
    );
    println!("{}: {}", "Repository URL".bold(), artifact_url);
}

pub fn jar_info<P: AsRef<Path>>(jar_path: P) {
    let jar_file = std::fs::File::open(jar_path).unwrap();
    let mut zip = zip::ZipArchive::new(jar_file).unwrap();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let file_name = file.name();
        if file_name.ends_with("/pom.xml") {
            let mut xml_code = String::new();
            file.read_to_string(&mut xml_code).unwrap();
            let project = Project::parse(&xml_code);
            if let Some(gav) = project.gav() {
                println!("{}: {}", "GAV".bold(), gav);
                let artifact_url = format!(
                    "https://repo1.maven.org/maven2/{}/{}/{}/",
                    project.group_id.unwrap(),
                    project.artifact_id.unwrap(),
                    project.version.unwrap(),
                );
                println!("{}: {}", "Repository URL".bold(), artifact_url);
            }
            if let Some(name) = &project.name {
                println!("{}: {}", "Name".bold(), name);
            }
            if let Some(description) = &project.description {
                println!("{}: {}", "Description".bold(), description);
            }
            if let Some(url) = &project.url {
                println!("{}: {}", "URL".bold(), url);
            }
            return;
        }
    }
    println!("Failed to find pom.xml in the jar.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcs_cli::clap_app::build_mcs_app;
    use dirs::home_dir;

    #[test]
    fn test_class_search() {
        let mcs_app = build_mcs_app();
        let mcs_matches = mcs_app.get_matches_from(&vec!["mcs", "class-search", "VelocityEngine"]);
        let class_search_matches = mcs_matches.subcommand_matches("class-search").unwrap();
        class_search(class_search_matches);
    }

    #[test]
    fn test_search() {
        let mcs_app = build_mcs_app();
        let mcs_matches =
            mcs_app.get_matches_from(&vec!["mcs", "search", "spring-messaging", "--format=maven"]);
        let class_search_matches = mcs_matches.subcommand_matches("search").unwrap();
        search(class_search_matches);
    }

    #[test]
    fn test_info() {
        let mcs_app = build_mcs_app();
        let mcs_matches = mcs_app.get_matches_from(&vec![
            "mcs",
            "info",
            "org.apache.commons:commons-lang3:3.17.0",
        ]);
        let info_matches = mcs_matches.subcommand_matches("info").unwrap();
        info(info_matches);
    }

    #[test]
    fn test_jar_info() {
        let jar_file = home_dir()
            .unwrap()
            .join(".m2")
            .join("repository")
            .join("commons-io")
            .join("commons-io")
            .join("2.18.0")
            .join("commons-io-2.18.0.jar");
        jar_info(jar_file);
    }

    #[test]
    fn test_parse_pom() {
        let url = " https://repo1.maven.org/maven2/org/apache/commons/commons-lang3/3.17.0/commons-lang3-3.17.0.pom";
        let pom = Project::load(url);
        println!("{:?}", pom);
    }
}
