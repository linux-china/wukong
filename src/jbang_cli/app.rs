use std::path::PathBuf;
use clap::{Arg, Command};
use crate::jbang_cli::{jbang_home, set_executable};

pub fn manage_app(app_matches: &clap::ArgMatches) {
    if let Some((sub_command, matches)) = app_matches.subcommand() {
        match sub_command {
            "install" => {
                let script_or_file = matches.get_one::<String>("scriptOrFile").unwrap();
                let command_name = if let Some(name) = matches.get_one::<String>("name") {
                    name.clone()
                } else if let Some(name) = derive_command_name(script_or_file) {
                    name
                } else {
                    eprintln!("Cannot derive command name from '{}', please use --name to specify it.", script_or_file);
                    return;
                };
                let user_params: Vec<&str> = matches.get_many::<String>("userParams")
                    .map(|values| values.map(|v| v.as_str()).collect())
                    .unwrap_or_default();
                install_app(&command_name, script_or_file, &user_params);
            }
            "uninstall" => {
                let name = matches.get_one::<String>("name").unwrap();
                let command_path = jbang_home().join("bin").join(name);
                if command_path.exists() {
                    std::fs::remove_file(&command_path).unwrap();
                } else {
                    eprintln!("Command not found: {}", command_path.to_str().unwrap());
                }
            }
            "list" => {
                list_apps();
            }
            "setup" => {
                if which::which("jbang").is_ok() {
                    println!("JBang environment is already set up.");
                } else {
                    let bin_path = jbang_home().join("bin");
                    let bin_path = bin_path.to_str().unwrap();
                    println!("Please add {} to PATH environment variable: export PATH=$PATH:{}", bin_path, bin_path);
                }
            }
            _ => println!("Unknown command"),
        }
    }
}

pub fn list_apps() {
    let bin_dir = jbang_home().join("bin");
    if bin_dir.exists() {
        for entry in std::fs::read_dir(bin_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let file_name = entry.file_name();
                let file_name = file_name.to_str().unwrap();
                if !file_name.starts_with(".") &&
                    file_name != "jbang" && !file_name.starts_with("jbang.") {
                    println!("{}", file_name);
                }
            }
        }
    }
}
/// Derive a command name from a script reference: local file, URL, GAV or alias.
/// e.g. `scripts/hello.java` -> `hello`, `https://github.com/x/y/raw/main/demo.jsh?x=1` -> `demo`,
/// `org.example:tool:1.0` -> `tool`, `hello@catalog` -> `hello`.
fn derive_command_name(script_or_file: &str) -> Option<String> {
    // strip URL query and fragment, then trailing slashes
    let reference = script_or_file.split(['?', '#']).next().unwrap_or("");
    let reference = reference.trim_end_matches(['/', '\\']);
    let is_path = reference.contains('/') || reference.contains('\\');
    let mut name = reference.rsplit(['/', '\\']).next().unwrap_or("");
    if !is_path && name.matches(':').count() >= 2 {
        // GAV: groupId:artifactId:version
        name = name.split(':').nth(1).unwrap_or("");
    } else if !is_path {
        // alias@catalog
        name = name.split('@').next().unwrap_or("");
    }
    if let Some(pos) = name.rfind('.') {
        if pos > 0 {
            name = &name[..pos];
        }
    }
    if name.is_empty() || name.contains(':') { None } else { Some(name.to_string()) }
}

pub fn install_app(command_name: &str, script_or_file: &str, user_params: &[&str]) {
    let file_path = PathBuf::from(script_or_file);
    let script_path = if file_path.exists() {
        let absolute_path = std::path::absolute(file_path).unwrap();
        absolute_path.to_str().unwrap().to_string()
    } else {
        script_or_file.to_string()
    };
    let command_path = jbang_home().join("bin").join(command_name);
    std::fs::write(&command_path, build_launcher_script(&script_path, user_params)).unwrap();
    set_executable(&command_path);
}

fn build_launcher_script(script_path: &str, user_params: &[&str]) -> String {
    let mut args = vec![script_path];
    args.extend_from_slice(user_params);
    let quoted_args = shlex::try_join(args).unwrap();
    format!("#!/bin/sh\nexec jbang run {} \"$@\"", quoted_args)
}

pub fn build_app_command() -> Command {
    Command::new("app")
        .about("Manage scripts installed on the user's PATH as commands.")
        .subcommand(
            Command::new("install")
                .about("Install a script as a command.")
                .arg(
                    Arg::new("name")
                        .long("name")
                        .help("A name for the command")
                        .num_args(1)
                        .required(false)
                )
                .arg(
                    Arg::new("scriptOrFile")
                        .help("A reference to a source file")
                        .index(1)
                        .required(true)
                )
                .arg(
                    Arg::new("userParams")
                        .help("Parameters to pass on to the script")
                        .index(2)
                        .num_args(1..)
                        .trailing_var_arg(true)
                        .allow_hyphen_values(true)
                        .required(false)
                )
        )
        .subcommand(
            Command::new("uninstall")
                .about("Removes a previously installed command.")
                .arg(
                    Arg::new("name")
                        .help("The name of the command")
                        .index(1)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("list")
                .about("Lists installed commands.")
                .arg(
                    Arg::new("format")
                        .long("format")
                        .help("Specify output format ('text' or 'json')")
                        .num_args(1)
                        .required(false)
                        .value_parser(["text", "json"])
                )
        )
        .subcommand(
            Command::new("setup")
                .about("Make jbang commands available for the user.")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_app() {
        install_app("hello", "scripts/hello.java", &["--verbose", "hello world"]);
    }

    #[test]
    fn test_derive_command_name() {
        assert_eq!(derive_command_name("hello.java").as_deref(), Some("hello"));
        assert_eq!(derive_command_name("scripts/hello.java").as_deref(), Some("hello"));
        assert_eq!(derive_command_name("./my.app/hello.java").as_deref(), Some("hello"));
        assert_eq!(derive_command_name("https://github.com/jbangdev/jbang-examples/blob/main/examples/helloworld.java?raw=true").as_deref(), Some("helloworld"));
        assert_eq!(derive_command_name("https://example.com/demo.jsh#main").as_deref(), Some("demo"));
        assert_eq!(derive_command_name("org.example:tool:1.0.0").as_deref(), Some("tool"));
        assert_eq!(derive_command_name("hello@jbangdev").as_deref(), Some("hello"));
        assert_eq!(derive_command_name("hello").as_deref(), Some("hello"));
        assert_eq!(derive_command_name("https://").as_deref(), None);
    }

    #[test]
    fn test_install_user_params() {
        let matches = build_app_command()
            .try_get_matches_from(["app", "install", "--name", "hi", "hello.java", "--verbose", "hello world"])
            .unwrap();
        let (_, install_matches) = matches.subcommand().unwrap();
        let user_params: Vec<&str> = install_matches.get_many::<String>("userParams")
            .unwrap().map(|v| v.as_str()).collect();
        assert_eq!(user_params, ["--verbose", "hello world"]);
        assert_eq!(
            build_launcher_script("hello.java", &user_params),
            "#!/bin/sh\nexec jbang run hello.java --verbose 'hello world' \"$@\""
        );
    }

    #[test]
    fn test_list_apps() {
        list_apps();
    }
}
