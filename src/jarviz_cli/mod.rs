use colored::Colorize;
use pad::PadStr;
use serde::{Deserialize, Serialize};
use std::io::Read;

pub mod clap_app;

pub fn bytecode(command_matches: &clap::ArgMatches) {
    if let Some((command, command_matches)) = command_matches.subcommand() {
        match command {
            "show" => bytecode_show(command_matches),
            &_ => println!("Unknown command"),
        }
    }
}

pub fn bytecode_show(command_matches: &clap::ArgMatches) {
    let gav = command_matches.get_one::<String>("gav").unwrap();
    let default_format = "gav".to_owned();
}

pub fn entries(command_matches: &clap::ArgMatches) {
    if let Some((command, command_matches)) = command_matches.subcommand() {
        match command {
            "extract" => entries_extract(command_matches),
            "find" => entries_find(command_matches),
            &_ => println!("Unknown command"),
        }
    }
}

pub fn entries_extract(command_matches: &clap::ArgMatches) {}

pub fn entries_find(command_matches: &clap::ArgMatches) {}

pub fn checksum(command_matches: &clap::ArgMatches) {
    let gav = command_matches.get_one::<String>("gav").unwrap();
    if gav.ends_with(".jar") {
        return;
    }
    let parts = gav.split(':').collect::<Vec<&str>>();
    let url = format!(
        "https://repo1.maven.org/maven2/{}/{}/{}/{}-{}.pom",
        parts[0].replace('.', "/"),
        parts[1],
        parts[2],
        parts[1],
        parts[2]
    );
}

pub fn jar_manifest(command_matches: &clap::ArgMatches) {
    if let Some((command, command_matches)) = command_matches.subcommand() {
        match command {
            "show" => jar_manifest_show(command_matches),
            "query" => jar_manifest_query(command_matches),
            &_ => println!("Unknown command"),
        }
    }
}

pub fn jar_manifest_show(command_matches: &clap::ArgMatches) {}

pub fn jar_manifest_query(command_matches: &clap::ArgMatches) {}

pub fn jar_module(command_matches: &clap::ArgMatches) {
    if let Some((command, command_matches)) = command_matches.subcommand() {
        match command {
            "name" => jar_module_name(command_matches),
            "descriptor" => jar_module_descriptor(command_matches),
            &_ => println!("Unknown command"),
        }
    }
}

pub fn jar_module_name(command_matches: &clap::ArgMatches) {}

pub fn jar_module_descriptor(command_matches: &clap::ArgMatches) {}

pub fn packages(command_matches: &clap::ArgMatches) {
    if let Some((command, command_matches)) = command_matches.subcommand() {
        match command {
            "show" => packages_show(command_matches),
            "query" => packages_query(command_matches),
            &_ => println!("Unknown command"),
        }
    }
}

pub fn packages_show(command_matches: &clap::ArgMatches) {}

pub fn packages_query(command_matches: &clap::ArgMatches) {}

pub fn services(command_matches: &clap::ArgMatches) {
    if let Some((command, command_matches)) = command_matches.subcommand() {
        match command {
            "show" => services_show(command_matches),
            "query" => services_query(command_matches),
            &_ => println!("Unknown command"),
        }
    }
}

pub fn services_show(command_matches: &clap::ArgMatches) {}

pub fn services_query(command_matches: &clap::ArgMatches) {}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::jarviz_cli::clap_app::build_jarviz_app;
    use crate::jarviz_cli::clap_app::build_jarviz_app;
    use dirs::home_dir;

    #[test]
    fn test_bytecode() {
        let mcs_app = build_jarviz_app();
        let mcs_matches = mcs_app.get_matches_from(&vec!["mcs", "class-search", "VelocityEngine"]);
        let class_search_matches = mcs_matches.subcommand_matches("class-search").unwrap();
        bytecode(class_search_matches);
    }

    #[test]
    fn test_entries() {
        let mcs_app = build_jarviz_app();
        let mcs_matches =
            mcs_app.get_matches_from(&vec!["mcs", "search", "spring-messaging", "--format=maven"]);
        let class_search_matches = mcs_matches.subcommand_matches("search").unwrap();
        bytecode(class_search_matches);
    }

    #[test]
    fn test_checksum() {
        let mcs_app = build_jarviz_app();
        let mcs_matches = mcs_app.get_matches_from(&vec![
            "mcs",
            "info",
            "org.apache.commons:commons-lang3:3.17.0",
        ]);
        let info_matches = mcs_matches.subcommand_matches("info").unwrap();
        checksum(info_matches);
    }

    #[test]
    fn test_jar_module() {
        let jar_file = home_dir()
            .unwrap()
            .join(".m2")
            .join("repository")
            .join("commons-io")
            .join("commons-io")
            .join("2.18.0")
            .join("commons-io-2.18.0.jar");
    }
}
