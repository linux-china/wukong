use clap::{Arg, Command};

pub fn manage_template(template_matches: &clap::ArgMatches) {
   // TODO implement manage_template
}
pub fn build_template_command() -> Command {
    Command::new("template")
        .about("Manage templates for scripts.")
        .subcommand(
            Command::new("add")
                .about("Add template for script reference.")
                .arg(
                    Arg::new("name")
                        .long("name")
                        .help("A name for the template")
                        .num_args(1)
                        .required(true)
                )
                .arg(
                    Arg::new("description")
                        .short('d')
                        .long("description")
                        .help("Rules for trusted sources")
                        .num_args(1)
                        .required(false)
                )
                .arg(
                    Arg::new("file")
                        .help("Path or URL to template file")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("remove")
                .about("Remove existing template.")
                .arg(
                    Arg::new("name")
                        .help("The name of the template")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("list")
                .about("Lists locally defined templates or from the given catalog.")
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
