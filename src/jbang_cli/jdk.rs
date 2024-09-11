use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use clap::{Arg, Command};
use java_properties::PropertiesError;
use serde::Serialize;
use crate::build_jbang_app;
use crate::foojay::extract_jdk;
use crate::jbang_cli::jbang_home;

fn get_current_jdk_path() -> String {
    let current_jdk = jbang_home().join("currentjdk");
    if current_jdk.exists() && current_jdk.is_symlink() {
        return current_jdk.read_link().unwrap().to_str().unwrap().to_string();
    }
    "".to_owned()
}

#[derive(Debug, Clone, Serialize)]
struct JBangJDK {
    pub id: String,
    pub version: String, // major version
    #[serde(rename = "fullVersion")]
    pub full_version: String,
    #[serde(rename = "javaHomeDir")]
    pub java_home_dir: String,
    #[serde(rename = "providerName")]
    pub provider_name: String,
}

fn find_installed_jdks() -> Vec<JBangJDK> {
    let mut jdks: Vec<JBangJDK> = vec![];
    let jdks_path = jbang_home().join("cache").join("jdks");
    let paths = fs::read_dir(&jdks_path).unwrap();
    for entry in paths {
        if let Ok(dir_entry) = entry {
            let jdk_path = dir_entry.path();
            if jdk_path.is_dir() && jdk_path.join("release").exists() {
                if let Ok(release_info) = read_release(jdk_path.join("release").as_path()) {
                    if let Some(java_version) = release_info.get("JAVA_VERSION") {
                        let java_major = if java_version.starts_with("1.8") {
                            "8"
                        } else if java_version.contains('.') {
                            java_version.split('.').next().unwrap()
                        } else {
                            java_version
                        };
                        let full_version = if let Some(java_runtime_version) = release_info.get("JAVA_RUNTIME_VERSION") {
                            java_runtime_version.clone()
                        } else {
                            java_version.clone()
                        };
                        jdks.push(JBangJDK {
                            id: format!("{}-jbang", java_major),
                            version: java_major.to_string(),
                            full_version,
                            java_home_dir: jdk_path.to_str().unwrap().to_string(),
                            provider_name: "jbang".to_string(),
                        });
                    }
                }
            }
        }
    }
    jdks
}

pub fn manage_jdk(jdk_matches: &clap::ArgMatches) {
    let jbang_home_path = jbang_home();
    if let Some((sub_command, matches)) = jdk_matches.subcommand() {
        match sub_command {
            "default" => {
                let version = matches.get_one::<String>("version").unwrap();
                let jdk_path = jbang_home_path.join("cache").join("jdks").join(version);
                if jdk_path.exists() {
                    let current_jdk_link = jbang_home_path.join("currentjdk");
                    if current_jdk_link.exists() {
                        fs::remove_file(current_jdk_link).unwrap();
                    }
                    fs::soft_link(jdk_path, jbang_home_path.join("currentjdk")).unwrap();
                    println!("Setting default JDK to {}", version);
                } else {
                    println!("JDK {} is not installed.", version);
                }
            }
            "home" => {
                if let Some(version) = matches.get_one::<String>("version") {
                    let jdk_path = jbang_home_path.join("cache").join("jdks").join(version);
                    if jdk_path.exists() {
                        println!("{}", jdk_path.to_str().unwrap());
                    } else {
                        println!("JDK {} is not installed.", version);
                    }
                } else {
                    println!("{}", get_current_jdk_path());
                }
            }
            "install" => {
                // install JDK through jbang
                let version = matches.get_one::<String>("version").unwrap();
                extract_jdk(version, &jbang_home_path.join("cache").join("jdks").join(version));
            }
            "java-env" => {
                let version = matches.get_one::<String>("version").unwrap();
                let jbang_home = jbang_home_path.to_str().unwrap();
                println!("export PATH=\"{}/cache/jdks/{}/bin:$PATH\"", jbang_home, version);
                println!("export JAVA_HOME=\"{}/cache/jdks/{}\"", jbang_home, version);
                println!("# Run this command to configure your shell:");
                println!("# eval $(jbang jdk java-env {})", version);
            }
            "list" => {
                let available = matches.get_flag("available");
                let show_details = matches.get_flag("show-details");
                let default_format = "text".to_owned();
                let format = matches.get_one::<String>("format").unwrap_or(&default_format);
                // current jdk
                let current_jdk_path = get_current_jdk_path();
                let jdks = find_installed_jdks();
                if !jdks.is_empty() {
                    if format == "json" {
                        println!("{}", serde_json::to_string_pretty(&jdks).unwrap())
                    } else {
                        println!("Installed JDKs (<=default):");
                        for jdk in &jdks {
                            print!("  {} ({})", jdk.version, jdk.full_version);
                            if current_jdk_path == jdk.java_home_dir {
                                println!(" <");
                            } else {
                                println!();
                            }
                        }
                    }
                }
            }
            "uninstall" => {
                let version = matches.get_one::<String>("version").unwrap();
                let jdk_path = jbang_home_path.join("cache").join("jdks").join(version);
                if jdk_path.exists() {
                    fs::remove_dir_all(&jdk_path).unwrap();
                    println!("JDK {} has been uninstalled.", version);
                    let current_jdk_path = get_current_jdk_path();
                    if jdk_path.to_str().unwrap() == current_jdk_path {
                        fs::remove_file(current_jdk_path).unwrap();
                        println!("JDK {} was the current JDK, it has been removed.", version);
                    }
                } else {
                    println!("JDK {} is not installed.", version);
                }
            }
            _ => {
                println!("Unknown command: {}", sub_command);
            }
        }
    } else {
        println!("Missing required subcommand.");
    }
}

