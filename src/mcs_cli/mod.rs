use pad::PadStr;
use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mcs_cli::clap_app::build_mcs_app;

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
}
