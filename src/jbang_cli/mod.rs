pub mod config;
pub mod jdk;
pub mod trust;
pub mod init;
pub mod template;
pub mod run;
pub mod models;
pub mod alias;
pub mod catalog;
pub mod info;
pub mod export;
pub mod cache;
pub mod clap_app;
pub mod build;
pub mod edit;
pub mod app;
pub mod version;
pub mod completion;

use std::fs::{File, Permissions};
use std::io::Read;
use std::path::{Path, PathBuf};
use zip::ZipArchive;
use wukong::common::run_command;
use crate::jbang_cli::models::JBangCatalog;

pub const JBANG_DEFAULT_JAVA_VERSION: &str = "17";

pub fn jbang_home() -> PathBuf {
    wukong::common::jbang_home()
}

pub fn jdk_home(jdk_version: &str) -> PathBuf {
    jbang_home().join("cache").join("jdks").join(jdk_version)
}

pub fn jbang_catalog() -> JBangCatalog {
    let jbang_catalog_json = jbang_home().join("jbang-catalog.json");
    if !jbang_catalog_json.exists() {
        JBangCatalog {
            catalogs: None,
            aliases: None,
            templates: None,
        }
    } else {
        serde_json::from_reader(File::open(jbang_catalog_json).unwrap()).unwrap()
    }
}

pub fn builtin_jbang_catalog() -> JBangCatalog {
    let jbang_jar_file_path = jbang_home().join("bin").join("jbang.jar");
    let archive = File::open(jbang_jar_file_path).unwrap();
    let mut archive = ZipArchive::new(archive).unwrap();
    let zip_file = archive.by_name("jbang-catalog.json").unwrap();
    serde_json::from_reader(zip_file).unwrap()
}

pub fn find_jbang_catalog_from_path(path: &PathBuf) -> Option<JBangCatalog> {
    if path.join("jbang-catalog.json").exists() {
        serde_json::from_reader(File::open(path.join("jbang-catalog.json")).unwrap()).ok()
    } else if let Some(parent) = path.parent() {
        find_jbang_catalog_from_path(&PathBuf::from(parent))
    } else {
        None
    }
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
        wukong::foojay::install_jdk(jdk_version, &jdk_home);
    }
    jdk_home
}

pub fn call_jbang_sub_command(commands: &[&str]) {
    let java_home = ensure_jdk_available(JBANG_DEFAULT_JAVA_VERSION);
    std::env::set_var("CLICOLOR_FORCE", "1");
    let jbang_home = jbang_home();
    let jbang_jar_path = jbang_home.join("bin").join("jbang.jar");
    let jbang_jar = jbang_jar_path.to_str().unwrap();
    let mut jbang_params = vec!["-classpath", jbang_jar, "dev.jbang.Main"];
    jbang_params.extend(commands);
    run_command(&java_exec(&java_home), &jbang_params).unwrap();
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

#[cfg(unix)]
pub fn set_executable<P: AsRef<Path>>(path: P) {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, Permissions::from_mode(0o755)).unwrap();
}

#[cfg(not(unix))]
pub fn set_executable<P: AsRef<Path>>(path: P) {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_command_help() {
        print_command_help("run");
    }

    #[test]
    fn test_find_jbang_catalog() {
        assert!(find_jbang_catalog_from_path(&PathBuf::from(".")).is_none());
    }

    #[test]
    fn test_builtin_jbang_catalog() {
        let catalog = builtin_jbang_catalog();
        println!("{:?}", catalog);
    }
}
