use std::path::{Path, PathBuf};
use colored::Colorize;
use wukong::common::{is_java_home, jbang_home, sdkman_home};
use crate::mt_cli::models::Toolchains;
use crate::sdkman_cli;
use crate::sdkman_cli::list::list_candidate;

pub mod models;
pub mod clap_app;

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
                    lines.push(format!("{}:\n {}", java_version.to_str().unwrap(), entry.path().display()));
                }
            }
        }
    });
    lines.sort_by(|a, b| compare_java_version(a, b));
    lines
}

fn compare_java_version(version1: &str, version2: &str) -> core::cmp::Ordering {
    let major_version1 = extract_major_version(version1);
    let major_version2 = extract_major_version(version2);
    if major_version1.parse::<u32>().is_ok() && major_version2.parse::<u32>().is_ok() {
        return major_version1.parse::<u32>().unwrap().cmp(&major_version2.parse::<u32>().unwrap());
    }
    major_version1.cmp(major_version2)
}

fn extract_major_version(version: &str) -> &str {
    let version = if version.contains(':') {
        &version[..version.find(':').unwrap()]
    } else {
        version
    };
    if version.starts_with("1.") {
        let version = &version[2..];
        &version[..version.find('.').unwrap()]
    } else if version.contains('.') {
        &version[..version.find('.').unwrap()]
    } else if version.contains('-') {
        &version[..version.find('-').unwrap()]
    } else {
        version
    }
}

pub fn list_command() {
    let toolchains = Toolchains::load();
    for toolchain in &toolchains.toolchain {
        if toolchain.type_ == "jdk" {
            let provides = &toolchain.provides;
            let jdk_home = toolchain.configuration.get("jdkHome").cloned().unwrap_or("".to_string());
            if !jdk_home.is_empty() {
                let vendor = &provides.vendor;
                let jdk_home_path = PathBuf::from(&jdk_home);
                let jdk_home_text = if jdk_home_path.exists() {
                    format!("{}", jdk_home.red())
                } else {
                    format!("{}", jdk_home)
                };
                if vendor.is_some() && !vendor.as_ref().unwrap().is_empty() {
                    println!("{}: {}\n  {}", provides.version.bold(), vendor.clone().unwrap(), jdk_home_text);
                } else {
                    println!("{}:\n  {}", provides.version.bold(), jdk_home_text);
                };
            }
        }
    }
}

pub fn add_command(command_matches: &clap::ArgMatches) {
    let mut version = command_matches.get_one::<String>("version").unwrap();
    let mut vendor = command_matches.get_one::<String>("vendor").map(|v| v.to_string());
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
            let installed_path = install_jdk(version);
            Some(installed_path.to_str().unwrap().to_string())
        } else {
            Some(java_home.to_str().unwrap().to_string())
        }
    } else { // sdkman
        let java_home = sdkman_home().join("candidates").join("java").join(version);
        let result = if !java_home.exists() {
            let installed_path = install_jdk(version);
            Some(installed_path.to_str().unwrap().to_string())
        } else {
            Some(java_home.to_str().unwrap().to_string())
        };
        result
    };
    if jdk_home.is_none() {
        println!("JDK not found: {}, please use sdkman or jbang to install it first.", version);
        return;
    }
    let mut toolchains = Toolchains::load();
    if version.contains('-') {
        let parts = version.split('-').collect::<Vec<&str>>();
        let version = &parts[0].to_string();
        let vendor = Some(parts[1].to_string());
        toolchains.add_jdk(&version, vendor, jdk_home.unwrap());
    } else {
        toolchains.add_jdk(&version, vendor, jdk_home.unwrap());
    }
    toolchains.write();
}

pub fn install_jdk(version: &str) -> PathBuf {
    if version.parse::<u32>().is_ok() { // jbang
        let java_home = jbang_home().join("cache").join("jdks").join(version);
        wukong::foojay::install_jdk(version, &java_home);
        java_home
    } else { // SDKMAN
        sdkman_cli::install::install_candidate("java", version);
        sdkman_home().join("candidates").join("java").join(version)
    }
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

    #[test]
    fn test_extract_major_version() {
        let version = "17.0.7-graalce:\n ~/.sdkman/candidates/java/17.0.7-graalce";
        println!("{}", extract_major_version(version));
    }
}
