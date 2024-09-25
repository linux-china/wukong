use std::path::PathBuf;
use clap::{Arg, Command};
use crate::jbang_cli::{jbang_home, set_executable};

pub fn manage_app(app_matches: &clap::ArgMatches) {
    if let Some((sub_command, matches)) = app_matches.subcommand() {
        match sub_command {
            "install" => {
                let script_or_file = matches.get_one::<String>("scriptOrFile").unwrap();
                let command_name = if let Some(name) = matches.get_one::<String>("name") {
                    name
                } else if script_or_file.contains('.') {
                    &script_or_file[..script_or_file.find('.').unwrap()]
                } else {
                    script_or_file
                };
                install_app(command_name, script_or_file);
            }
            "uninstall" => {
                let name = matches.get_one::<String>("name").unwrap();
                let command_path = jbang_home().join("bin").join(name);
                if command_path.exists() {
                    std::fs::remove_file(&command_path).unwrap();
                } else {
                    eprintln!("Command not found: {}", command_path.to_str().unwrap());
                }
            }
            "list" => {
                list_apps();
            }
            "setup" => {
                if which::which("jbang").is_ok() {
                    println!("JBang environment is already set up.");
                } else {
                    let bin_path = jbang_home().join("bin");
                    let bin_path = bin_path.to_str().unwrap();
                    println!("Please add {} to PATH environment variable: export PATH=$PATH:{}", bin_path, bin_path);
                }
            }
            _ => println!("Unknown command"),
        }
    }
}

pub fn list_apps() {
    let bin_dir = jbang_home().join("bin");
    if bin_dir.exists() {
        for entry in std::fs::read_dir(bin_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let file_name = entry.file_name();
                let file_name = file_name.to_str().unwrap();
                if !file_name.starts_with(".") &&
                    file_name != "jbang" && !file_name.starts_with("jbang.") {
                    println!("{}", file_name);
                }
            }
        }
    }
}
pub fn install_app(command_name: &str, script_or_file: &str) {
    let file_path = PathBuf::from(script_or_file);
    let script_path = if file_path.exists() {
        let absolute_path = std::path::absolute(file_path).unwrap();
        absolute_path.to_str().unwrap().to_string()
    } else {
        script_or_file.to_string()
    };
    let command_path = jbang_home().join("bin").join(command_name);
    let code = format!("#!/bin/sh\nexec jbang run {} \"$@\"", script_path);
    std::fs::write(&command_path, code).unwrap();
    set_executable(&command_path);
}

pub fn build_app_command() -> Command {
    Command::new("app")
        .about("Manage scripts installed on the user's PATH as commands.")
        .subcommand(
            Command::new("install")
                .about("Install a script as a command.")
                .arg(
                    Arg::new("name")
                        .long("name")
                        .help("A name for the command")
                        .num_args(1)
                        .required(true)
                )
                .arg(
                    Arg::new("scriptOrFile")
                        .help("A reference to a source file")
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::new("userParams")
                        .help("Parameters to pass on to the script")
                        .index(2)
                        .num_args(1..)
                        .required(false)
                )
        )
        .subcommand(
            Command::new("uninstall")
                .about("Removes a previously installed command.")
                .arg(
                    Arg::new("name")
                        .help("The name of the command")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("list")
                .about("Lists installed commands.")
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
            Command::new("setup")
                .about("Make jbang commands available for the user.")
                .arg(
                    Arg::new("format")
                        .long("format")
                        .help("Specify output format ('text' or 'json')")
                        .num_args(1)
                        .required(false)
                        .value_parser(["text", "json"])
                )
                .arg(
                    Arg::new("catalogName")
                        .help("The name of a catalog.")
                        .index(1)
                        .required(false)
                )
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_app() {
        install_app("hello", "scripts/hello.java");
    }

    #[test]
    fn test_list_apps() {
        list_apps();
    }
}
