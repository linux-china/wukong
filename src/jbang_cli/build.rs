use crate::jbang_cli::call_jbang_sub_command;
use clap::{Arg, Command};
use itertools::Itertools;

pub fn manage_build(build_matches: &clap::ArgMatches) {
    //let script_or_file = build_matches.get_one::<String>("scriptOrFile").unwrap();
    let args = std::env::args().collect::<Vec<String>>();
    let app_args = &args[2..].iter().map(|s| s.as_str()).collect_vec();
    call_jbang_sub_command(&app_args);
}
pub fn build_build_command() -> Command {
    Command::new("build")
        .about("Compiles and stores script in the cache.")
        .arg(
            Arg::new("scriptOrFile")
                .help("A reference to a source file")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::new("main")
                .help("Main class to use when running. Used primarily for running jar's.")
                .short('m')
                .long("main")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("deps")
                .long("deps")
                .help("Add additional dependencies (Use commas to separate them).")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("enable-preview")
                .long("enable-preview")
                .help("Activate Java preview features")
                .num_args(0)
                .required(false),
        )
        .arg(
            Arg::new("java")
                .long("java")
                .short('j')
                .help("JDK version to use for running the script.")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("java-options")
                .long("java-options")
                .help("Options to pass to the Java runtime.")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("native")
                .long("native")
                .short('n')
                .help("Build using native-image.")
                .num_args(0)
                .required(false),
        )
        .arg(
            Arg::new("insecure")
                .long("insecure")
                .help("Enable insecure trust of all SSL certificates.")
                .num_args(0)
                .required(false),
        )
}