fn read_release(release_file: &Path) -> Result<HashMap<String, String>, PropertiesError> {
    // Reading
    let f = File::open(&release_file)?;
    java_properties::read(BufReader::new(f)).map(|props| {
        let mut map = HashMap::new();
        for (key, value) in props {
            map.insert(key, value.trim_matches(&['"', '\'']).to_string());
        }
        map
    })
}

pub fn build_jdk_command() -> Command {
    Command::new("jdk")
        .about("Manage Java Development Kits installed by jbang.")
        .subcommand(
            Command::new("default")
                .about("Sets the default JDK to be used by JBang.")
                .arg(
                    Arg::new("version")
                        .help("The version of the JDK to select")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("home")
                .about("Prints the folder where the given JDK is installed.")
                .arg(
                    Arg::new("version")
                        .help("The version of the JDK to select")
                        .index(1)
                        .required(false)
                )
        )
        .subcommand(
            Command::new("install")
                .about("Installs a JDK.")
                .arg(
                    Arg::new("version")
                        .help("The version or id to install")
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::new("existingJdkPath")
                        .help("Pre installed JDK path")
                        .index(2)
                        .required(false)
                )
        )
        .subcommand(
            Command::new("java-env")
                .about("Prints out the environment variables needed to use the given JDK.")
                .arg(
                    Arg::new("version")
                        .help("The version of the JDK to select")
                        .index(1)
                        .required(false)
                )
        )
        .subcommand(
            Command::new("list")
                .about("Lists installed JDKs.")
                .arg(
                    Arg::new("available")
                        .long("available")
                        .help("Shows versions available for installation")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("show-details")
                        .long("show-details")
                        .help("Shows detailed information for each JDK (only when format=text)")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("format")
                        .long("format")
                        .help("Specify output format ('text' or 'json')")
                        .num_args(1)
                        .required(false)
                        .value_parser(["text", "json"])
                )
        )
        .subcommand(
            Command::new("uninstall")
                .about("Uninstalls an existing JDK.")
                .arg(
                    Arg::new("version")
                        .help("The version to uninstall")
                        .index(1)
                        .required(true)
                )
        )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jbang_home() {
        println!("JBANG: {}", jbang_home().to_str().unwrap());
    }

    #[test]
    fn test_jdk_list() {
        let jbang_app = build_jbang_app();
        let jbang_matches = jbang_app.get_matches_from(&vec!["jbang", "jdk", "list"]);
        let jdk_matches = jbang_matches.subcommand_matches("jdk").unwrap();
        manage_jdk(&jdk_matches);
    }

    #[test]
    fn test_read_release() {
        let release_file = jbang_home().join("cache").join("jdks").join("21").join("release");
        let info = read_release(release_file.as_path()).unwrap();
        println!("{:?}", info);
    }

    #[test]
    fn read_current_jdk() {
        let current_jdk = jbang_home().join("currentjdk");
        if current_jdk.is_symlink() {
            println!("{:?}", current_jdk.read_link().unwrap());
        }
    }
}
