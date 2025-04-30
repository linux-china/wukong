use std::fs::File;
use clap::{Arg, Command};
use tar::Archive;
use wukong::common::http_download;
use crate::jbang_cli::clap_app::VERSION;
use crate::jbang_cli::jbang_home;

pub fn manage_version(version_matches: &clap::ArgMatches) {
    if version_matches.get_flag("check") {
        check_jbang_version();
    } else if version_matches.get_flag("update") {
        println!("Updating jbang...");
        install_jbang();
    } else {
        display_version();
    }
}

pub fn get_jbang_version() -> String {
    std::fs::read_to_string(jbang_home().join("version.txt")).unwrap_or("unknown".to_string()).trim().to_string()
}

pub fn display_version() {
    println!("JBang: {}", get_jbang_version());
    println!("JBang-rs: {}", VERSION);
}

pub fn update_jbang() {
    println!("Updating jbang...");
    install_jbang();
}

pub fn check_jbang_version() {
    let url = "https://github.com/jbangdev/jbang/releases/latest/download/version.txt";
    let last_version = reqwest::blocking::get(url).unwrap().text().unwrap();
    let jbang_version = get_jbang_version();
    println!("{}", jbang_version);
    if jbang_version == last_version {
        println!("jbang is up-to-date");
    } else {
        println!("There is a new version of jbang available!");
        println!("You have version {} and {} is the latest.", jbang_version, last_version);
        println!("Run 'jbang version --update' to update to the latest version.");
    }
}

pub fn install_jbang() {
    let download_url = "https://github.com/jbangdev/jbang/releases/latest/download/jbang.tar";
    let temp_dir = std::env::temp_dir();
    let target_file_path = temp_dir.join("jbang.tar");
    http_download(&download_url, target_file_path.to_str().unwrap());
    let target_dir = jbang_home();
    let tar_file = File::open(&target_file_path).unwrap();
    let mut archive = Archive::new(tar_file);
    archive
        .entries().unwrap()
        .filter_map(|e| e.ok())
        .for_each(|mut entry| {
            let entry_path = entry.path().unwrap();
            let mut relative_path = entry_path.to_str().unwrap();
            if relative_path.starts_with("jbang/") {
                relative_path = &relative_path[(relative_path.find("/").unwrap() + 1)..];
            }
            let path = target_dir.join(relative_path);
            entry.set_preserve_mtime(true);
            entry.unpack(&path).unwrap();
        });
    std::fs::remove_file(&target_file_path).unwrap();
}

pub fn build_version_command() -> Command {
    Command::new("version")
        .about("Display version info.")
        .arg(
            Arg::new("check")
                .help("Check if a new version of jbang is available")
                .long("check")
                .num_args(0)
                .required(false)
        )
        .arg(
            Arg::new("update")
                .help("Update jbang to the latest version")
                .long("update")
                .num_args(0)
                .required(false)
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upgrade_jbang() {
        update_jbang();
    }

    #[test]
    fn test_check_jbang_version() {
        check_jbang_version();
    }
}
