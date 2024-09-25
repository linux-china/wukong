pub mod clap_app;

use std::fs;
use std::path::PathBuf;
use wukong::common::capture_command;
use wukong::foojay::install_jdk;

pub fn jenv_home() -> PathBuf {
    let home = dirs::home_dir().unwrap();
    home.join(".jenv")
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
            println!("{}", version.trim());
        } else {
            println!("system");
        }
    }
}

pub fn shell_command() {
    if let Some(jenv_version) = std::env::var("JENV_VERSION").ok() {
        println!("{}", jenv_version);
    } else {
        println!("jenv: no shell-specific version configured");
    }
}

pub fn version_command() {
    if let Ok(jenv_version) = std::env::var("JENV_VERSION") {
        println!("{} (set by JENV_VERSION environment variable)", jenv_version);
    } else if PathBuf::from(".java-version").exists() {
        let version = fs::read_to_string(".java-version").unwrap();
        println!("{} (set by .java-version)", version.trim());
    } else if jenv_home().join("version").exists() {
        let jenv_home = jenv_home();
        let version = fs::read_to_string(jenv_home.join("version")).unwrap();
        println!("{}(set by global)", version.trim());
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
        (version.trim().to_string(), "set by .java-version".to_owned())
    } else {
        ("".to_owned(), "".to_owned())
    };
    println!("  system");
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

pub fn which_command(command_matches: &clap::ArgMatches) {
    if let Some(command) = command_matches.get_one::<String>("command") {
        if let Some(jenv_version) = std::env::var("JENV_VERSION").ok() {
            let jenv_home = jenv_home();
            let java_home = jenv_home.join("versions").join(&jenv_version);
            println!("{}", java_home.join("bin").join(command).to_str().unwrap());
        } else if PathBuf::from(".java-version").exists() {
            let version = fs::read_to_string(".java-version").unwrap();
            let jenv_home = jenv_home();
            let java_home = jenv_home.join("versions").join(version.trim());
            println!("{}", java_home.join("bin").join(command).to_str().unwrap());
        } else if jenv_home().join("version").exists() {
            let jenv_home = jenv_home();
            let version = fs::read_to_string(jenv_home.join("version")).unwrap();
            let java_home = jenv_home.join("versions").join(version.trim());
            println!("{}", java_home.join("bin").join(command).to_str().unwrap());
        }
    }
}

pub fn whence_command(command_matches: &clap::ArgMatches) {
    if let Some(command) = command_matches.get_one::<String>("command") {
        let jenv_home = jenv_home();
        let versions_dir = jenv_home.join("versions");
        if versions_dir.exists() {
            for entry in fs::read_dir(versions_dir).unwrap() {
                let entry = entry.unwrap();
                let command_path = entry.path().join("bin").join(command);
                if command_path.exists() {
                    println!("{}", entry.file_name().to_str().unwrap());
                }
            }
        }
    }
}

pub fn add_command(command_matches: &clap::ArgMatches) {
    if let Some(version_or_path) = command_matches.get_one::<String>("versionOrPath") {
        if let Ok(num_version) = version_or_path.parse::<u32>() { // number
            let java_version = num_version.to_string();
            let java_home = jenv_home().join("versions").join(&java_version);
            if java_home.exists() {
                println!("version {} already exists", num_version);
            } else {
                println!("installing version {}", num_version);
                install_jdk(&java_version, &java_home);
                println!("version {} installed", num_version);
            }
        } else { // link java home with path
            let java_install_path = PathBuf::from(version_or_path);
            if java_install_path.exists() {
                let java_exec = java_exec(&java_install_path);
                if let Ok(output) = capture_command(&java_exec, &["-version"]) {
                    let result = if !output.stdout.is_empty() {
                        String::from_utf8_lossy(&output.stdout)
                    } else {
                        String::from_utf8_lossy(&output.stderr)
                    };
                    if result.is_empty() {
                        println!("Failed to execute {} -version", java_exec);
                        return;
                    }
                    let java_version = extract_java_version(&result);
                    let java_home = jenv_home().join("versions").join(&java_version);
                    symlink::symlink_dir(&java_install_path, &java_home).unwrap();
                } else {
                    println!("path {} is not a valid Java home", version_or_path);
                }
            } else {
                println!("path {} not exists", version_or_path);
            }
        }
    }
}

fn extract_java_version(text: &str) -> String {
    let first_line = text.lines().next().unwrap();
    if let Some(pos) = first_line.find("\"") {
        let end_pos = first_line.rfind("\"").unwrap();
        text[pos + 1..end_pos].to_string()
    } else {
        "".to_string()
    }
}

pub fn remove_command(command_matches: &clap::ArgMatches) {
    let java_version = command_matches.get_one::<String>("version").unwrap();
    let java_home = jenv_home().join("versions").join(&java_version);
    if java_home.exists() {
        if java_home.is_symlink() {
            symlink::remove_symlink_dir(&java_home).unwrap();
        } else {
            fs::remove_dir_all(java_home).unwrap();
        }
    } else {
        println!("version {} not exists", java_version);
    }
}

fn java_exec(java_home: &PathBuf) -> String {
    if cfg!(target_os = "windows") {
        java_home.join("bin").join("java.exe").to_str().unwrap().to_string()
    } else {
        java_home.join("bin").join("java").to_str().unwrap().to_string()
    }
}
#[cfg(test)]
mod tests {
    use crate::build_jenv_app;
    use super::*;

    #[test]
    fn test_extract_java_version() {
        let text = r#"openjdk version "1.8.0_332"
OpenJDK Runtime Environment (Temurin)(build 1.8.0_332-b09)
OpenJDK 64-Bit Server VM (Temurin)(build 25.332-b09, mixed mode)"#;
        println!("{}", extract_java_version(text));
        let text = r#"openjdk version "21" 2023-09-19
OpenJDK Runtime Environment (build 21+35-2513)
OpenJDK 64-Bit Server VM (build 21+35-2513, mixed mode, sharing)"#;
        println!("{}", extract_java_version(text));
    }
    #[test]
    fn test_add() {
        let app = build_jenv_app();
        let java_21_home = dirs::home_dir().unwrap().join(".jbang").join("cache").join("jdks").join("21");
        let matches = app.try_get_matches_from(vec!["jenv", "add", java_21_home.to_str().unwrap()]).unwrap();
        add_command(matches.subcommand_matches("add").unwrap());
    }

    #[test]
    fn test_remove() {
        let app = build_jenv_app();
        let matches = app.try_get_matches_from(vec!["jenv", "remove", "21"]).unwrap();
        remove_command(matches.subcommand_matches("remove").unwrap());
    }

    #[test]
    fn test_which() {
        let command = "javac";
        let app = build_jenv_app();
        let matches = app.try_get_matches_from(vec!["jenv", "which", command]).unwrap();
        which_command(matches.subcommand_matches("which").unwrap());
    }
}
