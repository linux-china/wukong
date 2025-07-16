use colored::Colorize;
use itertools::Itertools;
use pad::PadStr;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use walkdir::WalkDir;
use zip::ZipArchive;

pub mod clap_app;

fn get_major_version(zip_file: &mut dyn Read) -> u16 {
    let mut buffer = [0; 8];
    zip_file.read(&mut buffer).unwrap();
    // check if the file is a class file
    if &buffer[0..4] == b"\xCA\xFE\xBA\xBE" {
        // get major version from buffer[6] and buffer[7]
        let major_version = ((buffer[6] as u16) << 8) | (buffer[7] as u16);
        return major_version;
    }
    0
}

fn get_java_version(major_version: u16) -> &'static str {
    match major_version {
        45 => "1.1",
        46 => "1.2",
        47 => "1.3",
        48 => "1.4",
        49 => "5.0",
        50 => "6",
        51 => "7",
        52 => "8",
        53 => "9",
        54 => "10",
        55 => "11",
        56 => "12",
        57 => "13",
        58 => "14",
        59 => "15",
        60 => "16",
        61 => "17",
        62 => "18",
        63 => "19",
        64 => "20",
        65 => "21",
        66 => "22",
        67 => "23",
        68 => "24",
        69 => "25",
        _ => "Unknown",
    }
}

pub fn bytecode(command_matches: &clap::ArgMatches) {
    if let Some((command, command_matches)) = command_matches.subcommand() {
        match command {
            "show" => bytecode_show(command_matches),
            &_ => println!("Unknown command"),
        }
    }
}

