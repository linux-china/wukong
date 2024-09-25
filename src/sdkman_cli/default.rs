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
    let candidate_current_link = candidate_home.parent().unwrap().join("current");
    if candidate_current_link.exists() && candidate_current_link.is_symlink() {
        symlink::remove_symlink_dir(&candidate_current_link).unwrap();
    }
    symlink::symlink_dir(&candidate_home, &candidate_current_link).unwrap();
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
