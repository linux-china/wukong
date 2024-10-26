use std::path::Path;
use crate::sdkman_cli::{find_candidate_home, find_java_home, find_java_version};
use crate::sdkman_cli::install::install_candidate;

pub fn manage_use(use_matches: &clap::ArgMatches) {
    let candidate_name = use_matches.get_one::<String>("candidate").unwrap();
    let mut candidate_version = use_matches.get_one::<String>("version").unwrap().to_string();
    // find java home by major version
    if candidate_name == "java" && candidate_version.parse::<u32>().is_ok() {
        if let Some(java_home) = find_java_home(&candidate_version) {
            use_candidate(candidate_name, &java_home);
            return;
        } else {
            let java_version = find_java_version(&candidate_version).unwrap();
            candidate_version = java_version;
        }
    }
    let candidate_home = find_candidate_home(candidate_name, &candidate_version);
    if !candidate_home.exists() {
        install_candidate(candidate_name, &candidate_version);
    }
    if candidate_home.exists() {
        use_candidate(candidate_name, &candidate_home);
    } else {
        eprintln!("{}@{} not found, please check name and version correct or not!", candidate_name, candidate_version);
    }
}

fn use_candidate(candidate_name: &str, candidate_home_path: &Path) {
    let candidate_home_name = format!("{}_HOME", candidate_name.to_uppercase());
    let candidate_home = candidate_home_path.to_str().unwrap();
    println!("export {}=\"{}\"", candidate_home_name, candidate_home);
    if candidate_name == "java" && candidate_home.contains("graal") {
        println!("export GRAALVM_HOME=\"{}\"", candidate_home);
    }
    if candidate_home_path.join("bin").exists() {
        println!("export PATH=\"{}:$PATH\"", candidate_home);
    } else {
        println!("export PATH=\"{}/bin:$PATH\"", candidate_home);
    }
    println!("# Run this command to configure your shell:");
    println!("# eval $(sdk use {} {})", candidate_name, candidate_home);
}
