use crate::sdkman_cli::{find_candidate_home};

pub fn manage_default(default_matches: &clap::ArgMatches) {
    let candidate_name = default_matches.get_one::<String>("candidate").unwrap();
    let candidate_version = default_matches.get_one::<String>("version").unwrap();
    let candidate_home = find_candidate_home(candidate_name, candidate_version);
    if candidate_home.exists() {
        make_candidate_default(candidate_name, candidate_version);
    } else {
        eprintln!("{}@{} not installed, please install it first!", candidate_name, candidate_version);
    }
}


pub fn make_candidate_default(candidate_name: &str, candidate_version: &str) {
    let candidate_home = find_candidate_home(candidate_name, &candidate_version);
    if !candidate_home.exists() {
        eprintln!("{candidate_name}@{candidate_version} not installed, please use `sdk install {candidate_name} {candidate_version}` to install.", );
        return;
    }
    let candidate_current_link = candidate_home.parent().unwrap().join("current");
    // `is_symlink()` doesn't follow the link, so dangling links (target removed) are handled too;
    // `exists()` would return false for them and leave the stale link in place.
    if candidate_current_link.is_symlink() {
        let remove_result = symlink::remove_symlink_dir(&candidate_current_link)
            .or_else(|_| std::fs::remove_file(&candidate_current_link));
        if let Err(e) = remove_result {
            eprintln!("Failed to remove existing link {}: {}", candidate_current_link.display(), e);
            return;
        }
    } else if candidate_current_link.exists() {
        eprintln!("{} exists and is not a symlink, please remove it manually.", candidate_current_link.display());
        return;
    }
    if let Err(e) = symlink::symlink_dir(&candidate_home, &candidate_current_link) {
        eprintln!("Failed to create link {}: {}", candidate_current_link.display(), e);
    }
}

#[cfg(test)]
mod tests {
    use crate::sdkman_cli::clap_app::build_sdkman_app;
    use super::*;

    #[test]
    fn test_make_default() {
        let candidate_name = "ant";
        let candidate_version = "1.10.14";
        let sdkman_app = build_sdkman_app();
        let sdk_matches = sdkman_app.get_matches_from(&vec!["sdk", "default", candidate_name, candidate_version]);
        manage_default(sdk_matches.subcommand_matches("default").unwrap());
    }
}
