use clap::{Arg, Command};

pub const VERSION: &str = "0.2.3";


pub fn build_mt_app() -> Command {
    Command::new("mt")
        .version(VERSION)
        .about("mt - Maven Toolchains CLI")
        .subcommand(
            Command::new("jdks")
                .about("List all JDK on host")
        )
        .subcommand(
            Command::new("list")
                .about("List JDK in toolchains.xml")
        )
        .subcommand(
            Command::new("add")
                .about("Add JDK to toolchains.xml")
                .arg(
                    Arg::new("vendor")
                        .help("The java vendor")
                        .long("vendor")
                        .num_args(1)
                        .required(false)
                )
                .arg(
                    Arg::new("version")
                        .help("The java version")
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::new("path")
                        .help("Java home path")
                        .index(2)
                        .required(false)
                )
        )
        .subcommand(
            Command::new("remove")
                .about("Remove JDK from toolchains.xml")
                .arg(
                    Arg::new("vendor")
                        .help("The java vendor")
                        .long("vendor")
                        .num_args(1)
                        .required(false)
                )
                .arg(
                    Arg::new("version")
                        .help("The java version")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("vendors")
                .about("List all JDK vendors and versions")
        )
}
