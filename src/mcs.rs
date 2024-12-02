use crate::mcs_cli::clap_app::build_mcs_app;
use crate::mcs_cli::{class_search, search};

mod mcs_cli;

fn main() {
    let app = build_mcs_app();
    let matches = app.get_matches();
    if let Some((command, command_matches)) = matches.subcommand() {
        match command {
            "search" => search(command_matches),
            "class-search" => class_search(command_matches),
            &_ => println!("Unknown command"),
        }
    }
}
