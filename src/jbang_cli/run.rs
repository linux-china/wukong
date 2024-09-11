use clap::{Arg, ArgAction, Command};
use itertools::Itertools;
use crate::common::run_command;
use crate::jbang_cli::jbang_exec;

pub fn manage_run(run_matches: &clap::ArgMatches) {
    let script_or_file = run_matches.get_one::<String>("scriptOrFile").unwrap();
    let params: Vec<&String> = if let Some(user_params) = run_matches.get_many::<String>("userParams") {
        user_params.collect()
    } else {
        vec![]
    };
    run(script_or_file, &params.iter().map(|s| s.as_str()).collect_vec());
}
pub fn run(script_or_file: &str, user_params: &[&str]) {
    let mut jbang_params = vec!["run", script_or_file];
    jbang_params.extend(user_params);
    run_command(jbang_exec().to_str().unwrap(), &jbang_params).unwrap();
}

pub fn build_run_command() -> Command {
    Command::new("run")
        .about("Builds and runs provided script.")
        .arg(
            Arg::new("main")
                .help("Main class to use when running. Used primarily for running jar's.")
                .short('m')
                .long("main")
                .num_args(1)
                .required(false)
        )
        .arg(
            Arg::new("scriptOrFile")
                .help("A reference to a source file")
                .index(1)
                .required(false)
        )
        .arg(
            Arg::new("userParams")
                .help("Parameters to pass on to the script")
                .index(2)
                .num_args(1..)
                .action(ArgAction::Append)
                .required(false)
        )
}
