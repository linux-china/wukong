use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub mod list;
pub mod install;
pub mod default;
pub mod use_candidate;
pub mod uninstall;
pub mod home;
pub mod current;
pub mod env;
pub mod upgrade;
pub mod clap_app;
pub mod direnv;
pub mod init;

const SDKMAN_CANDIDATES_API: &str = "https://api.sdkman.io/2";
pub fn get_sdkman_platform() -> String {
    let os_name = match std::env::consts::OS {
        "macos" => "darwin",
        "windows" => "windows",
        "linux" => "linux",
        &_ => ""
    };
    let arch_name = match std::env::consts::ARCH {
        "aarch64" => "arm64",
        "x86_64" => "x64",
        &_ => "exotic"
    };
    if arch_name == "exotic" {
        "exotic".to_owned()
    } else {
        format!("{}{}", os_name, arch_name)
    }
}

pub fn sdkman_home() -> PathBuf {
    wukong::common::sdkman_home()
}

pub fn read_sdkman_config() -> HashMap<String, String> {
    let config_file_path = sdkman_home().join("etc").join("config");
    if config_file_path.exists() {
        let f2 = File::open(&config_file_path).unwrap();
        return java_properties::read(BufReader::new(f2)).unwrap();
    }
    HashMap::new()
}

pub fn list_candidate_names() -> Vec<String> {
    let mut candidate_names: Vec<String> = vec![];
    let candidates_dir = sdkman_home().join("candidates");
    // read sub directories for candidates_dir
    if candidates_dir.exists() {
        let entries = std::fs::read_dir(candidates_dir).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            if entry.path().is_dir() {
                let candidate_name = entry.file_name().into_string().unwrap();
                candidate_names.push(candidate_name);
            }
        }
    }
    candidate_names
}

pub fn find_candidate_home(candidate_name: &str, candidate_version: &str) -> PathBuf {
    sdkman_home().join("candidates")
        .join(candidate_name).join(candidate_version)
}

pub fn get_installed_candidate_default_version(candidate_name: &str) -> String {
    let candidate_current_link = sdkman_home().join("candidates").join(candidate_name).join("current");
    if candidate_current_link.exists() {
        let link_target_path = candidate_current_link.read_link().unwrap();
        return link_target_path.file_name().unwrap().to_str().unwrap().to_string();
    }
    "".to_owned()
}

pub fn get_remote_candidate_default_version(candidate_name: &str) -> String {
    let default_version_url = format!("{}/candidates/default/{}", SDKMAN_CANDIDATES_API, candidate_name);
    wukong::common::http_text(&default_version_url).trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdkman_platform() {
        println!("{}", get_sdkman_platform());
    }

    #[test]
    fn test_get_candidate_default_version() {
        let candidate_name = "java";
        println!("{}", get_installed_candidate_default_version(candidate_name));
    }

    #[test]
    fn test_read_config() {
        println!("{:?}", read_sdkman_config());
    }
}
