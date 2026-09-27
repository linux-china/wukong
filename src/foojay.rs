use std::collections::HashMap;
use std::path::{Path, PathBuf};
use itertools::Itertools;
use serde::{Deserialize};
use crate::common::{extract_tgz, extract_tgz_from_sub_path, extract_zip, http_download};

pub fn get_jdk_download_url(java_version: &str) -> String {
    let distro = "temurin";
    let platform_params = get_platform_params(distro);
    let extra_query = platform_params.iter().map(|(k, v)| {
        format!("{}={}", k, v)
    }).join("&");
    format!("https://api.foojay.io/disco/v3.0/directuris?javafx_bundled=false&package_type=jdk&latest=available&version={}&{}", java_version, extra_query)
}

fn get_platform_params(distro: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    params.insert("distro".to_string(), distro.to_string());
    match std::env::consts::ARCH {
        "x86_64" => {
            params.insert("architecture".to_owned(), "x64".to_owned());
        }
        "aarch64" => {
            params.insert("architecture".to_owned(), "aarch64".to_owned());
        }
        _ => panic!("Unsupported architecture"),
    };
    if cfg!(target_os = "linux") {
        params.insert("operating_system".to_owned(), "linux".to_owned());
        params.insert("libc_type".to_owned(), "glibc".to_owned());
        params.insert("archive_type".to_owned(), "tar.gz".to_owned());
    } else if cfg!(target_os = "macos") {
        params.insert("operating_system".to_owned(), "mac".to_owned());
        params.insert("libc_type".to_owned(), "libc".to_owned());
        params.insert("archive_type".to_owned(), "tar.gz".to_owned());
    } else if cfg!(target_os = "windows") {
        params.insert("operating_system".to_owned(), "windows".to_owned());
        params.insert("libc_type".to_owned(), "c_std_lib".to_owned());
        params.insert("archive_type".to_owned(), "zip".to_owned());
    } else {
        panic!("Unsupported OS");
    };
    params
}

pub fn install_jdk(java_version: &str, target_dir: &PathBuf) {
    let download_url = get_jdk_download_url(java_version);
    let temp_dir = std::env::temp_dir();
    let mut archive_file_name = format!("jdk-{}.tar.gz", java_version);
    if cfg!(target_os = "windows") {
        archive_file_name = format!("jdk-{}.zip", java_version);
    }
    let archive_file_path = temp_dir.join(archive_file_name);
    if archive_file_path.exists() { // remove broken downloaded file
        std::fs::remove_file(&archive_file_path).unwrap();
    }
    http_download(&download_url, archive_file_path.to_str().unwrap());
    // extract into a sibling temp dir first, then rename, so an interrupted install never leaves a broken JDK at target_dir
    let parent_dir = target_dir.parent().expect("JDK target dir must have a parent");
    std::fs::create_dir_all(parent_dir).unwrap();
    let dir_name = target_dir.file_name().unwrap().to_string_lossy();
    let staging_dir = parent_dir.join(format!(".{}.installing-{}", dir_name, std::process::id()));
    if staging_dir.exists() {
        std::fs::remove_dir_all(&staging_dir).unwrap();
    }
    if cfg!(target_family = "windows") {
        extract_zip(&archive_file_path, &staging_dir, true);
    } else {
        if cfg!(target_os = "macos") {
            extract_tgz_from_sub_path(&archive_file_path, &staging_dir, "Contents/Home/");
        } else {
            extract_tgz(&archive_file_path, &staging_dir, true);
        }
    }
    if !is_jdk_installed(&staging_dir) {
        let _ = std::fs::remove_dir_all(&staging_dir);
        panic!("Failed to install JDK {}: bin/java or release file not found after extraction", java_version);
    }
    if target_dir.exists() { // remove old or broken jdk
        std::fs::remove_dir_all(&target_dir).unwrap();
    }
    std::fs::rename(&staging_dir, target_dir).unwrap();
    std::fs::remove_file(&archive_file_path).unwrap();
}

/// check whether a complete JDK is installed in the directory, not just that the directory exists
pub fn is_jdk_installed(java_home: &Path) -> bool {
    let java_exec = if cfg!(target_os = "windows") { "java.exe" } else { "java" };
    java_home.join("bin").join(java_exec).is_file() && java_home.join("release").is_file()
}

#[derive(Debug, Clone, Deserialize)]
struct PackagesResponse {
    pub result: Vec<FoojayJDK>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FoojayJDK {
    #[serde(rename = "major_version")]
    pub major_version: u32,
    #[serde(rename = "java_version")]
    pub java_version: String,
    #[serde(rename = "distribution")]
    pub distribution: String,
}

pub fn list_jdk(distro: &str, release_status: &str) -> Vec<FoojayJDK> {
    let platform_params = get_platform_params(distro);
    let extra_query = platform_params.iter().map(|(k, v)| {
        format!("{}={}", k, v)
    }).join("&");
    let url = format!("https://api.foojay.io/disco/v3.0/packages?release_status={}&package_type=jdk&latest=available&{}", release_status, extra_query);
    let mut jdks = reqwest::blocking::get(&url).unwrap().json::<PackagesResponse>().unwrap().result;
    // dedup_by only removes consecutive duplicates, so sort by major version first
    jdks.sort_by(|a, b| b.major_version.cmp(&a.major_version));
    jdks.dedup_by(|a, b| a.major_version == b.major_version);
    jdks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_download_url() {
        let java_version = "21";
        println!("{}", get_jdk_download_url(java_version));
    }

    #[test]
    fn test_list_jdk() {
        let jdks = list_jdk("temurin", "ga");
        for jdk in &jdks {
            println!("{}:{}", jdk.major_version, jdk.java_version);
        }
    }
}
