use clap::{Arg, ArgAction, Command};
use itertools::Itertools;
use crate::common::{capture_command, run_command_line};
use crate::jbang_cli::{ensure_jdk_available, java_exec, jbang_home, JBANG_DEFAULT_JAVA_VERSION};

pub fn manage_run(run_matches: &clap::ArgMatches) {
    let script_or_file = run_matches.get_one::<String>("scriptOrFile").unwrap();
    let args = std::env::args().collect::<Vec<String>>();
    let app_args = &args[3..].iter().map(|s| s.as_str()).collect_vec();
    jbang_run(script_or_file, app_args);
}
pub fn jbang_run(script_or_file: &str, user_params: &[&str]) {
    let jdk_home = ensure_jdk_available(JBANG_DEFAULT_JAVA_VERSION);
    let java_exec = java_exec(&jdk_home);
    let jbang_home = jbang_home();
    let jbang_jar = jbang_home.join("bin").join("jbang.jar");
    // java -classpath $HOME/.jbang/bin/jbang.jar dev.jbang.Main run hello.java param1 param2
    let mut args = vec!["-classpath", jbang_jar.to_str().unwrap(), "dev.jbang.Main", "run", script_or_file];
    args.extend(user_params);
    let output = capture_command(&java_exec, &args).unwrap();
    let app_command_line = String::from_utf8_lossy(&output.stdout);
    run_command_line(app_command_line.trim()).unwrap();
}

pub fn build_run_command() -> Command {
    Command::new("run")
        .about("Builds and runs provided script.")
        .arg(
            Arg::new("main")
                .help("Main class to use when running. Used primarily for running jar's.")
                .short('m')
                .long("main")
                .num_args(1)
                .required(false)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jbang_run() {
        jbang_run("scripts/hello.java", &["first", "second"]);
    }
}
