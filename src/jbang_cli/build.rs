use clap::{Arg, Command};
use itertools::Itertools;
use crate::jbang_cli::call_jbang_sub_command;

pub fn manage_build(build_matches: &clap::ArgMatches) {
    let script_or_file = build_matches.get_one::<String>("scriptOrFile").unwrap();
    call_jbang_sub_command(&["build", script_or_file.as_str()]);
}
pub fn build_build_command() -> Command {
    Command::new("build")
        .about("Compiles and stores script in the cache.")
        .arg(
            Arg::new("scriptOrFile")
                .help("A reference to a source file")
                .index(1)
                .required(false)
        )
}
