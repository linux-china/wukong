use crate::sdkman_cli::{find_candidate_home};

pub fn manage_use(use_matches: &clap::ArgMatches) {
    let candidate_name = use_matches.get_one::<String>("candidate").unwrap();
    let candidate_version = use_matches.get_one::<String>("version").unwrap();
    let candidate_home = find_candidate_home(candidate_name, candidate_version);
    if candidate_home.exists() {
        let candidate_home_name = format!("{}_HOME", candidate_name.to_uppercase());
        let home_path = candidate_home.to_str().unwrap();
        println!("export PATH=\"{}:$PATH\"", home_path);
        println!("export {}=\"{}\"", candidate_home_name, home_path);
        println!("# Run this command to configure your shell:");
        println!("# eval $(sdk use {} {})", candidate_name, candidate_version);
    } else {
        eprintln!("{}@{} not installed, please install it first!", candidate_name, candidate_version);
    }
}
