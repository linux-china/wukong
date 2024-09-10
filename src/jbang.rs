//! clap App for command cli

mod common;
mod foojay;

use std::collections::HashMap;
use std::fs;
use std::fs::{read, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use clap::{Command, Arg, ArgAction};
use java_properties::PropertiesError;
use serde::Serialize;
use crate::common::run_command;
use crate::foojay::extract_jdk;

pub const VERSION: &str = "0.1.0";

fn main() {
    let app = build_jbang_app();
    let matches = app.get_matches();
    if let Some((command, command_matches)) = matches.subcommand() {
        if command == "jdk" {
            manage_jdk(command_matches);
        }
    }
}


fn jbang_home() -> PathBuf {
    if let Ok(jbang_home) = std::env::var("JBANG_DIR") {
        PathBuf::from(jbang_home)
    } else {
        dirs::home_dir().unwrap().join(".jbang")
    }
}

fn jbang_exec() -> PathBuf {
    let jbang_bin_dir = jbang_home().join("bin");
    if cfg!(windows) {
        jbang_bin_dir.join("jbang.cmd")
    } else {
        jbang_bin_dir.join("jbang")
    }
}

fn get_current_jdk_path() -> String {
    let current_jdk = jbang_home().join("currentjdk");
    if current_jdk.exists() && current_jdk.is_symlink() {
        return current_jdk.read_link().unwrap().to_str().unwrap().to_string();
    }
    "".to_owned()
}

fn get_jdk_path(version: &str) -> PathBuf {
    jbang_home().join("cache").join("jdks").join(version)
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
                            full_version: full_version,
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

fn manage_jdk(jdk_matches: &clap::ArgMatches) {
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
    java_properties::read(BufReader::new(f)).map(|(props)| {
        let mut map = HashMap::new();
        for (key, value) in props {
            map.insert(key, value.trim_matches(&['"', '\'']).to_string());
        }
        map
    })
}

pub fn build_jbang_app() -> Command {
    let run_command = Command::new("run")
        .about("Builds and runs provided script.")
        .arg(
            Arg::new("main")
                .short('m')
                .long("main")
                .help("Main class to use when running. Used primarily for running jar's.")
                .required(true)
        )
        .arg(
            Arg::new("scriptOrFile")
                .help("A reference to a source file")
                .index(1)
                .required(false)
        )
        .arg(
            Arg::new("userParams")
                .help("Parameters to pass on to the script.")
                .required(false)
                .index(2)
                .num_args(1..)
        );
    let build_command = Command::new("build")
        .about("Compiles and stores script in the cache.")
        .arg(
            Arg::new("build-dir")
                .long("build-dir")
                .num_args(1)
                .help("Use given directory for build results")
                .required(false)
        )
        .arg(
            Arg::new("scriptOrFile")
                .help("A reference to a source file")
                .index(1)
                .required(false)
        );
    let init_command = Command::new("init")
        .about("Builds and runs provided script.")
        .arg(
            Arg::new("template")
                .short('t')
                .long("template")
                .num_args(1)
                .help("Init script with a java class useful for scripting")
                .required(true)
        )
        .arg(
            Arg::new("scriptOrFile")
                .help(" file or URL to a Java code file")
                .index(1)
                .required(false)
        )
        .arg(
            Arg::new("params")
                .help("Parameters to pass on to the generation.")
                .required(false)
                .index(2)
                .num_args(1..)
        );
    let jdk_command = Command::new("jdk")
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
        );
    Command::new("jbang")
        .version(VERSION)
        .about("jbang - Unleash the power of Java")
        .arg(
            Arg::new("config")
                .long("config")
                .num_args(1)
                .help("Path to config file to be used instead of the default")
                .required(false),
        )
        .arg(
            Arg::new("fresh")
                .long("fresh")
                .action(ArgAction::SetTrue)
                .help("Make sure we use fresh (i.e. non-cached) resources.")
                .required(false),
        )
        .arg(
            Arg::new("insecure")
                .long("insecure")
                .action(ArgAction::SetTrue)
                .help("Enable insecure trust of all SSL certificates.")
                .required(false),
        )
        .arg(
            Arg::new("offline")
                .short('o')
                .long("offline")
                .action(ArgAction::SetTrue)
                .help("Work offline. Fail-fast if dependencies are missing.")
                .required(false),
        )
        .arg(
            Arg::new("preview")
                .long("preview")
                .action(ArgAction::SetTrue)
                .help("Enable jbang preview features.")
                .required(false),
        )
        .arg(
            Arg::new("quiet")
                .long("quiet")
                .action(ArgAction::SetTrue)
                .help("jbang will be quiet, only print when error occurs.")
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("jbang will be verbose on what it does.")
                .required(false),
        )
        .subcommand(run_command)
        .subcommand(build_command)
        .subcommand(init_command)
        .subcommand(jdk_command)
}

#[cfg(test)]
mod tests {
    use std::fs;
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
