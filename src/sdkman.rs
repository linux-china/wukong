mod common;
mod sdkman_cli;

use clap::{Command, Arg};
use crate::sdkman_cli::app::build_sdkman_app;
use crate::sdkman_cli::default::manage_default;
use crate::sdkman_cli::direnv::manage_direnv;
use crate::sdkman_cli::env::{build_env_command, manage_env};
use crate::sdkman_cli::home::manage_home;
use crate::sdkman_cli::install::manage_install;
use crate::sdkman_cli::list::manage_list;
use crate::sdkman_cli::uninstall::manage_uninstall;
use crate::sdkman_cli::upgrade::manage_upgrade;
use crate::sdkman_cli::use_candidate::manage_use;

fn main() {
    let app = build_sdkman_app();
    let matches = app.get_matches();
    if let Some((command, command_matches)) = matches.subcommand() {
        match command {
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

