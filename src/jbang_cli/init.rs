use std::collections::HashMap;
use std::fs::Permissions;
use clap::{Arg, Command};
use handlebars::{Handlebars};

const TEMPLATES: &[&str] = &["hello"];

fn handlebars() -> Handlebars<'static> {
    let mut hbs = Handlebars::new();
    hbs.register_template_string("hello", include_str!("templates/hello.java")).unwrap();
    hbs
}

pub fn manage_init(init_matches: &clap::ArgMatches) {
    let mut script_file = init_matches.get_one::<String>("scriptOrFile").unwrap().to_string();
    let default_template = "hello".to_owned();
    let template = init_matches.get_one::<String>("template").unwrap_or(&default_template);
    if !TEMPLATES.contains(&template.as_str()) {
        eprintln!("Unknown template: {}", template);
        return;
    }
    let mut class_name = script_file.clone();
    if !script_file.contains('.') {
        script_file = format!("{}.java", script_file);
    } else {
        class_name = script_file.split('.').next().unwrap().to_string();
    }
    let mut context: HashMap<String, String> = HashMap::new();
    context.insert("className".to_string(), class_name);
    let code = handlebars().render(template, &context).unwrap();
    std::fs::write(&script_file, code).unwrap();
    set_executable(&script_file);
    println!("Script file: {}", script_file);
}

#[cfg(unix)]
fn set_executable(path: &str) {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, Permissions::from_mode(0o755)).unwrap();
}

#[cfg(not(unix))]
fn set_executable(path: &str) {}

pub fn build_init_command() -> Command {
    Command::new("init")
        .about("Builds and runs provided script.")
        .arg(
            Arg::new("template")
                .short('t')
                .long("template")
                .num_args(1)
                .help("Init script with a java class useful for scripting")
                .required(false)
        )
        .arg(
            Arg::new("scriptOrFile")
                .help(" file or URL to a Java code file")
                .index(1)
                .required(true)
        )
        .arg(
            Arg::new("params")
                .help("Parameters to pass on to the generation.")
                .required(false)
                .index(2)
                .num_args(1..)
        )
}
