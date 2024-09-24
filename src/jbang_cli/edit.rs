use clap::{Arg, Command};
use crate::jbang_cli::call_jbang_sub_command;

pub fn manage_edit(edit_matches: &clap::ArgMatches) {
    let script_file = edit_matches.get_one::<String>("scriptOrFile").unwrap();
    call_jbang_sub_command(&["edit", script_file.as_str()]);
}

pub fn build_edit_command() -> Command {
    Command::new("edit")
        .about("Setup a temporary project to edit script in an IDE.")
        .arg(
            Arg::new("scriptOrFile")
                .help("A reference to a source file")
                .index(1)
                .required(false)
        )
        .arg(
            Arg::new("additionalFiles")
                .help("A reference to a source file")
                .index(2)
                .num_args(1..)
                .required(false)
        )
}
