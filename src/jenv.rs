use crate::jenv_cli::{add_command, commands_command, completion_command, global_command, local_command, remove_command, shell_command, version_command, versions_command, which_command};
use crate::jenv_cli::app::build_jenv_app;

mod jenv_cli;
mod foojay;
mod common;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    // jenv init scripts
    if args.len() >= 2 && args[1] == "init" {
        jenv_cli::init();
        return;
    }
    let app = build_jenv_app();
    let matches = app.get_matches();
    if let Some((command, command_matches)) = matches.subcommand() {
        match command {
            "commands" => commands_command(),
            "local" => local_command(command_matches),
            "global" => global_command(command_matches),
            "shell" => shell_command(),
            "version" => version_command(),
            "versions" => versions_command(),
            "which" => which_command(command_matches),
            "whence" => which_command(command_matches),
            "add" => add_command(command_matches),
            "remove" => remove_command(command_matches),
            "completion" => completion_command(command_matches),
            &_ => println!("Unknown command"),
        }
    }
}
