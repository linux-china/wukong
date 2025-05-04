use crate::jbang_cli::alias::{add_alias, get_description_value, list_aliases, remove_alias};
use crate::jbang_cli::call_jbang_sub_command;
use crate::jbang_cli::models::Alias;
use clap::{Arg, Command};
use itertools::Itertools;

pub fn manage_wrapper(wrapper_matches: &clap::ArgMatches) {
    if let Some((sub_command, matches)) = wrapper_matches.subcommand() {
        match sub_command {
            "install" => {
                let args = std::env::args().collect::<Vec<String>>();
                let mut install_args = vec!["wrapper", "install"];
                if args.len() > 3 {
                    let app_args = &args[3..].iter().map(|s| s.as_str()).collect_vec();
                    install_args.extend(app_args);
                }
                call_jbang_sub_command(install_args.as_slice());
            }
            _ => {}
        }
    }
}
pub fn build_wrapper_command() -> Command {
    Command::new("wrapper")
        .about("Manage jbang wrapper for a folder.")
        .subcommand(
            Command::new("install")
                .about("Install/Setup jbang as a `wrapper` script in a folder.")
                .arg(
                    Arg::new("force")
                        .long("force")
                        .short('f')
                        .help("Force installation of wrapper even if files already exist")
                        .num_args(0)
                        .required(false),
                )
                .arg(
                    Arg::new("dir")
                        .long("dir")
                        .short('d')
                        .help("The folder to install the wrapper into.")
                        .num_args(1)
                        .required(false),
                ),
        )
}
