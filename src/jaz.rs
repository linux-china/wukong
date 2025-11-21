use dotenvx_rs::dotenvx;
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

struct JavaInfo {
    home: String,
    version: String,
    vendor: String,
}

fn main() {
    let jaz_args = env::args().collect::<Vec<_>>();
    let mut java_args: Vec<String> = vec![];
    java_args.extend(jaz_args[1..].iter().map(|s| s.to_string()));
    // .env load with dotenvx
    dotenvx::dotenv().ok();
    // construct command
    let mut command = Command::new("java");
    command
        .envs(env::vars())
        .args(&java_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    let mut child = command.spawn().unwrap();
    let status = child.wait().unwrap();
    let exit_code = if let Some(code) = status.code() {
        code
    } else {
        // On Unix, process was terminated by signal
        #[cfg(unix)]
        {
            use std::os::unix::process::ExitStatusExt;
            if let Some(signal) = status.signal() {
                std::process::exit(128 + signal);
            }
        }
        1
    };
    std::process::exit(exit_code);
}

fn get_java_info() -> JavaInfo {
    let java_home = get_java_home().unwrap();
    let java_home_path = PathBuf::from(&java_home);
    let java_exe_path = java_home_path.join("bin").join("java");
    let mut command = Command::new(java_exe_path);
    command.arg("--version");
    let output = command.output().expect("Failed to execute command");
    let version_output = String::from_utf8_lossy(&output.stdout);
    let version_lines: Vec<&str> = version_output.lines().collect();
    let version_line = version_lines[0];
    let parts: Vec<&str> = version_line
        .split(" ")
        .map(|s| s.trim_matches(|c| c == '"' || c == '\''))
        .collect();
    let mut java_version = parts.iter().find(|part| part[0..2].parse::<i32>().is_ok());

    JavaInfo {
        home: java_home,
        version: java_version.unwrap_or(&"21").to_string(),
        vendor: "".to_string(),
    }
}

pub fn get_java_home() -> Option<String> {
    if let Ok(java_home) = env::var("JAVA_HOME") {
        return Some(java_home);
    } else if let Ok(java_exec) = which::which("java") {
        if java_exec.is_symlink() {
            if let Ok(target) = std::fs::read_link(&java_exec) {
                return Some(
                    target
                        .parent()
                        .unwrap()
                        .parent()
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                );
            }
        } else {
            return Some(
                java_exec
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
            );
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_java_version() {
        let info = get_java_info();
        println!("Java version: {}", info.version)
    }

    #[test]
    fn test_get_java_home() {
        let java_home = get_java_home();
        println!("{:?}", java_home);
    }
}
