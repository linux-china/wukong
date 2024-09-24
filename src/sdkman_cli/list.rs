use crate::sdkman_cli::{get_installed_candidate_default_version, get_sdkman_platform, sdkman_home, SDKMAN_CANDIDATES_API};

pub fn manage_list(list_matches: &clap::ArgMatches) {
    if let Some(candidate_name) = list_matches.get_one::<String>("candidate") {
        list_candidate(candidate_name);
    } else {
        list_all_candidates();
    }
}

pub fn list_all_candidates() {
    let list_url = format!("{}/candidates/list", SDKMAN_CANDIDATES_API);
    println!("{}", wukong::common::http_text(&list_url));
}

pub fn list_candidate(candidate_name: &str) {
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
    let list_url = format!("{}/candidates/{}/{}/versions/list?current={}&installed={}",
                           SDKMAN_CANDIDATES_API,
                           candidate_name,
                           get_sdkman_platform(),
                           current_version,
                           installed_versions.join(",")
    );
    println!("{}", wukong::common::http_text(&list_url));
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
