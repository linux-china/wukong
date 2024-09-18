use crate::sdkman_cli::{find_candidate_home};

pub fn manage_home(default_matches: &clap::ArgMatches) {
    let candidate_name = default_matches.get_one::<String>("candidate").unwrap();
    let candidate_version = default_matches.get_one::<String>("version").unwrap();
    let candidate_home = find_candidate_home(candidate_name, candidate_version);
    if candidate_home.exists() {
        println!("{}", candidate_home.to_str().unwrap());
    } else {
        eprintln!("{} {} is not installed on your system.", candidate_name, candidate_version);
    }
}

