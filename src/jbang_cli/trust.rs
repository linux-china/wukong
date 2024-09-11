use clap::{Arg, Command};
use crate::jbang_cli::jbang_home;
use itertools::Itertools;

const COMMENTS: &str = r#"
// URL's matching one or more entries in the list below will be trusted to be runnable by jbang.
// The following examples show what entries can look like:
// - "https://jbang.dev": Matches this specific domain using https
// - "https://jbang.dev/foo": Matches https://jbang.dev/foo and https://jbang.dev/foo/bar,
//   but not https://jbang.dev/foobar or https://jbang.dev/bar
// - "https://*.jbang.dev": Match all domains ending in "jbang.dev" using https
// - "jbang.dev": Match this specific domain using either http or https
// - "*.jbang.dev": Match all domains ending in "jbang.dev" using either http or https
// - "*": Match all sources using either http or https (This effectively disables any checks. Be careful!)
//
// By default, jbang trusts "localhost" and file://.
// You can use the "jbang --trust=rule1,rule2,.." command to add to this file.
// Note: --trust will rewrite this file. Preserving just the elements. Additional data and comments will not be kept.
"#;

fn read_trusted_domains() -> Vec<String> {
    let json_text = jbang_home().join("trusted-sources.json");
    if json_text.exists() {
        let data = std::fs::read_to_string(json_text).unwrap();
        let json_text = data.lines().filter(|line| !line.starts_with("//")).join("\n");
        serde_json::from_str::<Vec<String>>(&json_text).unwrap()
    } else {
        vec![]
    }
}

fn write_trusted_domains(domains: &[String]) {
    let source_file = jbang_home().join("trusted-sources.json");
    let data = format!("{}\n{}", COMMENTS, serde_json::to_string_pretty(domains).unwrap());
    std::fs::write(source_file, data).unwrap();
}

pub fn manage_trust(trust_matches: &clap::ArgMatches) {
    if let Some((sub_command, matches)) = trust_matches.subcommand() {
        match sub_command {
            "add" => {
                let arg0 = matches.get_one::<String>("arg0").unwrap();
                let mut domains = read_trusted_domains();
                if !domains.contains(arg0) {
                    domains.push(arg0.clone());
                    write_trusted_domains(&domains);
                }
            }
            "remove" => {
                let arg0 = matches.get_one::<String>("arg0").unwrap();
                let mut domains = read_trusted_domains();
                domains.retain(|domain| domain != arg0);
                write_trusted_domains(&domains);
            }
            "list" => {
                let domains = read_trusted_domains();
                let mut seq = 1;
                for domain in domains {
                    println!("{} = {}", seq, domain);
                    seq = seq + 1;
                }
            }
            _ => {}
        }
    }
}

pub fn build_trust_command() -> Command {
    Command::new("trust")
        .about("Manage which domains you trust to run scripts from.")
        .subcommand(
            Command::new("add")
                .about("Add trust domains.")
                .arg(
                    Arg::new("arg0")
                        .help("Rules for trusted sources")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("remove")
                .about("Remove trust domains.")
                .arg(
                    Arg::new("arg0")
                        .help("Rules for trusted sources")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("list")
                .about("Show defined trust domains.")
                .arg(
                    Arg::new("format")
                        .long("format")
                        .help("Specify output format ('text' or 'json')")
                        .num_args(1)
                        .required(false)
                        .value_parser(["text", "json"])
                )
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_trusted_domains() {
        println!("{:?}", read_trusted_domains());
    }
}
