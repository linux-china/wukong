//! clap App for JBang command cli

mod common;
mod foojay;
mod jbang_cli;

use std::path::PathBuf;
use clap::{Command, Arg, ArgAction};
use crate::jbang_cli::config::{build_config_command, manage_config};
use crate::jbang_cli::init::{build_init_command, manage_init};
use crate::jbang_cli::jbang_home;
use crate::jbang_cli::jdk::{build_jdk_command, manage_jdk};
use crate::jbang_cli::template::{build_template_command, manage_template};
use crate::jbang_cli::trust::{build_trust_command, manage_trust};
use crate::jbang_cli::upgrade::{install_jbang, upgrade_jbang};

pub const VERSION: &str = "0.1.0";

fn main() {
    let jbang_home = jbang_home();
    if !jbang_home.exists() {
        install_jbang();
    }
    let app = build_jbang_app();
    let matches = app.get_matches();
    if let Some((command, command_matches)) = matches.subcommand() {
        match command {
            "jdk" => manage_jdk(command_matches),
            "config" => manage_config(command_matches),
            "trust" => manage_trust(command_matches),
            "init" => manage_init(command_matches),
            "template" => manage_template(command_matches),
            "upgrade" => upgrade_jbang(),
            "version" => display_version(&jbang_home),
            &_ => println!("Unknown command"),
        }
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
