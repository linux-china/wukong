use crate::sdkman_cli::{list_candidate_names, sdkman_home};

pub fn manage_current(current_matches: &clap::ArgMatches) {
    if let Some(candidate_name) = current_matches.get_one::<String>("candidate") {
        let candidate_current_link = sdkman_home().join("candidates").join(candidate_name).join("current");
        if candidate_current_link.exists() {
            let link_target_path = candidate_current_link.read_link().unwrap();
            let candidate_version = link_target_path.file_name().unwrap().to_str().unwrap();
            println!("Using {} version {}", candidate_name, candidate_version);
        } else {
            eprintln!("Not using any version of {}", candidate_name);
        }
    } else {
        list_candidates_current();
    }
}

fn list_candidates_current() {
    let candidates_dir = sdkman_home().join("candidates");
    let candidate_names = list_candidate_names();
    if candidate_names.is_empty() {
        eprintln!("No candidate installed yet.");
        return;
    }
    println!("Using:");
    for candidate_name in &candidate_names {
        let candidate_current_link = candidates_dir.join(candidate_name).join("current");
        if candidate_current_link.exists() {
            let link_target_path = candidate_current_link.read_link().unwrap();
            let candidate_version = link_target_path.file_name().unwrap().to_str().unwrap();
            println!("{}: {}", candidate_name, candidate_version);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sdkman_cli::clap_app::build_sdkman_app;
    use super::*;

    #[test]
    fn test_current() {
        let candidate_name = "java";
        let sdkman_app = build_sdkman_app();
        let sdk_matches = sdkman_app.get_matches_from(&vec!["sdk", "current", candidate_name]);
        manage_current(sdk_matches.subcommand_matches("current").unwrap());
    }
}

