use clap::{Arg, ArgAction, Command};
use crate::jbang_cli::alias::build_alias_command;
use crate::jbang_cli::app::build_app_command;
use crate::jbang_cli::build::build_build_command;
use crate::jbang_cli::cache::build_cache_command;
use crate::jbang_cli::catalog::build_catalog_command;
use crate::jbang_cli::config::build_config_command;
use crate::jbang_cli::edit::build_edit_command;
use crate::jbang_cli::export::build_export_command;
use crate::jbang_cli::info::build_info_command;
use crate::jbang_cli::init::build_init_command;
use crate::jbang_cli::jdk::build_jdk_command;
use crate::jbang_cli::run::build_run_command;
use crate::jbang_cli::template::build_template_command;
use crate::jbang_cli::trust::build_trust_command;

pub const VERSION: &str = "0.2.1";

pub fn build_jbang_app() -> Command {
    let run_command = build_run_command();
    let build_command = build_build_command();
    let version_command = Command::new("version")
        .about("Display version info.");
    let jdk_command = build_jdk_command();
    let config_command = build_config_command();
    let trust_command = build_trust_command();
    let init_command = build_init_command();
    let edit_command = build_edit_command();
    let template_command = build_template_command();
    let alias_command = build_alias_command();
    let app_command = build_app_command();
    let catalog_command = build_catalog_command();
    let info_command = build_info_command();
    let export_command = build_export_command();
    let cache_command = build_cache_command();
    let upgrade_command = Command::new("upgrade")
        .about("Upgrade jbang to the latest version.");
    Command::new("jbang")
        .version(VERSION)
        .about("jbang - Unleash the power of Java")
        .arg(
            Arg::new("config")
                .long("config")
                .num_args(1)
                .help("Path to config file to be used instead of the default")
                .required(false),
        )
        .arg(
            Arg::new("fresh")
                .long("fresh")
                .action(ArgAction::SetTrue)
                .help("Make sure we use fresh (i.e. non-cached) resources.")
                .required(false),
        )
        .arg(
            Arg::new("insecure")
                .long("insecure")
                .action(ArgAction::SetTrue)
                .help("Enable insecure trust of all SSL certificates.")
                .required(false),
        )
        .arg(
            Arg::new("offline")
                .short('o')
                .long("offline")
                .action(ArgAction::SetTrue)
                .help("Work offline. Fail-fast if dependencies are missing.")
                .required(false),
        )
        .arg(
            Arg::new("preview")
                .long("preview")
                .action(ArgAction::SetTrue)
                .help("Enable jbang preview features.")
                .required(false),
        )
        .arg(
            Arg::new("quiet")
                .long("quiet")
                .action(ArgAction::SetTrue)
                .help("jbang will be quiet, only print when error occurs.")
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("jbang will be verbose on what it does.")
                .required(false),
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
        .subcommand(run_command)
        .subcommand(build_command)
        .subcommand(init_command)
        .subcommand(edit_command)
        .subcommand(cache_command)
        .subcommand(export_command)
        .subcommand(jdk_command)
        .subcommand(config_command)
        .subcommand(trust_command)
        .subcommand(alias_command)
        .subcommand(template_command)
        .subcommand(catalog_command)
        .subcommand(app_command)
        .subcommand(info_command)
        .subcommand(version_command)
        .subcommand(upgrade_command)

}
