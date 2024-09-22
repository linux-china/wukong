use std::path::PathBuf;
use clap::{Arg, Command};
use colored::Colorize;
use crate::jbang_cli::{find_jbang_catalog_from_path, jbang_catalog};
use crate::jbang_cli::models::{CatalogRef, JBangCatalog};

pub fn manage_catalog(catalog_matches: &clap::ArgMatches) {
    if let Some((sub_command, matches)) = catalog_matches.subcommand() {
        match sub_command {
            "add" => {
                let name = matches.get_one::<String>("name").unwrap();
                let file = matches.get_one::<String>("file").unwrap();
                let description = matches.get_one::<String>("description").map(|d| d.to_string());
                let catalog_ref = CatalogRef {
                    catalog_ref: file.clone(),
                    description,
                    import_items: true,
                };
                add_catalog(name, catalog_ref);
            }
            "remove" => {
                let name = matches.get_one::<String>("name").unwrap();
                remove_catalog(name);
            }
            "list" => {
                list_catalogs();
            }
            _ => {}
        }
    }
}

pub fn list_catalogs() {
    // list jbang level catalogs
    let jbang_catalog = jbang_catalog();
    print_catalog(&jbang_catalog);
    // list catalog from current directory
    if let Some(project_catalog) = find_jbang_catalog_from_path(&PathBuf::from(".")) {
        print_catalog(&project_catalog);
    }
}

fn print_catalog(catalog: &JBangCatalog) {
    if let Some(catalog_map) = &catalog.catalogs {
        for (name, catalog_ref) in catalog_map {
            if let Some(description) = &catalog_ref.description {
                println!("{}: {}\n  {}\n", name.yellow(), description, catalog_ref.catalog_ref);
            } else {
                println!("{}\n  {}", name.yellow(), catalog_ref.catalog_ref);
            }
        }
    }
}

pub fn add_catalog(name: &str, catalog_ref: CatalogRef) {
    let mut jbang_catalog = jbang_catalog();
    jbang_catalog.add_catalog(name, catalog_ref);
    jbang_catalog.write_default();
}

pub fn remove_catalog(name: &str) {
    let mut jbang_catalog = jbang_catalog();
    jbang_catalog.remove_catalog(name);
    jbang_catalog.write_default();
}

pub fn build_catalog_command() -> Command {
    Command::new("catalog")
        .about("Manage Catalogs of aliases.")
        .subcommand(
            Command::new("add")
                .about("Add a catalog.")
                .arg(
                    Arg::new("name")
                        .long("name")
                        .help("A name for the alias")
                        .num_args(1)
                        .required(true)
                )
                .arg(
                    Arg::new("description")
                        .long("description")
                        .help("A description for the alias")
                        .num_args(1)
                        .required(false)
                )
                .arg(
                    Arg::new("file")
                        .help("Path or URL to alias file")
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::new("scriptOrFile")
                        .help("A reference to a source file")
                        .num_args(1)
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::new("params")
                        .help("Parameters to pass on to the script")
                        .num_args(1..)
                        .index(2)
                        .required(false)
                )
        )
        .subcommand(
            Command::new("remove")
                .about("Remove existing catalog.")
                .arg(
                    Arg::new("file")
                        .help("Path or URL to alias file")
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::new("name")
                        .help("The name of the alias")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("list")
                .about("Show currently defined catalogs.")
                .arg(
                    Arg::new("format")
                        .long("format")
                        .help("Specify output format ('text' or 'json')")
                        .num_args(1)
                        .required(false)
                        .value_parser(["text", "json"])
                )
                .arg(
                    Arg::new("file")
                        .short('f')
                        .help("Path to the catalog file to use")
                        .num_args(1)
                        .required(false)
                )
                .arg(
                    Arg::new("catalogName")
                        .help("The name of a catalog.")
                        .index(1)
                        .required(false)
                )
        )
        .subcommand(
            Command::new("update")
                .about("Retrieve the latest contents of the catalogs.")
                .arg(
                    Arg::new("format")
                        .long("format")
                        .help("Specify output format ('text' or 'json')")
                        .num_args(1)
                        .required(false)
                        .value_parser(["text", "json"])
                )
                .arg(
                    Arg::new("file")
                        .short('f')
                        .help("Path to the catalog file to use")
                        .num_args(1)
                        .required(false)
                )
                .arg(
                    Arg::new("catalogName")
                        .help("The name of a catalog.")
                        .index(1)
                        .required(false)
                )
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_catalogs() {
        list_catalogs();
    }

    #[test]
    fn test_remove_catalog() {
        let name = "demo";
        remove_catalog(name);
    }

    #[test]
    fn test_add_catalog() {
        let name = "demo";
        let catalog_ref = CatalogRef {
            catalog_ref: "https://github.com/jbangdev/jbang-catalog/blob/HEAD/jbang-catalog.json".to_string(),
            description: Some("Demo catalog".to_string()),
            import_items: true,
        };
        add_catalog(name, catalog_ref);
    }
}
