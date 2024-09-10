use clap::{Arg, Command};

pub fn manage_trust(trust_matches: &clap::ArgMatches) {
    if let Some((sub_command, matches)) = trust_matches.subcommand() {
        match sub_command {
            "add" => {
                let arg0 = matches.get_one::<String>("arg0").unwrap();
            }
            "remove" => {
                let arg0 = matches.get_one::<String>("arg0").unwrap();
            }
            "list" => {

            }
            _ => {}
        }
    }
}

pub fn build_trust_command() -> Command {
    Command::new("trust")
        .about("Manage which domains you trust to run scripts from.")
        .subcommand(
            Command::new("add")
                .about("Add trust domains.")
                .arg(
                    Arg::new("arg0")
                        .help("Rules for trusted sources")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("remove")
                .about("Remove trust domains.")
                .arg(
                    Arg::new("arg0")
                        .help("Rules for trusted sources")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("list")
                .about("Show defined trust domains.")
                .arg(
                    Arg::new("format")
                        .long("format")
                        .help("Specify output format ('text' or 'json')")
                        .num_args(1)
                        .required(false)
                        .value_parser(["text", "json"])
                )
        )
}
