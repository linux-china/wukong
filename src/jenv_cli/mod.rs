use std::fs;
use std::path::PathBuf;

pub fn jenv_home() -> PathBuf {
    let home = dirs::home_dir().unwrap();
    home.join(".jenv")
}

fn system_java_home() -> Option<String> {
    if let Ok(java_exec) = which::which("java") {
        java_exec.parent().unwrap().parent().unwrap().to_str().map(|s| s.to_string())
    } else {
        None
    }
}

fn get_shell_name() -> String {
    if let Ok(shell) = std::env::var("SHELL") {
        shell.split("/").last().unwrap().to_string()
    } else {
        "bash".to_string()
    }
}

pub fn init() {
    let jenv_home = jenv_home();
    let shell_name = get_shell_name();
    println!("export PATH=\"/{}/shims:${{PATH}}\"", jenv_home.to_str().unwrap());
    println!("export JENV_SHELL={}", shell_name);
    println!("export JENV_LOADED=1");
    println!("unset JAVA_HOME");
    println!("unset JDK_HOME");
    println!("source <(jenv completion {})", shell_name);
}

pub fn commands_command() {
    let commands = vec!["--version", "--help", "commands",
                        "local", "global", "shell", "version", "versions",
                        "which", "whence", "add", "remove", "completion"];
    println!("{}", commands.join(" "));
}

pub fn completion_command(command_matches: &clap::ArgMatches) {
    if let Some(shell) = command_matches.get_one::<String>("shell") {
        let shell = shell.to_lowercase();
        match shell.as_str() {
            "bash" => {
                println!("{}", include_str!("completions/jenv.bash"));
            }
            "zsh" => {
                println!("{}", include_str!("completions/jenv.zsh"));
            }
            "fish" => {
                println!("{}", include_str!("completions/jenv.fish"));
            }
            &_ => {}
        }
    } else {
        println!("jenv: no shell specified, please use: jenv completions bash/zsh/fish");
    }
}

pub fn local_command(command_matches: &clap::ArgMatches) {
    if let Some(version) = command_matches.get_one::<String>("version") {
        fs::write(".java-version", version).unwrap();
    } else {
        let java_version_file = PathBuf::from(".java-version");
        if java_version_file.exists() {
            let version = fs::read_to_string(java_version_file).unwrap();
            println!("{}", version);
        } else {
            println!("jenv: no local version configured for this directory");
        }
    }
}

pub fn global_command(command_matches: &clap::ArgMatches) {
    if let Some(version) = command_matches.get_one::<String>("version") {
        let jenv_home = jenv_home();
        let global_version_file = jenv_home.join("version");
        fs::write(global_version_file, version).unwrap();
    } else {
        let jenv_home = jenv_home();
        let global_version_file = jenv_home.join("version");
        if global_version_file.exists() {
            let version = fs::read_to_string(global_version_file).unwrap();
            println!("{}", version);
        } else {
            println!("system");
        }
    }
}

pub fn shell_command(command_matches: &clap::ArgMatches) {}

pub fn version_command() {
    if let Ok(jenv_version) = std::env::var("JENV_VERSION") {
        println!("{} (set by JENV_VERSION environment variable)", jenv_version);
    } else if PathBuf::from(".java-version").exists() {
        let version = fs::read_to_string(".java-version").unwrap();
        println!("{} (set by .java-version)", version);
    } else if jenv_home().join("version").exists() {
        let jenv_home = jenv_home();
        let version = fs::read_to_string(jenv_home.join("version")).unwrap();
        println!("{}(set by global)", version);
    } else {
        println!("system");
    }
}

pub fn versions_command() {
    let jenv_home = jenv_home();
    let versions_dir = jenv_home.join("versions");
    let (current_version, reason) = if let Ok(jenv_version) = std::env::var("JENV_VERSION") {
        (jenv_version, "set by JENV_VERSION environment variable".to_owned())
    } else if PathBuf::from(".java-version").exists() {
        let version = fs::read_to_string(".java-version").unwrap();
        (version, "set by .java-version".to_owned())
    } else {
        ("".to_owned(), "".to_owned())
    };
    if let Some(system_java_home) = system_java_home() {
        println!("  system({})", system_java_home);
    }
    if versions_dir.exists() {
        for entry in fs::read_dir(versions_dir).unwrap() {
            let entry = entry.unwrap();
            let file_name = entry.file_name();
            let java_version = file_name.to_str().unwrap();
            if java_version == current_version {
                println!("* {} ({})", java_version, reason);
            } else {
                println!("  {}", java_version);
            }
        }
    }
}

pub fn which_command(command_matches: &clap::ArgMatches) {}

pub fn whence_command(command_matches: &clap::ArgMatches) {}

pub fn add_command(command_matches: &clap::ArgMatches) {}

pub fn remove_command(command_matches: &clap::ArgMatches) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        println!("add");
    }
}
