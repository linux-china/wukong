//! clap App for command cli

mod common;
mod foojay;
mod jbang_cli;

use clap::{Command, Arg, ArgAction};
use serde::Serialize;
use crate::jbang_cli::config::manage_config;
use crate::jbang_cli::jdk::manage_jdk;

pub const VERSION: &str = "0.1.0";

fn main() {
    let app = build_jbang_app();
    let matches = app.get_matches();
    if let Some((command, command_matches)) = matches.subcommand() {
        match command {
            "jdk" => manage_jdk(command_matches),
            "config" => manage_config(command_matches),
            &_ => println!("Unknown command"),
        }
    }
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
    let init_command = Command::new("init")
        .about("Builds and runs provided script.")
        .arg(
            Arg::new("template")
                .short('t')
                .long("template")
                .num_args(1)
                .help("Init script with a java class useful for scripting")
                .required(true)
        )
        .arg(
            Arg::new("scriptOrFile")
                .help(" file or URL to a Java code file")
                .index(1)
                .required(false)
        )
        .arg(
            Arg::new("params")
                .help("Parameters to pass on to the generation.")
                .required(false)
                .index(2)
                .num_args(1..)
        );
    let jdk_command = Command::new("jdk")
        .about("Manage Java Development Kits installed by jbang.")
        .subcommand(
            Command::new("default")
                .about("Sets the default JDK to be used by JBang.")
                .arg(
                    Arg::new("version")
                        .help("The version of the JDK to select")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("home")
                .about("Prints the folder where the given JDK is installed.")
                .arg(
                    Arg::new("version")
                        .help("The version of the JDK to select")
                        .index(1)
                        .required(false)
                )
        )
        .subcommand(
            Command::new("install")
                .about("Installs a JDK.")
                .arg(
                    Arg::new("version")
                        .help("The version or id to install")
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::new("existingJdkPath")
                        .help("Pre installed JDK path")
                        .index(2)
                        .required(false)
                )
        )
        .subcommand(
            Command::new("java-env")
                .about("Prints out the environment variables needed to use the given JDK.")
                .arg(
                    Arg::new("version")
                        .help("The version of the JDK to select")
                        .index(1)
                        .required(false)
                )
        )
        .subcommand(
            Command::new("list")
                .about("Lists installed JDKs.")
                .arg(
                    Arg::new("available")
                        .long("available")
                        .help("Shows versions available for installation")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("show-details")
                        .long("show-details")
                        .help("Shows detailed information for each JDK (only when format=text)")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("format")
                        .long("format")
                        .help("Specify output format ('text' or 'json')")
                        .num_args(1)
                        .required(false)
                        .value_parser(["text", "json"])
                )
        )
        .subcommand(
            Command::new("uninstall")
                .about("Uninstalls an existing JDK.")
                .arg(
                    Arg::new("version")
                        .help("The version to uninstall")
                        .index(1)
                        .required(true)
                )
        );
    let config_command = Command::new("config")
        .about("Read and write configuration options.")
        .subcommand(
            Command::new("get")
                .about("The name of the configuration option to get")
                .arg(
                    Arg::new("key")
                        .help("The key to get")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("set")
                .about("Set a configuration value")
                .arg(
                    Arg::new("key")
                        .help("The name of the configuration option to set")
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::new("value")
                        .help("The value to set for the configuration option")
                        .index(2)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("unset")
                .about("Remove a configuration value")
                .arg(
                    Arg::new("key")
                        .help("The key to remove")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("list")
                .about("List active configuration values")
                .arg(
                    Arg::new("format")
                        .long("format")
                        .help("Specify output format ('text' or 'json')")
                        .num_args(1)
                        .required(false)
                        .value_parser(["text", "json"])
                )
        );
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
        .subcommand(init_command)
        .subcommand(jdk_command)
        .subcommand(config_command)
}
