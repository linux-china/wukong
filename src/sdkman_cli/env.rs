use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;
use clap::{Command};
use crate::sdkman_cli::install::install_candidate;
use crate::sdkman_cli::{get_installed_candidate_default_version, sdkman_home};

pub fn manage_env(env_matches: &clap::ArgMatches) {
    if let Some((command, _)) = env_matches.subcommand() {
        match command {
            "init" => env_init(),
            "install" => env_install(),
            "clear" => env_clear(),
            &_ => println!("Unknown command"),
        }
    }
}

pub fn env_init() {
    let sdkmanrc_path = PathBuf::from(".sdkmanrc");
    if !sdkmanrc_path.exists() {
        let java_default_version = get_installed_candidate_default_version("java");
        if !java_default_version.is_empty() {
            write_candidates(vec![format!("java={}", java_default_version)]);
        }
    } else {
        eprintln!(".sdkmanrc already exists!");
    }
}

pub fn env_install() {
    let sdkmanrc_path = PathBuf::from(".sdkmanrc");
    if sdkmanrc_path.exists() {
        let text = std::fs::read_to_string(sdkmanrc_path).unwrap();
        for line in text.lines() {
            if !line.is_empty() && !line.starts_with("#") {
                let parts: Vec<&str> = line.trim().split("=").collect();
                if parts.len() == 2 {
                    install_candidate(parts[0].trim(), parts[1].trim());
                }
            }
        }
    } else {
        eprintln!(".sdkmanrc not exists!");
    }
}

pub fn env_clear() {
    let sdkmanrc_path = PathBuf::from(".sdkmanrc");
    if sdkmanrc_path.exists() {
        let f2 = File::open(&sdkmanrc_path).unwrap();
        let map = java_properties::read(BufReader::new(f2)).unwrap();
        let mut candidates: Vec<String> = vec![];
        for (key, value) in map {
            candidates.push(format!("{}={}", key, value));
        }
        if !candidates.is_empty() {
            for candidate in &candidates {
                println!("Restored {} (default)", candidate);
            }
            write_candidates(candidates);
        }
    } else {
        eprintln!(".sdkmanrc not exists!");
    }
}

fn write_candidates(candidates: Vec<String>) {
    let mut lines = vec!["# Enable auto-env through the sdkman_auto_env config".to_owned(),
                         "# Add key=value pairs of SDKs to use below".to_owned()];
    lines.extend(candidates);
    let sdkmanrc_path = PathBuf::from(".sdkmanrc");
    let mut file = File::create(&sdkmanrc_path).unwrap();
    file.write_all(lines.join("\n").as_bytes()).unwrap();
}

pub fn build_env_command() -> Command {
    Command::new("env")
        .about("control SDKs on a project level, setting up specific versions for a directory.")
        .subcommand(Command::new("install").about("install and switch to the SDK versions specified in .sdkmanrc"))
        .subcommand(Command::new("init").about("allows for the creation of a default .sdkmanrc file with a single entry for the java candidate, set to the current default value)"))
        .subcommand(Command::new("clear").about("reset all SDK versions to their system defaults"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_candidates() {
        let candidates = vec!["java=17.0.4-tem".to_owned()];
        write_candidates(candidates);
    }
}
