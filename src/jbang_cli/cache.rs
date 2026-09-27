use std::collections::HashMap;
use clap::{Arg, Command};
use wukong::common::jbang_home;

pub fn manage_cache(cache_matches: &clap::ArgMatches) {
    if let Some((sub_command, matches)) = cache_matches.subcommand() {
        match sub_command {
            "clear" => {
                let cache_path = jbang_home().join("cache");
                let mut names: HashMap<&str, &str> = HashMap::new();
                names.insert("deps", "deps");
                names.insert("groovyc", "groovycs");
                names.insert("jar", "jars");
                names.insert("jdk", "jdks");
                names.insert("kotlinc", "kotlincs");
                names.insert("project", "projects");
                names.insert("script", "scripts");
                names.insert("stdin", "stdins");
                names.insert("url", "urls");
                let none_selected = !names.keys().any(|key| matches.get_flag(key));
                // `--all` clears everything, including JDKs
                if matches.get_flag("all") {
                    if cache_path.exists() {
                        std::fs::remove_dir_all(cache_path).unwrap();
                    }
                    return;
                }
                // no cache specified: clear the default (safe) set, same as upstream JBang,
                // JDKs, projects, kotlinc and groovyc caches are kept
                if none_selected {
                    for value in ["urls", "jars", "scripts", "stdins", "deps"] {
                        let cache_path = cache_path.join(value);
                        if cache_path.exists() {
                            std::fs::remove_dir_all(cache_path).unwrap();
                        }
                    }
                    return;
                }
                for (key, value) in names {
                    if matches.get_flag(key) {
                        let cache_path = cache_path.join(value);
                        if cache_path.exists() {
                            std::fs::remove_dir_all(cache_path).unwrap();
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn build_cache_command() -> Command {
    Command::new("cache")
        .about("Manage compiled scripts in the local cache.")
        .subcommand(
            Command::new("clear")
                .about("Clear the various caches used by jbang. By default this will clear the JAR, script, stdin, URL and dependency caches. To clear other caches list them explicitly i.e. '--jdk' for JDKs, '--project' for temporary projects, or use '--all'.")
                .arg(
                    Arg::new("all")
                        .help("clear all caches, including JDKs")
                        .long("all")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("deps")
                        .help("clear dependency cache only")
                        .long("deps")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("groovyc")
                        .help("clear groovyc cache only")
                        .long("groovyc")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("jar")
                        .help("clear JAR cache only")
                        .long("jar")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("jdk")
                        .help("clear JDK cache only")
                        .long("jdk")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("kotlinc")
                        .help("clear kotlinc cache only")
                        .long("kotlinc")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("project")
                        .help("clear temporary projects cache only")
                        .long("project")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("script")
                        .help("clear script cache only")
                        .long("script")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("stdin")
                        .help("clear stdin cache only")
                        .long("stdin")
                        .num_args(0)
                        .required(false)
                )
                .arg(
                    Arg::new("url")
                        .help("clear URL cache only")
                        .long("url")
                        .num_args(0)
                        .required(false)
                )
        )
}
