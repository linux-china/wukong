use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use colored::Colorize;
use java_properties::PropertiesError;
use crate::jbang_cli::jbang_home;

fn jbang_config_path() -> PathBuf {
    jbang_home().join("jbang.properties")
}

fn read_config() -> Result<HashMap<String, String>, PropertiesError> {
    let config_path = jbang_config_path();
    let config = if config_path.exists() {
        let f = File::open(&config_path)?;
        java_properties::read(BufReader::new(f))?
    } else {
        HashMap::new()
    };
    Ok(config)
}

fn update_config(config: &HashMap<String, String>) {
    let f = File::open(jbang_config_path()).unwrap();
    java_properties::write(BufWriter::new(f), &config).unwrap();
}

fn read_config_with_default() -> Result<HashMap<String, String>, PropertiesError> {
    let mut config = read_config()?;
    if !config.contains_key("format") {
        config.insert("format".to_owned(), "text".to_owned());
    }
    if !config.contains_key("init.template") {
        config.insert("init.template".to_owned(), "hello".to_owned());
    }
    Ok(config)
}

pub fn manage_config(config_matches: &clap::ArgMatches) {
    if let Some((sub_command, matches)) = config_matches.subcommand() {
        match sub_command {
            "set" => {
                let key = matches.get_one::<String>("key").unwrap();
                let value = matches.get_one::<String>("value").unwrap();
                let mut config = read_config().unwrap();
                config.insert(key.to_string(), value.to_string());
                update_config(&config);
            }
            "get" => {
                let key = matches.get_one::<String>("key").unwrap();
                let config = read_config_with_default().unwrap();
                if let Some(value) = config.get(key) {
                    println!("{}", value);
                } else {
                    eprintln!("[jbang] No configuration option found with that name: {}", key);
                }
            }
            "unset" => {
                let key = matches.get_one::<String>("key").unwrap();
                let mut config = read_config().unwrap();
                if config.contains_key(key) {
                    config.remove(key);
                    update_config(&config);
                }
            }
            "list" => {
                let config = read_config_with_default().unwrap();
                for (key, value) in &config {
                    println!("{} = {}", key.bright_yellow(), value);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_properties() {
        let map = read_config().unwrap();
        println!("{:?}", map);
    }
}
