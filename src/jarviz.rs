use crate::jarviz_cli::bytecode::bytecode;
use crate::jarviz_cli::clap_app::build_jarviz_app;
use crate::jarviz_cli::entries::entries;
use crate::jarviz_cli::services::services;
use crate::jarviz_cli::{checksum, jar_manifest, jar_module, packages};

mod jarviz_cli;

fn main() {
    let app = build_jarviz_app();
    let matches = app.get_matches();
    if let Some((command, command_matches)) = matches.subcommand() {
        match command {
            "bytecode" => bytecode(command_matches),
            "checksum" => checksum(command_matches),
            "entries" => entries(command_matches),
            "manifest" => jar_manifest(command_matches),
            "module" => jar_module(command_matches),
            "packages" => packages(command_matches),
            "services" => services(command_matches),
            &_ => println!("Unknown command"),
        }
    }
}
