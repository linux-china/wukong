use crate::sdkman_cli::{find_candidate_home};
use crate::sdkman_cli::install::install_candidate;

pub fn manage_use(use_matches: &clap::ArgMatches) {
    let candidate_name = use_matches.get_one::<String>("candidate").unwrap();
    let candidate_version = use_matches.get_one::<String>("version").unwrap();
    let candidate_home = find_candidate_home(candidate_name, candidate_version);
    if !candidate_home.exists() {
        install_candidate(candidate_name, candidate_version);
    }
    if candidate_home.exists() {
        let candidate_home_name = format!("{}_HOME", candidate_name.to_uppercase());
        let home_path = candidate_home.to_str().unwrap();
        if candidate_home.join("bin").exists() {
            println!("export PATH=\"{}:$PATH\"", home_path);
        } else {
            println!("export PATH=\"{}/bin:$PATH\"", home_path);
        }
        println!("export {}=\"{}\"", candidate_home_name, home_path);
        println!("# Run this command to configure your shell:");
        println!("# eval $(sdk use {} {})", candidate_name, candidate_version);
    } else {
        eprintln!("{}@{} not found, please check name and version correct or not!", candidate_name, candidate_version);
    }
}
