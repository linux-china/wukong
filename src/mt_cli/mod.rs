use std::fmt::format;
use std::path::PathBuf;
use colored::Colorize;
use crate::common::{is_java_home, jbang_home, sdkman_home};
use crate::mt_cli::models::Toolchains;
use crate::sdkman_cli::list::list_candidate;

pub mod models;
pub mod app;

pub fn m2_dir() -> PathBuf {
    dirs::home_dir().unwrap().join(".m2")
}

pub fn jdks_command() {
    // list all JDKs on from JBang
    let jbang_home = jbang_home();
    if jbang_home.exists() {
        let jdks = jbang_home.join("cache").join("jdks");
        if jdks.exists() {
            let lines = list_jdks(&jdks);
            print_jdks(&lines, "JBang");
        }
    }
    // list all JDKs from SDKMAN
    let sdkman_home = sdkman_home();
    if sdkman_home.exists() {
        let jdks = sdkman_home.join("candidates").join("java");
        if jdks.exists() {
            let lines = list_jdks(&jdks);
            print_jdks(&lines, "SDKMAN");
        }
    }
    // list all JDKs from Gradle
    let gradle_jdks = dirs::home_dir().unwrap().join(".gradle").join("jdks");
    if gradle_jdks.exists() {
        let lines = list_jdks(&gradle_jdks);
        print_jdks(&lines, "Gradle");
    }
    if cfg!(target_os = "macos") {
        // list all JDKs from /Library/Java/JavaVirtualMachines
        let jdks = PathBuf::from("/Library/Java/JavaVirtualMachines");
        if jdks.exists() {
            let lines = list_jdks(&gradle_jdks);
            print_jdks(&lines, "System");
        }
        let jdks = dirs::home_dir().unwrap().join("Library").join("Java").join("JavaVirtualMachines");
        if jdks.exists() {
            let lines = list_jdks(&jdks);
            print_jdks(&lines, "User");
        }
        // homebrew
        let cellar_dirs = ["/opt/homebrew/Cellar", "/usr/local/Cellar"];
        let mut lines: Vec<String> = Vec::new();
        for cellar_dir in cellar_dirs.iter() {
            let cellar_path = PathBuf::from(*cellar_dir);
            if cellar_path.exists() {
                cellar_path.read_dir().unwrap().for_each(|entry| {
                    if let Ok(entry) = entry {
                        let child = entry.path();
                        let file_name = child.file_name().unwrap().to_str().unwrap();
                        if file_name.starts_with("openjdk") {
                            lines.extend(list_jdks(&child))
                        }
                    }
                });
            }
        }
        print_jdks(&lines, "Homebrew");
    } else if cfg!(target_os = "windows") {
        // list all JDKs from C:\Program Files\Java
        let jdks = PathBuf::from("C:\\Program Files\\Java");
        if jdks.exists() {
            let lines = list_jdks(&jdks);
            print_jdks(&lines, "System");
        }
    } else if cfg!(target_os = "linux") {
        // list all JDKs from /usr/lib/jvm
        let jdks = PathBuf::from("/usr/lib/jvm");
        if jdks.exists() {
            let lines = list_jdks(&jdks);
            print_jdks(&lines, "System");
        }
    }
}

fn print_jdks(lines: &[String], title: &str) {
    if lines.len() > 0 {
        println!("===== {} JDKs =====", title);
        for line in lines {
            println!("{}", line);
        }
    }
}

fn list_jdks(base_path: &PathBuf) -> Vec<String> {
    let mut lines = Vec::new();
    base_path.read_dir().unwrap().for_each(|entry| {
        if let Ok(entry) = entry {
            let child = entry.path();
            if child.is_dir() && is_java_home(&child) {
                let java_version = entry.file_name();
                if java_version != "current" {
                    lines.push(format!("{}:\n {}", java_version.to_str().unwrap().yellow(), entry.path().display()));
                }
            }
        }
    });
    lines
}

pub fn list_command() {
    let toolchains = Toolchains::load();
    for toolchain in &toolchains.toolchain {
        if toolchain.type_ == "jdk" {
            let provides = &toolchain.provides;
            let jdk_home = toolchain.configuration.get("jdkHome").cloned().unwrap_or("".to_string());
            if !jdk_home.is_empty() {
                let vendor = &provides.vendor;
                if vendor.is_some() && !vendor.as_ref().unwrap().is_empty() {
                    println!("{}: {}\n  {}", provides.version, vendor.clone().unwrap(), jdk_home);
                } else {
                    println!("{}:\n  {}", provides.version, jdk_home);
                };
            }
        }
    }
}

pub fn add_command(command_matches: &clap::ArgMatches) {
    let version = command_matches.get_one::<String>("version").unwrap();
    let vendor = command_matches.get_one::<String>("vendor").map(|v| v.to_string());
    let jdk_path = command_matches.get_one::<String>("path");
    let jdk_home = if let Some(java_home) = jdk_path { // add jdk from path
        if !PathBuf::from(java_home).exists() {
            println!("Path not exists: {}", java_home);
            None
        } else {
            Some(java_home.clone())
        }
    } else if let Ok(_) = version.parse::<u32>() { // jbang JDK
        let java_home = jbang_home().join("cache").join("jdks").join(version);
        if !java_home.exists() {
            None
        } else {
            Some(java_home.to_str().unwrap().to_string())
        }
    } else { // sdkman
        let java_home = sdkman_home().join("candidates").join("java").join(version);
        if !java_home.exists() {
            None
        } else {
            Some(java_home.to_str().unwrap().to_string())
        }
    };
    if jdk_home.is_none() {
        println!("JDK not found: {}, please use sdkman or jbang to install it first.", version);
        return;
    }
    let mut toolchains = Toolchains::load();
    toolchains.add_jdk(version, vendor, jdk_home.unwrap());
    toolchains.write();
}

pub fn remove_command(command_matches: &clap::ArgMatches) {
    let version = command_matches.get_one::<String>("version").unwrap();
    let vendor = command_matches.get_one::<String>("vendor").map(|v| v.to_string());
    let mut toolchains = Toolchains::load();
    toolchains.remove_jdk(&version, vendor);
    toolchains.write();
}

pub fn vendors_command() {
    list_candidate("java");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_command() {
        list_command();
    }

    #[test]
    fn test_jdks() {
        jdks_command();
    }
}
