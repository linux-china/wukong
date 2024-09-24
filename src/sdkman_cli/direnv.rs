use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use clap::Command;
use wukong::common::jbang_home;
use crate::sdkman_cli::install::install_candidate;
use crate::sdkman_cli::sdkman_home;

pub fn manage_direnv(direnv_matches: &clap::ArgMatches) {
    if direnv_matches.subcommand_matches("init").is_some() {
        direnv_init();
    } else {
        direnv_hook();
    }
}

pub fn direnv_init() {
    let sdkmanrc_path = PathBuf::from(".sdkmanrc");
    if !sdkmanrc_path.exists() {
        std::fs::write(".envrc", "eval $(sdk direnv)").unwrap();
    } else {
        let code = std::fs::read_to_string(sdkmanrc_path).unwrap();
        if !code.contains("eval $(sdk direnv)") {
            std::fs::write(".envrc", format!("{}\neval $(sdk direnv)", code)).unwrap();
        }
    }
}

pub fn direnv_hook() {
    let mut paths: Vec<String> = vec![];
    let mut java_version = None;
    let candidates_path = sdkman_home().join("candidates");
    let sdkman_rc = PathBuf::from(".sdkmanrc");
    if sdkman_rc.exists() {
        let pairs = java_properties::read(BufReader::new(File::open(&sdkman_rc).unwrap())).unwrap();
        for (key, value) in &pairs {
            let candidate_home = candidates_path.join(key).join(value);
            if candidate_home.exists() {
                let candidate_home_dir = candidate_home.to_str().unwrap();
                println!("export {}_HOME={}", key.to_uppercase(), candidate_home_dir);
                if candidate_home.join("bin").exists() {
                    paths.push(candidate_home.join("bin").to_str().unwrap().to_string());
                } else {
                    paths.push(candidate_home_dir.to_string());
                }
                if key == "java" {
                    java_version = Some(value.clone());
                }
            } else {
                install_candidate(key, value);
            }
        }
    }
    if java_version.is_none() {
        let java_version_file = PathBuf::from(".java-version");
        if java_version_file.exists() {
            let java_version = std::fs::read_to_string(java_version_file).unwrap().trim().to_string();
            if java_version.parse::<u32>().is_ok() { // load java home from JBang
                let java_home = jbang_home().join("cache").join("jdks").join(&java_version);
                if java_home.exists() {
                    let java_home_dir = java_home.to_str().unwrap();
                    println!("export JAVA_HOME={}", java_home_dir);
                    paths.push(java_home.join("bin").to_str().unwrap().to_string());
                } else {
                    wukong::foojay::install_jdk(&java_version, &java_home);
                }
            } else { // load java home from SDKMAN
                let java_home = candidates_path.join("java").join(&java_version);
                if java_home.exists() {
                    let java_home_dir = java_home.to_str().unwrap();
                    println!("export JAVA_HOME={}", java_home_dir);
                    paths.push(java_home.join("bin").to_str().unwrap().to_string());
                } else {
                    install_candidate("java", &java_version);
                }
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

