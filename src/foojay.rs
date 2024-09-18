use std::path::{PathBuf};
use crate::common::{extract_tgz, extract_tgz_from_sub_path, extract_zip, http_download};

pub fn get_jdk_download_url(java_version: &str) -> String {
    let distro = "temurin";
    let mut os = "linux";
    let arch = match std::env::consts::ARCH {
        "x86_64" => "x64",
        "aarch64" => "aarch64",
        _ => panic!("Unsupported architecture"),
    };
    let mut archive_type = "tar.gz";
    let mut libc_type = "glibc";
    if cfg!(target_os = "linux") {
        os = "linux";
    } else if cfg!(target_os = "macos") {
        os = "mac";
        libc_type = "libc";
    } else if cfg!(target_os = "windows") {
        os = "windows";
        libc_type = "c_std_lib";
        archive_type = "zip";
    } else {
        panic!("Unsupported OS");
    };
    format!("https://api.foojay.io/disco/v3.0/directuris?distro={distro}&javafx_bundled=false&libc_type={libc_type}&archive_type={archive_type}&operating_system={os}&package_type=jdk&version={java_version}&architecture={arch}&latest=available")
}

pub fn install_jdk(java_version: &str, target_dir: &PathBuf) {
    let download_url = get_jdk_download_url(java_version);
    let temp_dir = std::env::temp_dir();
    let mut archive_file_name = format!("jdk-{}.tar.gz", java_version);
    if cfg!(target_os = "windows") {
        archive_file_name = format!("jdk-{}.zip", java_version);
    }
    let archive_file_path = temp_dir.join(archive_file_name);
    http_download(&download_url, archive_file_path.to_str().unwrap());
    if target_dir.exists() { // remove old jdk version
        std::fs::remove_dir_all(&target_dir).unwrap();
    }
    if cfg!(target_family = "windows") {
        extract_zip(&archive_file_path, target_dir, true);
    } else {
        if cfg!(target_os = "macos") {
            extract_tgz_from_sub_path(&archive_file_path, target_dir, "Contents/Home/");
        } else {
            extract_tgz(&archive_file_path, target_dir, true);
        }
    }
    std::fs::remove_file(&archive_file_path).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_download_url() {
        let java_version = "21";
        println!("{}", get_jdk_download_url(java_version));
    }
}
