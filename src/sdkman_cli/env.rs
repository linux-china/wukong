use std::fs::File;
use std::io::Write;
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
        let mut file = File::create(&sdkmanrc_path).unwrap();
        let mut lines = vec!["# Enable auto-env through the sdkman_auto_env config".to_owned(),
                             "# Add key=value pairs of SDKs to use below".to_owned()];
        let java_default_version = get_installed_candidate_default_version("java");
        if !java_default_version.is_empty() {
            lines.push(format!("java={}", java_default_version));
        }
        file.write_all(lines.join("\n").as_bytes()).unwrap();
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
        let text = std::fs::read_to_string(&sdkmanrc_path).unwrap();
        let mut candidates: Vec<String> = vec![];
        for line in text.lines() {
            if !line.is_empty() && !line.starts_with("#") {
                let parts: Vec<&str> = line.trim().split("=").collect();
                let candidate_name = parts[0].trim();
                let default_version = get_installed_candidate_default_version(candidate_name);
                if !default_version.is_empty() {
                    candidates.push(format!("{}={}", candidate_name, default_version));
                } else {
                    candidates.push(line.to_string());
                }
            }
        }
        if !candidates.is_empty() {
            let mut file = File::create(&sdkmanrc_path).unwrap();
            let mut lines = vec!["# Enable auto-env through the sdkman_auto_env config".to_owned(),
                                 "# Add key=value pairs of SDKs to use below".to_owned()];
            lines.extend(&candidates);
            file.write_all(lines.join("\n").as_bytes()).unwrap();
            for candidate in &candidates {
                println!("Restored {} (default)", candidate);
            }
        }
    } else {
        eprintln!(".sdkmanrc not exists!");
    }
}

pub fn build_env_command() -> Command {
    Command::new("env")
        .about("control SDKs on a project level, setting up specific versions for a directory.")
        .subcommand(Command::new("install").about("install and switch to the SDK versions specified in .sdkmanrc"))
        .subcommand(Command::new("init").about("allows for the creation of a default .sdkmanrc file with a single entry for the java candidate, set to the current default value)"))
        .subcommand(Command::new("clear").about("reset all SDK versions to their system defaults"))
}
