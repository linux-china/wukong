use clap::{Arg, ArgAction, Command};

pub const VERSION: &str = "0.1.0";

pub fn build_mcs_app() -> Command {
    Command::new("mcs")
        .version(VERSION)
        .about("mcs - Maven Central Search")
        .subcommand(
            Command::new("search")
                .about("Search artifacts in Maven Central by coordinates")
                .arg(
                    Arg::new("format")
                        .help("Show result in <type> format, Supported types are: maven, gradle, sbt, ivy, grape, leiningen, jbang, gav")
                        .long("format")
                        .short('f')
                        .num_args(1)
                        .required(false),
                )
                .arg(
                    Arg::new("limit")
                        .help("Show <count> results")
                        .long("limit")
                        .short('l')
                        .num_args(1)
                        .required(false),
                )
                .arg(
                    Arg::new("query")
                        .help("What to search for. If the search term contains a colon( : ), such as 'g:org.apache.commons' or `a:commons-lang3', it is considered a literal groupId and artifactId Otherwise, the search term is considered a wildcard search")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("class-search")
                .about("Search artifacts in Maven Central by class name")
                .arg(
                    Arg::new("full-name")
                        .help("The class name to search for.")
                        .long("full-name")
                        .short('f')
                        .action(ArgAction::SetTrue)
                        .required(false),
                )
                .arg(
                    Arg::new("limit")
                        .help("Show <count> results")
                        .long("limit")
                        .short('l')
                        .num_args(1)
                        .required(false),
                )
                .arg(
                    Arg::new("query")
                        .help("The class name to search for.")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("info")
                .about("Display artifact information by GAV")
                .arg(
                    Arg::new("gav")
                        .help("GAV coordinates of the artifact to display, such as 'org.apache.commons:commons-lang3:3.17.0' or local jar file")
                        .index(1)
                        .required(true),
                )
        )
}
