use std::collections::HashMap;
use std::path::PathBuf;
use clap::{Arg, Command};
use lazy_static::lazy_static;
use url::Url;
use crate::jbang_cli::jbang_catalog;
use crate::jbang_cli::models::Template;

lazy_static! {
    pub static ref TEMPLATES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("hello","Basic Hello World template");
        m.insert("hello.groovy","Basic groovy Hello World template");
        m.insert("hello.kt","Basic kotlin Hello World template");
        m.insert("cli","CLI template");
        m
    };
}
pub fn manage_template(template_matches: &clap::ArgMatches) {
    if let Some((sub_command, matches)) = template_matches.subcommand() {
        match sub_command {
            "list" => {
                list_templates();
            }
            "add" => {
                add_template(matches);
            }
            "remove" => {
                remove_template(matches);
            }
            _ => {}
        }
    }
}

pub fn add_template(matches: &clap::ArgMatches) {
    let name = matches.get_one::<String>("name").unwrap();
    let description = matches.get_one::<String>("description").map(|item| item.to_string());
    let file = matches.get_one::<String>("file").unwrap();
    let mut file_refs: HashMap<String, String> = HashMap::new();
    if file.starts_with("http://") || file.starts_with("https://") {
        let url = Url::parse(file).unwrap();
        let url_path = url.path();
        if let Some(pos) = url_path.rfind('/') {
            let file_name = &url_path[pos + 1..];
            let ext_name = if let Some(pos) = file_name.rfind('.') {
                file_name[pos + 1..].to_string()
            } else {
                "java".to_owned()
            };
            file_refs.insert(format!("{{basename}}.{}", ext_name), file.clone());
        }
    } else {
        let file_path = PathBuf::from(file);
        let absolute_path = std::fs::canonicalize(&file_path).unwrap();
        let absolute_path = absolute_path.as_path();
        let ext_name = absolute_path.extension().unwrap().to_str().unwrap();
        file_refs.insert(format!("{{basename}}.{}", ext_name), absolute_path.to_str().unwrap().to_string());
    }
    let template = Template {
        file_refs: file_refs,
        description: description,
        properties: None,
    };
    let mut jbang_catalog = jbang_catalog();
    jbang_catalog.add_template(name, template);
    jbang_catalog.write_default();
}

pub fn remove_template(matches: &clap::ArgMatches) {
    let name = matches.get_one::<String>("name").unwrap();
    let mut jbang_catalog = jbang_catalog();
    if let Some(templates) = &jbang_catalog.templates {
        if templates.contains_key(name) {
            jbang_catalog.remove_template(name);
            jbang_catalog.write_default();
        }
    }
}

pub fn list_templates() {
    for (key, value) in TEMPLATES.iter() {
        println!("{}", *key);
        println!("  {}", *value);
    }
    let jbang_catalog = jbang_catalog();
    if let Some(templates) = &jbang_catalog.templates {
        for (key, value) in templates.iter() {
            println!("{}", key);
            println!("  {}", value.description.as_ref().unwrap_or(&"No description".to_string()));
        }
    }
}
pub fn build_template_command() -> Command {
    Command::new("template")
        .about("Manage templates for scripts.")
        .subcommand(
            Command::new("add")
                .about("Add template for script reference.")
                .arg(
                    Arg::new("name")
                        .long("name")
                        .help("A name for the template")
                        .num_args(1)
                        .required(true)
                )
                .arg(
                    Arg::new("description")
                        .short('d')
                        .long("description")
                        .help("Rules for trusted sources")
                        .num_args(1)
                        .required(false)
                )
                .arg(
                    Arg::new("file")
                        .help("Path or URL to template file")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("remove")
                .about("Remove existing template.")
                .arg(
                    Arg::new("name")
                        .help("The name of the template")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("list")
                .about("Lists locally defined templates or from the given catalog.")
                .arg(
                    Arg::new("format")
                        .long("format")
                        .help("Specify output format ('text' or 'json')")
                        .num_args(1)
                        .required(false)
                        .value_parser(["text", "json"])
                )
                .arg(
                    Arg::new("catalogName")
                        .help("The name of a catalog.")
                        .index(1)
                        .required(false)
                )
        )
}