pub fn bytecode_show(command_matches: &clap::ArgMatches) {
    let mut class_info = HashMap::<u16, u32>::new();
    if let Some(gav) = command_matches.get_one::<String>("gav") {
        let parts = gav.split(":").collect::<Vec<&str>>();
        let group = parts[0].replace('.', "/");
        let artifact = parts[1];
        let version = parts[2];
        let url = format!(
            "https://repo1.maven.org/maven2/{}/{}/{}/{}-{}.jar",
            group, artifact, version, artifact, version,
        );
        let mut res = reqwest::blocking::get(url).unwrap();
        let mut buf: Vec<u8> = Vec::new();
        let _ = res.read_to_end(&mut buf);
        let reader = std::io::Cursor::new(buf);
        let mut archive = ZipArchive::new(reader).unwrap();
        for i in 0..archive.len() {
            let mut zip_file = archive.by_index(i).unwrap();
            if zip_file.is_file() {
                let major_version = get_major_version(&mut zip_file);
                if major_version > 0 {
                    class_info
                        .entry(major_version)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }
        }
    } else if let Some(file) = command_matches.get_one::<String>("file") {
        let mut archive = ZipArchive::new(File::open(file).unwrap()).unwrap();
        for i in 0..archive.len() {
            let mut zip_file = archive.by_index(i).unwrap();
            if zip_file.is_file() {
                let major_version = get_major_version(&mut zip_file);
                if major_version > 0 {
                    class_info
                        .entry(major_version)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }
        }
    } else if let Some(url) = command_matches.get_one::<String>("url") {
        let mut res = reqwest::blocking::get(url).unwrap();
        let mut buf: Vec<u8> = Vec::new();
        let _ = res.read_to_end(&mut buf);
        let reader = std::io::Cursor::new(buf);
        let mut archive = ZipArchive::new(reader).unwrap();
        for i in 0..archive.len() {
            let mut zip_file = archive.by_index(i).unwrap();
            if zip_file.is_file() {
                let major_version = get_major_version(&mut zip_file);
                if major_version > 0 {
                    class_info
                        .entry(major_version)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }
        }
    } else if let Some(directory) = command_matches.get_one::<String>("directory") {
        for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let path = entry.path();
                if path.to_str().unwrap().ends_with(".jar") {
                    let mut archive = ZipArchive::new(File::open(path).unwrap()).unwrap();
                    for i in 0..archive.len() {
                        let mut zip_file = archive.by_index(i).unwrap();
                        if zip_file.is_file() {
                            let major_version = get_major_version(&mut zip_file);
                            if major_version > 0 {
                                class_info
                                    .entry(major_version)
                                    .and_modify(|e| *e += 1)
                                    .or_insert(1);
                            }
                        }
                    }
                }
            }
        }
    } else {
        println!("Please provide a valid GAV, file, or URL.");
        return;
    };
    if class_info.is_empty() {
        println!("No class files found in the provided input.");
        return;
    }
    // iterate through the class_info map and print the major version and count
    println!("Major Version | Java Version | Count");
    println!("--------------|--------------|-------");
    for (major_version, count) in class_info.iter().sorted_by_key(|x| x.1) {
        let java_version = get_java_version(*major_version);
        println!(
            "{:>13} | {:>12} | {:>5}",
            major_version.to_string().blue(),
            java_version.green(),
            count.to_string().yellow()
        );
    }
}

pub fn entries(command_matches: &clap::ArgMatches) {
    if let Some((command, command_matches)) = command_matches.subcommand() {
        match command {
            "extract" => entries_extract(command_matches),
            "find" => entries_find(command_matches),
            &_ => println!("Unknown command"),
        }
    }
}

pub fn entries_extract(command_matches: &clap::ArgMatches) {}

pub fn entries_find(command_matches: &clap::ArgMatches) {}

pub fn checksum(command_matches: &clap::ArgMatches) {
    let gav = command_matches.get_one::<String>("gav").unwrap();
    if gav.ends_with(".jar") {
        return;
    }
    let parts = gav.split(':').collect::<Vec<&str>>();
    let url = format!(
        "https://repo1.maven.org/maven2/{}/{}/{}/{}-{}.pom",
        parts[0].replace('.', "/"),
        parts[1],
        parts[2],
        parts[1],
        parts[2]
    );
}

pub fn jar_manifest(command_matches: &clap::ArgMatches) {
    if let Some((command, command_matches)) = command_matches.subcommand() {
        match command {
            "show" => jar_manifest_show(command_matches),
            "query" => jar_manifest_query(command_matches),
            &_ => println!("Unknown command"),
        }
    }
}

pub fn jar_manifest_show(command_matches: &clap::ArgMatches) {}

pub fn jar_manifest_query(command_matches: &clap::ArgMatches) {}

pub fn jar_module(command_matches: &clap::ArgMatches) {
    if let Some((command, command_matches)) = command_matches.subcommand() {
        match command {
            "name" => jar_module_name(command_matches),
            "descriptor" => jar_module_descriptor(command_matches),
            &_ => println!("Unknown command"),
        }
    }
}

pub fn jar_module_name(command_matches: &clap::ArgMatches) {}

pub fn jar_module_descriptor(command_matches: &clap::ArgMatches) {}

pub fn packages(command_matches: &clap::ArgMatches) {
    if let Some((command, command_matches)) = command_matches.subcommand() {
        match command {
            "show" => packages_show(command_matches),
            "query" => packages_query(command_matches),
            &_ => println!("Unknown command"),
        }
    }
}

pub fn packages_show(command_matches: &clap::ArgMatches) {}

pub fn packages_query(command_matches: &clap::ArgMatches) {}

pub fn services(command_matches: &clap::ArgMatches) {
    if let Some((command, command_matches)) = command_matches.subcommand() {
        match command {
            "show" => services_show(command_matches),
            "query" => services_query(command_matches),
            &_ => println!("Unknown command"),
        }
    }
}

pub fn services_show(command_matches: &clap::ArgMatches) {}

pub fn services_query(command_matches: &clap::ArgMatches) {}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::jarviz_cli::clap_app::build_jarviz_app;
    use dirs::home_dir;
    use std::fs::File;
    use zip::ZipArchive;

    #[test]
    fn test_bytecode() {
        let mcs_app = build_jarviz_app();
        let mcs_matches = mcs_app.get_matches_from(&vec!["mcs", "class-search", "VelocityEngine"]);
        let class_search_matches = mcs_matches.subcommand_matches("class-search").unwrap();
        bytecode(class_search_matches);
    }

    #[test]
    fn test_jar_extract() {
        let archive = File::open("/Users/linux_china/.m2/repository/org/apache/commons/commons-csv/1.14.0/commons-csv-1.14.0.jar").unwrap();
        let mut archive = ZipArchive::new(archive).unwrap();
        //list entries in the jar file
        for i in 0..archive.len() {
            let mut zip_file = archive.by_index(i).unwrap();
            if zip_file.is_file() {
                let major_version = get_major_version(&mut zip_file);
                let file_name = zip_file.name().to_string();
                if major_version > 0 {
                    println!(
                        "{}: {}",
                        file_name.pad_to_width(50).green(),
                        format!("Major Version: {}", major_version).blue()
                    );
                }
            }
        }
    }
    #[test]
    fn test_jar_extract_from_url() {
        let url = "https://repo1.maven.org/maven2/org/apache/commons/commons-csv/1.14.0/commons-csv-1.14.0.jar";
        let mut res = reqwest::blocking::get(url).unwrap();
        let mut buf: Vec<u8> = Vec::new();
        let _ = res.read_to_end(&mut buf);
        let reader = std::io::Cursor::new(buf);
        let mut archive = ZipArchive::new(reader).unwrap();
        //list entries in the jar file
        for i in 0..archive.len() {
            let mut zip_file = archive.by_index(i).unwrap();
            if zip_file.is_file() {
                let major_version = get_major_version(&mut zip_file);
                let file_name = zip_file.name().to_string();
                if major_version > 0 {
                    println!(
                        "{}: {}",
                        file_name.pad_to_width(50).green(),
                        format!("Major Version: {}", major_version).blue()
                    );
                }
            }
        }
    }

    #[test]
    fn test_entries() {
        let mcs_app = build_jarviz_app();
        let mcs_matches =
            mcs_app.get_matches_from(&vec!["mcs", "search", "spring-messaging", "--format=maven"]);
        let class_search_matches = mcs_matches.subcommand_matches("search").unwrap();
        bytecode(class_search_matches);
    }

    #[test]
    fn test_checksum() {
        let mcs_app = build_jarviz_app();
        let mcs_matches = mcs_app.get_matches_from(&vec![
            "mcs",
            "info",
            "org.apache.commons:commons-lang3:3.17.0",
        ]);
        let info_matches = mcs_matches.subcommand_matches("info").unwrap();
        checksum(info_matches);
    }

    #[test]
    fn test_jar_module() {
        let jar_file = home_dir()
            .unwrap()
            .join(".m2")
            .join("repository")
            .join("commons-io")
            .join("commons-io")
            .join("2.18.0")
            .join("commons-io-2.18.0.jar");
    }
}
