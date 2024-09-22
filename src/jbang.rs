//! clap App for JBang command cli

mod common;
mod foojay;
mod jbang_cli;

use std::path::PathBuf;
use clap::{ArgMatches};
use crate::jbang_cli::config::{manage_config};
use crate::jbang_cli::init::{manage_init};
use crate::jbang_cli::{jbang_home, print_command_help, JBANG_DEFAULT_JAVA_VERSION};
use crate::jbang_cli::jdk::{manage_jdk};
use crate::jbang_cli::template::{manage_template};
use crate::jbang_cli::trust::{manage_trust};
use crate::jbang_cli::upgrade::{install_jbang, upgrade_jbang};
use itertools::Itertools;
use crate::foojay::install_jdk;
use crate::jbang_cli::alias::{manage_alias};
use crate::jbang_cli::app::{build_jbang_app, VERSION};
use crate::jbang_cli::cache::{manage_cache};
use crate::jbang_cli::catalog::{manage_catalog};
use crate::jbang_cli::export::{manage_export};
use crate::jbang_cli::info::{manage_info};
use crate::jbang_cli::run::{manage_run, jbang_run};

pub const JBANG_SUB_COMMANDS: [&str; 17] = ["run", "build", "init", "edit", "cache", "export",
    "jdk", "config", "trust", "alias", "template", "catalog", "app", "completion", "info", "version", "wrapper"];

fn main() {
    let jbang_home = jbang_home();
    if !jbang_home.exists() {
        install_jbang();
        // install default JDK
        let default_jdk_home = jbang_home.join("cache").join("jdks").join(JBANG_DEFAULT_JAVA_VERSION);
        if !default_jdk_home.exists() {
            install_jdk(JBANG_DEFAULT_JAVA_VERSION, &default_jdk_home);
        }
    }
    let args = std::env::args().collect::<Vec<String>>();
    // check run script from jbang
    if args.len() >= 3 && args[1] == "run" { // jbang run script_file
        let script_path = &args[2];
        if script_path == "-h" || script_path == "--help" {
            print_command_help("run");
            return;
        } else if !script_path.starts_with("-") {
            jbang_run(&args[2], &args[3..].iter().map(|s| s.as_str()).collect_vec());
            return;
        }
    } else if args.len() >= 2 { // jbang script file
        let script_path = &args[1];
        if !script_path.starts_with("-") && !JBANG_SUB_COMMANDS.contains(&script_path.as_str()) {
            jbang_run(&args[1], &args[2..].iter().map(|s| s.as_str()).collect_vec());
            return;
        }
    }
    let app = build_jbang_app();
    let matches = app.get_matches();
    // inject insecure
    inject_insecure(&matches);
    if let Some((command, command_matches)) = matches.subcommand() {
        match command {
            "jdk" => manage_jdk(command_matches),
            "config" => manage_config(command_matches),
            "trust" => manage_trust(command_matches),
            "init" => manage_init(command_matches),
            "template" => manage_template(command_matches),
            "run" => manage_run(command_matches),
            "catalog" => manage_catalog(command_matches),
            "alias" => manage_alias(command_matches),
            "info" => manage_info(command_matches),
            "export" => manage_export(command_matches),
            "cache" => manage_cache(command_matches),
            "upgrade" => upgrade_jbang(),
            "version" => display_version(&jbang_home),
            &_ => println!("Unknown command"),
        }
    } else if let Some(script_or_file) = matches.get_one::<String>("scriptOrFile") {
        let params: Vec<&String> = if let Some(user_params) = matches.get_many::<String>("userParams") {
            user_params.collect()
        } else {
            vec![]
        };
        jbang_run(script_or_file, &params.iter().map(|s| s.as_str()).collect_vec());
    }
}

fn inject_insecure(matches: &ArgMatches) {
    if matches.get_flag("insecure") {
        std::env::set_var("ONEIO_ACCEPT_INVALID_CERTS", "true")
    }
}

fn display_version(jbang_home: &PathBuf) {
    let jbang_version = std::fs::read_to_string(jbang_home.join("version.txt")).unwrap_or("unknown".to_string());
    println!("JBang: {}", jbang_version.trim());
    println!("JBang-rs: {}", VERSION);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help() {
        let app = build_jbang_app();
        app.get_matches_from(vec!["jbang", "--help"]);
    }
}
