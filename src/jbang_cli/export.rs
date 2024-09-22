use clap::{Arg, Command};
use crate::jbang_cli::call_jbang_sub_command;

pub fn manage_export(export_matches: &clap::ArgMatches) {
    if let Some((sub_command, matches)) = export_matches.subcommand() {
        match sub_command {
            "portable" => {
                let script_or_file = matches.get_one::<String>("scriptOrFile").unwrap();
                call_jbang_sub_command(&["export", "portable", script_or_file]);
            }
            "local" => {
                let script_or_file = matches.get_one::<String>("scriptOrFile").unwrap();
                call_jbang_sub_command(&["export", "local", script_or_file]);
            }
            "mavenrepo" => {
                let script_or_file = matches.get_one::<String>("scriptOrFile").unwrap();
                call_jbang_sub_command(&["export", "mavenrepo", script_or_file]);
            }
            "native" => {
                let script_or_file = matches.get_one::<String>("scriptOrFile").unwrap();
                call_jbang_sub_command(&["export", "native", script_or_file]);
            }
            "fatjar" => {
                let script_or_file = matches.get_one::<String>("scriptOrFile").unwrap();
                call_jbang_sub_command(&["export", "fatjar", script_or_file]);
            }
            "jlink" => {
                let script_or_file = matches.get_one::<String>("scriptOrFile").unwrap();
                call_jbang_sub_command(&["export", "jlink", script_or_file]);
            }
            _ => {}
        }
    }
}


pub fn build_export_command() -> Command {
    Command::new("export")
        .about("Export the result of a build.")
        .subcommand(
            Command::new("portable")
                .about("Exports jar together with dependencies in way that makes it portable")
                .arg(
                    Arg::new("scriptOrFile")
                        .help("A reference to a source file")
                        .num_args(1)
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("local")
                .about("Exports jar with classpath referring to local machine dependent locations.")
                .arg(
                    Arg::new("scriptOrFile")
                        .help("A reference to a source file")
                        .num_args(1)
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("mavenrepo")
                .about("Exports directory that can be used to publish as a maven repository")
                .arg(
                    Arg::new("scriptOrFile")
                        .help("A reference to a source file")
                        .num_args(1)
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("native")
                .about("Exports native executable")
                .arg(
                    Arg::new("scriptOrFile")
                        .help("A reference to a source file")
                        .num_args(1)
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("fatjar")
                .about("Exports an executable jar with all necessary dependencies included inside")
                .arg(
                    Arg::new("scriptOrFile")
                        .help("A reference to a source file")
                        .num_args(1)
                        .index(1)
                        .required(true)
                )
        ).subcommand(
        Command::new("jlink")
            .about("Exports a minimized JDK distribution")
            .arg(
                Arg::new("scriptOrFile")
                    .help("A reference to a source file")
                    .num_args(1)
                    .index(1)
                    .required(true)
            )
    )
}
