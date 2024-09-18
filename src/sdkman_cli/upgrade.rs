use crate::sdkman_cli::{find_candidate_home, get_remote_candidate_default_version, sdkman_home};
use crate::sdkman_cli::install::install_candidate;

pub fn manage_upgrade(upgrade_matches: &clap::ArgMatches) {
    if let Some(candidate_name) = upgrade_matches.get_one::<String>("candidate") {
        upgrade_candidate(candidate_name);
    } else {
        // list all candidates
        let candidates_dir = sdkman_home().join("candidates");
        // read sub directories for candidates_dir
        if candidates_dir.exists() {
            let entries = std::fs::read_dir(candidates_dir).unwrap();
            for entry in entries {
                let entry = entry.unwrap();
                if entry.path().is_dir() {
                    let candidate_name = entry.file_name().into_string().unwrap();
                    upgrade_candidate(&candidate_name);
                }
            }
        }
    }
}

fn upgrade_candidate(candidate_name: &str) {
    let default_remote_version = get_remote_candidate_default_version(candidate_name);
    if default_remote_version == "" {
        eprintln!("Failed to find default version for : {}", candidate_name);
        return;
    }
    let candidate_home = find_candidate_home(candidate_name, &default_remote_version);
    if !candidate_home.exists() {
        println!("Begin to upgrade {} to {}", candidate_name, default_remote_version);
        install_candidate(candidate_name, &default_remote_version);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upgrade_candidate() {
        let candidate_name = "ant";
        upgrade_candidate(candidate_name);
    }
}
