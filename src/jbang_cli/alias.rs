use clap::{Arg, Command};

pub fn manage_alias(alias_matches: &clap::ArgMatches) {
    if let Some((sub_command, matches)) = alias_matches.subcommand() {
        match sub_command {
            "add" => {

            }
            "remove" => {

            }
            "list" => {

            }
            _ => {}
        }
    }
}

pub fn build_alias_command() -> Command {
    Command::new("alias")
        .about("Manage aliases for scripts.")
        .subcommand(
            Command::new("add")
                .about("Add alias for script reference.")
                .arg(
                    Arg::new("name")
                        .long("name")
                        .help("A name for the alias")
                        .num_args(1)
                        .required(true)
                )
                .arg(
                    Arg::new("description")
                        .long("description")
                        .help("A description for the alias")
                        .num_args(1)
                        .required(false)
                )
                .arg(
                    Arg::new("file")
                        .help("Path or URL to alias file")
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::new("scriptOrFile")
                        .help("A reference to a source file")
                        .num_args(1)
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::new("params")
                        .help("Parameters to pass on to the script")
                        .num_args(1..)
                        .index(2)
                        .required(false)
                )
        )
        .subcommand(
            Command::new("remove")
                .about("Remove existing alias.")
                .arg(
                    Arg::new("file")
                        .help("Path or URL to alias file")
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::new("name")
                        .help("The name of the alias")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("list")
                .about("Lists locally defined aliases or from the given catalog.")
                .arg(
                    Arg::new("format")
                        .long("format")
                        .help("Specify output format ('text' or 'json')")
                        .num_args(1)
                        .required(false)
                        .value_parser(["text", "json"])
                )
                .arg(
                    Arg::new("file")
                        .short('f')
                        .help("Path to the catalog file to use")
                        .num_args(1)
                        .required(false)
                )
                .arg(
                    Arg::new("catalogName")
                        .help("The name of a catalog.")
                        .index(1)
                        .required(false)
                )
        )
}
