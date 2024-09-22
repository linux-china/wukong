use std::path::PathBuf;
use colored::Colorize;
use crate::common::{jbang_home, sdkman_home};
use crate::mt_cli::models::Toolchains;
use crate::sdkman_cli::list::list_candidate;

pub mod models;
pub mod app;

pub fn m2_dir() -> PathBuf {
    dirs::home_dir().unwrap().join(".m2")
}

pub fn jdks_command() {
    // list all JDK on host
    let jbang_home = jbang_home();
    if jbang_home.exists() {
        let jdks = jbang_home.join("cache").join("jdks");
        if jdks.exists() {
            println!("===== JBang jdks =====");
            jdks.read_dir().unwrap().for_each(|entry| {
                if let Ok(entry) = entry {
                    let java_version = entry.file_name();
                    println!("{}:\n {}", java_version.to_str().unwrap().yellow(), entry.path().display());
                }
            });
        }
    }
    let sdkman_home = sdkman_home();
    if sdkman_home.exists() {
        let jdks = sdkman_home.join("candidates").join("java");
        if jdks.exists() {
            println!("===== SDKMAN jdks =====");
            jdks.read_dir().unwrap().for_each(|entry| {
                if let Ok(entry) = entry {
                    let java_version = entry.file_name();
                    if java_version != "current" {
                        println!("{}:\n {}", java_version.to_str().unwrap().yellow(), entry.path().display());
                    }
                }
            });
        }
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
}
