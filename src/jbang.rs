//! clap App for JBang command cli

mod common;
mod foojay;
mod jbang_cli;

use std::path::PathBuf;
use clap::{Command, Arg, ArgAction, ArgMatches};
use crate::jbang_cli::config::{build_config_command, manage_config};
use crate::jbang_cli::init::{build_init_command, manage_init};
use crate::jbang_cli::{jbang_home, JBANG_DEFAULT_JAVA_VERSION};
use crate::jbang_cli::jdk::{build_jdk_command, manage_jdk};
use crate::jbang_cli::template::{build_template_command, manage_template};
use crate::jbang_cli::trust::{build_trust_command, manage_trust};
use crate::jbang_cli::upgrade::{install_jbang, upgrade_jbang};
use itertools::Itertools;
use crate::foojay::install_jdk;
use crate::jbang_cli::run::{manage_run, jbang_run};

pub const VERSION: &str = "0.1.0";
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
        if !script_path.starts_with("-") {
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

pub fn build_jbang_app() -> Command {
    let run_command = Command::new("run")
        .about("Builds and runs provided script.")
        .arg(
            Arg::new("main")
                .short('m')
                .long("main")
                .help("Main class to use when running. Used primarily for running jar's.")
                .required(true)
        )
        .arg(
            Arg::new("scriptOrFile")
                .help("A reference to a source file")
                .index(1)
                .required(false)
        )
        .arg(
            Arg::new("userParams")
                .help("Parameters to pass on to the script.")
                .required(false)
                .index(2)
                .num_args(1..)
        );
    let build_command = Command::new("build")
        .about("Compiles and stores script in the cache.")
        .arg(
            Arg::new("build-dir")
                .long("build-dir")
                .num_args(1)
                .help("Use given directory for build results")
                .required(false)
        )
        .arg(
            Arg::new("scriptOrFile")
                .help("A reference to a source file")
                .index(1)
                .required(false)
        );
    let version_command = Command::new("version")
        .about("Display version info.");
    let jdk_command = build_jdk_command();
    let config_command = build_config_command();
    let trust_command = build_trust_command();
    let init_command = build_init_command();
    let template_command = build_template_command();
    let upgrade_command = Command::new("upgrade")
        .about("Upgrade jbang to the latest version.");
    Command::new("jbang")
        .version(VERSION)
        .about("jbang - Unleash the power of Java")
        .arg(
            Arg::new("config")
                .long("config")
                .num_args(1)
                .help("Path to config file to be used instead of the default")
                .required(false),
        )
        .arg(
            Arg::new("fresh")
                .long("fresh")
                .action(ArgAction::SetTrue)
                .help("Make sure we use fresh (i.e. non-cached) resources.")
                .required(false),
        )
        .arg(
            Arg::new("insecure")
                .long("insecure")
                .action(ArgAction::SetTrue)
                .help("Enable insecure trust of all SSL certificates.")
                .required(false),
        )
        .arg(
            Arg::new("offline")
                .short('o')
                .long("offline")
                .action(ArgAction::SetTrue)
                .help("Work offline. Fail-fast if dependencies are missing.")
                .required(false),
        )
        .arg(
            Arg::new("preview")
                .long("preview")
                .action(ArgAction::SetTrue)
                .help("Enable jbang preview features.")
                .required(false),
        )
        .arg(
            Arg::new("quiet")
                .long("quiet")
                .action(ArgAction::SetTrue)
                .help("jbang will be quiet, only print when error occurs.")
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("jbang will be verbose on what it does.")
                .required(false),
        )
        .arg(
            Arg::new("scriptOrFile")
                .help("A reference to a source file")
                .index(1)
                .required(false)
        )
        .arg(
            Arg::new("userParams")
                .help("Parameters to pass on to the script")
                .index(2)
                .num_args(1..)
                .action(ArgAction::Append)
                .required(false)
        )
        .subcommand(run_command)
        .subcommand(build_command)
        .subcommand(jdk_command)
        .subcommand(config_command)
        .subcommand(trust_command)
        .subcommand(init_command)
        .subcommand(template_command)
        .subcommand(upgrade_command)
        .subcommand(version_command)
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
