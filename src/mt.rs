use crate::mt_cli::{add_command, jdks_command, list_command, remove_command, vendors_command};
use crate::mt_cli::app::build_mt_app;

mod common;
mod mt_cli;
mod sdkman_cli;


fn main() {
    let app = build_mt_app();
    let matches = app.get_matches();
    if let Some((command, command_matches)) = matches.subcommand() {
        match command {
            "jdks" => jdks_command(),
            "list" => list_command(),
            "add" => add_command(command_matches),
            "remove" => remove_command(command_matches),
            "vendors" => vendors_command(),
            &_ => println!("Unknown command"),
        }
    }
}

