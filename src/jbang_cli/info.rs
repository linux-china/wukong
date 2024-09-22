use clap::{Arg, Command};
use crate::jbang_cli::call_jbang_sub_command;

pub fn manage_info(info_matches: &clap::ArgMatches) {
    if let Some((sub_command, matches)) = info_matches.subcommand() {
        match sub_command {
            "tools" => {
                let script_or_file = matches.get_one::<String>("scriptOrFile").unwrap();
                call_jbang_sub_command(&["info", "tools", script_or_file]);
            }
            "classpath" => {
                let script_or_file = matches.get_one::<String>("scriptOrFile").unwrap();
                call_jbang_sub_command(&["info", "classpath", script_or_file]);
            }
            "jar" => {
                let script_or_file = matches.get_one::<String>("scriptOrFile").unwrap();
                call_jbang_sub_command(&["info", "jar", script_or_file]);
            }
            _ => {}
        }
    }
}


pub fn build_info_command() -> Command {
    Command::new("info")
        .about("Provides info about the script for tools (and humans who are tools).")
        .subcommand(
            Command::new("tools")
                .about("Prints a json description usable for tools/IDE's to get classpath and more info for a jbang script/application. Exact format is still quite experimental.")
                .arg(
                    Arg::new("scriptOrFile")
                        .help("A reference to a source file")
                        .num_args(1)
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("classpath")
                .about("Prints class-path used for this application using operating system specific path separation.")
                .arg(
                    Arg::new("scriptOrFile")
                        .help("A reference to a source file")
                        .num_args(1)
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("jar")
                .about("Prints the path to this application's JAR file.")
                .arg(
                    Arg::new("scriptOrFile")
                        .help("A reference to a source file")
                        .num_args(1)
                        .index(1)
                        .required(true)
                )
        )
}
