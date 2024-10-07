use clap::{Arg, Command};
use itertools::Itertools;
use crate::jbang_cli::call_jbang_sub_command;

pub fn manage_cache(cache_matches: &clap::ArgMatches) {
    if let Some((sub_command, matches)) = cache_matches.subcommand() {
        match sub_command {
            "clear" => {
                let args = std::env::args().collect::<Vec<String>>();
                let app_args = &args[3..].iter().map(|s| s.as_str()).collect_vec();
                let mut command_args = vec!["cache", "clear"];
                command_args.extend(app_args);
                call_jbang_sub_command(&command_args);
            }
            _ => {}
        }
    }
}


pub fn build_cache_command() -> Command {
    Command::new("cache")
        .about("Manage compiled scripts in the local cache.")
        .subcommand(
            Command::new("clear")
                .about("Clear the various caches used by jbang. By default this will clear the JAR, script, stdin and URL caches. To clear other caches list them explicitly i.e. '--project' for temporary projects.")
                .arg(
                    Arg::new("all")
                        .help("clear all caches")
                        .long("all")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("deps")
                        .help("clear dependency cache only")
                        .long("deps")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("groovyc")
                        .help("clear groovyc cache only")
                        .long("groovyc")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("jar")
                        .help("clear JAR cache only")
                        .long("jar")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("jdk")
                        .help("clear JDK cache only")
                        .long("jdk")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("kotlinc")
                        .help("clear kotlinc cache only")
                        .long("kotlinc")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("project")
                        .help("clear temporary projects cache only")
                        .long("project")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("script")
                        .help("clear script cache only")
                        .long("script")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("stdin")
                        .help("clear stdin cache only")
                        .long("stdin")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("url")
                        .help("clear URL cache only")
                        .long("url")
                        .num_args(0)
                        .required(false)
                )
        )
}
