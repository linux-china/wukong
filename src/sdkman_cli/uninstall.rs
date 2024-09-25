use crate::sdkman_cli::find_candidate_home;

pub fn manage_uninstall(uninstall_matches: &clap::ArgMatches) {
    let candidate_name = uninstall_matches.get_one::<String>("candidate").unwrap();
    let candidate_version = uninstall_matches.get_one::<String>("version").unwrap();
    let candidate_home = find_candidate_home(candidate_name, candidate_version);
    if candidate_home.exists() {
        let candidate_current_link = candidate_home.parent().unwrap().join("current");
        if candidate_current_link.exists()  && candidate_current_link.is_symlink() {
            let link_target_path = candidate_current_link.read_link().unwrap();
            if candidate_home == link_target_path {
                symlink::remove_symlink_dir(&candidate_current_link).unwrap();
            }
        }
        std::fs::remove_dir_all(&candidate_home).unwrap();
        println!("{}@{} removed from {}", candidate_name, candidate_version, candidate_home.to_str().unwrap());
    } else {
        eprintln!("{}@{} not installed, please install it first!", candidate_name, candidate_version);
    }
}

#[cfg(test)]
mod tests {
    use crate::sdkman_cli::clap_app::build_sdkman_app;
    use super::*;

    #[test]
    fn test_uninstall() {
        let candidate_name = "ant";
        let candidate_version = "1.10.13";
        let sdkman_app = build_sdkman_app();
        let sdk_matches = sdkman_app.get_matches_from(&vec!["sdk", "uninstall", candidate_name, candidate_version]);
        manage_uninstall(sdk_matches.subcommand_matches("uninstall").unwrap());
    }
}
