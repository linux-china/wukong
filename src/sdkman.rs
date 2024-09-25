mod sdkman_cli;

use std::fs::File;
use std::io::BufReader;
use wukong::common::sdkman_home;
use crate::sdkman_cli::app::build_sdkman_app;
use crate::sdkman_cli::default::manage_default;
use crate::sdkman_cli::direnv::manage_direnv;
use crate::sdkman_cli::env::{manage_env};
use crate::sdkman_cli::home::manage_home;
use crate::sdkman_cli::init::shell_hook;
use crate::sdkman_cli::install::manage_install;
use crate::sdkman_cli::list::manage_list;
use crate::sdkman_cli::read_sdkman_config;
use crate::sdkman_cli::uninstall::manage_uninstall;
use crate::sdkman_cli::upgrade::manage_upgrade;
use crate::sdkman_cli::use_candidate::manage_use;

fn main() {
    let app = build_sdkman_app();
    let matches = app.get_matches();
    if let Some((command, command_matches)) = matches.subcommand() {
        load_config();
        match command {
            "init" => shell_hook(),
            "list" => manage_list(command_matches),
            "install" => manage_install(command_matches),
            "uninstall" => manage_uninstall(command_matches),
            "default" => manage_default(command_matches),
            "use" => manage_use(command_matches),
            "home" => manage_home(command_matches),
            "env" => manage_env(command_matches),
            "direnv" => manage_direnv(command_matches),
            "upgrade" => manage_upgrade(command_matches),
            &_ => println!("Unknown command"),
        }
    }
}

fn load_config() {
    let config = read_sdkman_config();
    if let Some(insecure_ssl) = config.get("sdkman_insecure_ssl") {
        if insecure_ssl == "true" {
            std::env::set_var("ONEIO_ACCEPT_INVALID_CERTS", "true");
        }
    }
    if let Some(colour_enable) = config.get("sdkman_colour_enable") {
        if colour_enable == "false" {
            std::env::set_var("CLICOLOR", "0");
        }
    }
}


