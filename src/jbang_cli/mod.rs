pub mod config;
pub mod jdk;
pub mod trust;
pub mod upgrade;
pub mod init;
pub mod template;
pub mod run;

use std::path::PathBuf;
use crate::common::run_command;

pub const JBANG_DEFAULT_JAVA_VERSION: &str = "17";

pub fn jbang_home() -> PathBuf {
    if let Ok(jbang_home) = std::env::var("JBANG_DIR") {
        PathBuf::from(jbang_home)
    } else {
        dirs::home_dir().unwrap().join(".jbang")
    }
}

pub fn jdk_home(jdk_version: &str) -> PathBuf {
    jbang_home().join("cache").join("jdks").join(jdk_version)
}

pub fn java_exec(java_home: &PathBuf) -> String {
    if cfg!(target_os = "windows") {
        java_home.join("bin").join("java.exe").to_str().unwrap().to_string()
    } else {
        java_home.join("bin").join("java").to_str().unwrap().to_string()
    }
}

pub fn jbang_exec() -> PathBuf {
    let jbang_bin_dir = jbang_home().join("bin");
    if cfg!(windows) {
        jbang_bin_dir.join("jbang.cmd")
    } else {
        jbang_bin_dir.join("jbang")
    }
}

pub fn ensure_jdk_available(jdk_version: &str) -> PathBuf {
    let jdk_home = jbang_home().join("cache").join("jdks").join(jdk_version);
    if !jdk_home.exists() {
        crate::foojay::install_jdk(jdk_version, &jdk_home);
    }
    jdk_home
}

pub fn print_command_help(sub_command: &str) {
    let java_home = ensure_jdk_available(JBANG_DEFAULT_JAVA_VERSION);
    std::env::set_var("CLICOLOR_FORCE", "1");
    let jbang_home = jbang_home();
    let jbang_jar_path = jbang_home.join("bin").join("jbang.jar");
    let jbang_jar = jbang_jar_path.to_str().unwrap();
    let jbang_params = vec!["-classpath", jbang_jar, "dev.jbang.Main", sub_command, "--help"];
    run_command(&java_exec(&java_home), &jbang_params).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_command_help() {
        print_command_help("run");
    }
}
