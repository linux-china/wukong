use crate::sdkman_cli::{find_candidate_home, get_remote_candidate_default_version, list_candidate_names, read_sdkman_config};
use crate::sdkman_cli::default::make_candidate_default;
use crate::sdkman_cli::install::install_candidate;

pub fn manage_upgrade(upgrade_matches: &clap::ArgMatches) {
    let mut accept_as_default = upgrade_matches.get_flag("yes");
    if !accept_as_default {
        accept_as_default = read_sdkman_config().contains_key("sdkman_auto_answer");
    }
    if let Some(candidate_name) = upgrade_matches.get_one::<String>("candidate") {
        upgrade_candidate(candidate_name, accept_as_default);
    } else {
        let candidate_names = list_candidate_names();
        for candidate_name in candidate_names {
            upgrade_candidate(&candidate_name, accept_as_default);
        }
    }
}

fn upgrade_candidate(candidate_name: &str, accept_as_default: bool) {
    let default_remote_version = get_remote_candidate_default_version(candidate_name);
    if default_remote_version == "" {
        eprintln!("Failed to find default version for : {}", candidate_name);
        return;
    }
    let candidate_home = find_candidate_home(candidate_name, &default_remote_version);
    if !candidate_home.exists() {
        println!("Begin to upgrade {} to {}", candidate_name, default_remote_version);
        install_candidate(candidate_name, &default_remote_version);
        if accept_as_default {
            make_candidate_default(candidate_name, &default_remote_version);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upgrade_candidate() {
        let candidate_name = "ant";
        upgrade_candidate(candidate_name, true);
    }
}
