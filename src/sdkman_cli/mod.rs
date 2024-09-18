use std::path::PathBuf;
use crate::common::http_text;

pub mod list;
pub mod install;
pub mod default;
pub mod use_candidate;
pub mod uninstall;
pub mod home;
pub mod current;
pub mod env;

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
    if let Ok(jbang_home) = std::env::var("SDKMAN_DIR") {
        PathBuf::from(jbang_home)
    } else {
        dirs::home_dir().unwrap().join(".sdkman")
    }
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
    http_text(&default_version_url).trim().to_string()
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
        println!("{}", get_candidate_default_version(candidate_name));
    }
}
