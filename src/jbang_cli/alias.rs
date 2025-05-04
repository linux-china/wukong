use crate::jbang_cli::models::{Alias, JBangCatalog};
use crate::jbang_cli::{find_jbang_catalog_from_path, jbang_catalog};
use clap::{Arg, Command};
use colored::Colorize;
use std::path::PathBuf;

pub fn manage_alias(alias_matches: &clap::ArgMatches) {
    if let Some((sub_command, matches)) = alias_matches.subcommand() {
        match sub_command {
            "add" => {
                let name = matches.get_one::<String>("name").unwrap();
                let script_ref = matches.get_one::<String>("scriptOrFile").unwrap().clone();
                let mut description = matches
                    .get_one::<String>("description")
                    .map(|d| d.to_string());
                if description.is_none() {
                    description = get_description_value(script_ref.as_str());
                }
                let alias = Alias {
                    description,
                    script_ref,
                };
                add_alias(name, alias);
            }
            "remove" => {
                let alias_name = matches.get_one::<String>("name").unwrap();
                remove_alias(alias_name);
            }
            "list" => {
                list_aliases();
            }
            _ => {}
        }
    }
}

pub fn list_aliases() {
    let jbang_catalog = jbang_catalog();
    print_catalog_alias(&jbang_catalog);
    // list catalog from the current directory
    if let Some(project_catalog) = find_jbang_catalog_from_path(&PathBuf::from(".")) {
        print_catalog_alias(&project_catalog);
    }
}

fn print_catalog_alias(catalog: &JBangCatalog) {
    if let Some(alias_map) = &catalog.aliases {
        for (name, alias) in alias_map {
            if let Some(description) = &alias.description {
                println!(
                    "{}: {}\n  {}\n",
                    name.yellow(),
                    description,
                    alias.script_ref
                );
            } else {
                println!("{}\n  {}", name.yellow(), alias.script_ref);
            }
        }
    }
}

pub fn remove_alias(alias_name: &str) {
    let mut catalog = jbang_catalog();
    catalog.remove_alias(alias_name);
    catalog.write_default();
}

pub fn add_alias(name: &str, alias: Alias) {
    let mut catalog = jbang_catalog();
    catalog.add_alias(name, alias);
    catalog.write_default();
}

pub fn get_description_value(script_ref: &str) -> Option<String> {
    let script = std::fs::read_to_string(script_ref).unwrap_or_default();
    for line in script.lines() {
        if line.starts_with("//DESCRIPTION ") {
            let description = line.trim_start_matches("//DESCRIPTION ").trim();
            if !description.is_empty() {
                return Some(description.to_string());
            }
        }
    }
    None
}

pub fn build_alias_command() -> Command {
    Command::new("alias")
        .about("Manage aliases for scripts.")
        .subcommand(
            Command::new("add")
                .about("Add alias for script reference.")
                .arg(
                    Arg::new("name")
                        .long("name")
                        .help("A name for the alias")
                        .num_args(1)
                        .required(true),
                )
                .arg(
                    Arg::new("description")
                        .long("description")
                        .help("A description for the alias")
                        .num_args(1)
                        .required(false),
                )
                .arg(
                    Arg::new("file")
                        .short('f')
                        .help("Path to the catalog file to use")
                        .num_args(1)
                        .required(false),
                )
                .arg(
                    Arg::new("scriptOrFile")
                        .help("A reference to a source file")
                        .num_args(1)
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::new("params")
                        .help("Parameters to pass on to the script")
                        .num_args(1..)
                        .index(2)
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove existing alias.")
                .arg(
                    Arg::new("file")
                        .short('f')
                        .help("Path to the catalog file to use")
                        .num_args(1)
                        .required(false),
                )
                .arg(
                    Arg::new("name")
                        .help("The name of the alias")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("Lists locally defined aliases or from the given catalog.")
                .arg(
                    Arg::new("format")
                        .long("format")
                        .help("Specify output format ('text' or 'json')")
                        .num_args(1)
                        .required(false)
                        .value_parser(["text", "json"]),
                )
                .arg(
                    Arg::new("file")
                        .short('f')
                        .help("Path to the catalog file to use")
                        .num_args(1)
                        .required(false),
                )
                .arg(
                    Arg::new("catalogName")
                        .help("The name of a catalog.")
                        .index(1)
                        .required(false),
                ),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        list_aliases();
    }

    #[test]
    fn test_add_alias() {
        let name = "hello";
        let alias = Alias {
            description: Some("hello world".to_string()),
            script_ref:
                "https://github.com/jbangdev/jbang-examples/blob/HEAD/examples/helloworld.java"
                    .to_string(),
        };
        add_alias(name, alias);
    }

    #[test]
    fn test_remove() {
        let name = "hello";
        remove_alias(name);
    }

    #[test]
    fn test_description() {
        let script_ref = "tests/hello.java";
        let description = get_description_value(script_ref);
        assert_eq!(description, Some("hello world".to_string()));
    }
}
