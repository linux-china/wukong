use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use clap::Command;
use crate::sdkman_cli::install::install_candidate;
use crate::sdkman_cli::{find_java_home, sdkman_home};

pub fn manage_direnv(direnv_matches: &clap::ArgMatches) {
    if direnv_matches.subcommand_matches("init").is_some() {
        direnv_init();
    } else {
        direnv_hook();
    }
}

pub fn direnv_init() {
    let envrc_path = PathBuf::from(".envrc");
    if !envrc_path.exists() {
        std::fs::write(".envrc", "eval $(sdk direnv)").unwrap();
    } else {
        let code = std::fs::read_to_string(envrc_path).unwrap();
        if !code.contains("eval $(sdk direnv)") {
            std::fs::write(".envrc", format!("{}\neval $(sdk direnv)", code)).unwrap();
        }
    }
    println!("direnv initialized");
}

/// Output of the hook is evaluated by shell(`eval $(sdk direnv)`), so only `export` statements are written to stdout,
/// and all other messages(installation progress etc.) must go to stderr.
pub fn direnv_hook() {
    let mut paths: Vec<String> = vec![];
    let mut java_version = None;
    let candidates_path = sdkman_home().join("candidates");
    let sdkman_rc = PathBuf::from(".sdkmanrc");
    if sdkman_rc.exists() {
        let pairs = java_properties::read(BufReader::new(File::open(&sdkman_rc).unwrap())).unwrap();
        for (candidate_name, candidate_version) in &pairs {
            let mut candidate_home = candidates_path.join(candidate_name).join(candidate_version);
            if candidate_name == "java" && candidate_version.parse::<u32>().is_ok() {
                if let Some(java_home) = find_java_home(candidate_version) {
                    candidate_home = java_home;
                }
            }
            if !candidate_home.exists() {
                candidate_home = install_candidate(candidate_name, candidate_version);
            }
            if candidate_home.exists() {
                export_candidate_home(candidate_name, &candidate_home, &mut paths);
                if candidate_name == "java" {
                    java_version = Some(candidate_version.clone());
                }
            }
        }
    }
    if java_version.is_none() {
        let java_version_file = PathBuf::from(".java-version");
        if java_version_file.exists() {
            let version = std::fs::read_to_string(java_version_file).unwrap().trim().to_string();
            let java_home = if version.parse::<u32>().is_ok() { // major version, such as 21
                find_java_home(&version)
            } else { // SDKMAN version, such as 21.0.4-tem
                Some(candidates_path.join("java").join(&version)).filter(|home| home.exists())
            };
            // install_candidate() resolves major version to SDKMAN version and downloads it from SDKMAN broker
            let java_home = java_home.unwrap_or_else(|| install_candidate("java", &version));
            if java_home.exists() {
                export_candidate_home("java", &java_home, &mut paths);
                java_version = Some(version);
            }
        }
    }
    if let Some(java_version) = java_version {
        println!("export JENV_VERSION={}", java_version);
    }
    if !paths.is_empty() {
        println!("export PATH={}:$PATH", paths.join(":"));
    }
}

fn export_candidate_home(candidate_name: &str, candidate_home: &PathBuf, paths: &mut Vec<String>) {
    let candidate_home_dir = candidate_home.to_str().unwrap();
    println!("export {}_HOME={}", candidate_name.to_uppercase(), candidate_home_dir);
    if candidate_name == "java" && candidate_home_dir.contains("graal") {
        println!("export GRAALVM_HOME={}", candidate_home_dir);
    }
    if candidate_home.join("bin").exists() {
        paths.push(candidate_home.join("bin").to_str().unwrap().to_string());
    } else {
        paths.push(candidate_home_dir.to_string());
    }
}

pub fn build_direnv_command() -> Command {
    Command::new("direnv")
        .about("Integration with direnv `.envrc`")
        .subcommand(
            Command::new("init")
                .about("Generate hook for direnv")
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direnv_hook() {
        direnv_hook();
    }
}

