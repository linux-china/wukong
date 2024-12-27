use crate::sdkman_cli::{
    get_installed_candidate_default_version, get_sdkman_platform, list_candidate_names,
    sdkman_home, SDKMAN_CANDIDATES_API,
};
use colored::Colorize;

pub fn manage_list(list_matches: &clap::ArgMatches) {
    if let Some(candidate_name) = list_matches.get_one::<String>("candidate") {
        list_candidate(candidate_name);
    } else if list_matches.get_flag("local") {
        list_installed_candidates();
    } else {
        list_all_candidates();
    }
}

pub fn list_all_candidates() {
    let list_url = format!("{}/candidates/list", SDKMAN_CANDIDATES_API);
    println!("{}", wukong::common::http_text(&list_url));
}

pub fn list_candidate(candidate_name: &str) {
    let mut candidate_name = candidate_name;
    if candidate_name == "jdk" {
        candidate_name = "java";
    }
    let current_version = get_installed_candidate_default_version(candidate_name);
    let mut installed_versions: Vec<String> = vec![];
    let candidate_repo = sdkman_home().join("candidates").join(candidate_name);
    // read sub directories for candidate_repo
    if candidate_repo.exists() {
        let entries = std::fs::read_dir(candidate_repo).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            if entry.path().is_dir() {
                let version = entry.file_name().into_string().unwrap();
                if version != "current" {
                    installed_versions.push(version);
                }
            }
        }
    }
    let list_url = format!(
        "{}/candidates/{}/{}/versions/list?current={}&installed={}",
        SDKMAN_CANDIDATES_API,
        candidate_name,
        get_sdkman_platform(),
        current_version,
        installed_versions.join(",")
    );
    println!("{}", wukong::common::http_text(&list_url));
}

pub fn list_installed_candidates() {
    let candidates_dir = sdkman_home().join("candidates");
    let candidate_names = list_candidate_names();
    if candidate_names.is_empty() {
        eprintln!("No candidate installed yet.");
        return;
    }
    println!("Installed candidates:");
    for candidate_name in &candidate_names {
        let candidate_repo = candidates_dir.join(candidate_name);
        let current_version = get_installed_candidate_default_version(candidate_name);
        // list sub directories from candidate_repo
        let entries = std::fs::read_dir(candidate_repo).unwrap();
        let mut installed_versions: Vec<String> = vec![];
        for entry in entries {
            let entry = entry.unwrap();
            if entry.path().is_dir() {
                let version = entry.file_name().into_string().unwrap();
                if version != "current" {
                    if version == current_version {
                        installed_versions.insert(0, version.bold().to_string());
                    } else {
                        installed_versions.push(version);
                    }
                }
            }
        }
        println!("{}: {}", candidate_name, installed_versions.join(", "));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_candidate() {
        let candidate_name = "java";
        list_candidate(candidate_name);
    }
}
