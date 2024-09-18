use crate::common::{extract_tgz, extract_tgz_from_sub_path, extract_zip, get_redirect_url, http_download};
use crate::sdkman_cli::{find_candidate_home, get_remote_candidate_default_version, get_sdkman_platform, SDKMAN_CANDIDATES_API};

pub fn manage_install(install_matches: &clap::ArgMatches) {
    if let Some(candidate_name) = install_matches.get_one::<String>("candidate") {
        let candidate_version = if let Some(version) = install_matches.get_one::<String>("version") {
            version.clone()
        } else {
            get_remote_candidate_default_version(candidate_name)
        };
        if candidate_version == "" {
            eprintln!("Failed to find default version for : {}", candidate_name);
            return;
        }
        install_candidate(candidate_name, &candidate_version);
    } else {
        println!("No candidate supplied!");
        println!("Please use `sdk install candidate_name` to install candidate.")
    }
}

pub fn install_candidate(candidate_name: &str, candidate_version: &str) {
    let candidate_home = find_candidate_home(candidate_name, candidate_version);
    if candidate_home.exists() {
        println!("{}@{} installed already: {}", candidate_name, candidate_version, candidate_home.to_str().unwrap());
        return;
    }
    let sdkman_platform = get_sdkman_platform();
    let download_url = format!("{}/broker/download/{}/{}/{}", SDKMAN_CANDIDATES_API,
                               candidate_name, candidate_version,
                               sdkman_platform);
    let real_download_url = get_redirect_url(&download_url).unwrap();
    let archive_file_name = &real_download_url[real_download_url.rfind("/").unwrap() + 1..];
    let temp_dir = std::env::temp_dir();
    let archive_file_path = temp_dir.join(archive_file_name);
    if !archive_file_path.exists() {
        http_download(&real_download_url, archive_file_path.to_str().unwrap());
    }
    if archive_file_name.ends_with("tar.gz") {
        if candidate_name == "java" && sdkman_platform.starts_with("darwin") { // JDK on Mac
            extract_tgz_from_sub_path(&archive_file_path, &candidate_home, "Contents/Home/");
        } else {
            extract_tgz(&archive_file_path, &candidate_home, true);
        }
    } else {
        extract_zip(&archive_file_path, &candidate_home, true);
    }
    std::fs::remove_file(&archive_file_path).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::sdkman_cli::get_remote_candidate_default_version;
    use super::*;

    #[test]
    fn test_candidate_default_version() {
        let candidate_name = "java";
        println!("java: {}", get_remote_candidate_default_version(candidate_name));
    }

    #[test]
    fn test_install_candidate() {
        let candidate_name = "ant";
        let version = "1.10.14";
        install_candidate(candidate_name, version);
    }

    #[test]
    fn test_install_java_candidate() {
        let candidate_name = "java";
        let version = "22.0.2-tem";
        install_candidate(candidate_name, version);
    }
}
