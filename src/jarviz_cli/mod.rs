use crate::jarviz_cli::bytecode::get_major_version;
use colored::Colorize;
use itertools::Itertools;
use pad::PadStr;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use walkdir::WalkDir;
use zip::ZipArchive;

pub mod bytecode;
pub mod clap_app;

fn scan_local_archive<P: AsRef<Path>>(
    path: P,
    class_info: &mut HashMap<u16, u32>,
    include_details: bool,
    class_details: &mut HashMap<u16, Vec<String>>,
) {
    let mut archive = build_archive_from_local(path);
    for i in 0..archive.len() {
        let mut zip_file = archive.by_index(i).unwrap();
        if zip_file.is_file() {
            let major_version = get_major_version(&mut zip_file);
            if major_version > 0 {
                class_info
                    .entry(major_version)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
                if include_details {
                    let class_full_name = zip_file.name().to_string();
                    class_details
                        .entry(major_version)
                        .and_modify(|items| items.push(class_full_name.clone()))
                        .or_insert(vec![class_full_name.clone()]);
                }
            }
        }
    }
}

fn build_archive_from_local<P: AsRef<Path>>(path: P) -> ZipArchive<File> {
    let archive = File::open(path).unwrap();
    ZipArchive::new(archive).unwrap()
}

fn build_archive_from_url(url: &str) -> ZipArchive<io::Cursor<Vec<u8>>> {
    let mut res = reqwest::blocking::get(url).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    let _ = res.read_to_end(&mut buf);
    let reader = io::Cursor::new(buf);
    ZipArchive::new(reader).unwrap()
}

fn scan_remote_archive(
    url: &str,
    class_info: &mut HashMap<u16, u32>,
    include_details: bool,
    class_details: &mut HashMap<u16, Vec<String>>,
) {
    let mut archive = build_archive_from_url(url);
    for i in 0..archive.len() {
        let mut zip_file = archive.by_index(i).unwrap();
        if zip_file.is_file() {
            let major_version = get_major_version(&mut zip_file);
            if major_version > 0 {
                class_info
                    .entry(major_version)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
                if include_details {
                    let class_full_name = zip_file.name().to_string();
                    class_details
                        .entry(major_version)
                        .and_modify(|items| items.push(class_full_name.clone()))
                        .or_insert(vec![class_full_name.clone()]);
                }
            }
        }
    }
}

fn archive_manifest_url(url: &str) -> Option<String> {
    let mut res = reqwest::blocking::get(url).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    let _ = res.read_to_end(&mut buf);
    let reader = io::Cursor::new(buf);
    let mut archive = ZipArchive::new(reader).unwrap();
    archive
        .by_name("META-INF/MANIFEST.MF")
        .ok()
        .and_then(|mut file| {
            let mut manifest_content = String::new();
            file.read_to_string(&mut manifest_content).unwrap();
            Some(manifest_content)
        })
}

fn archive_manifest_local<P: AsRef<Path>>(path: P) -> Option<String> {
    let archive = File::open(path).unwrap();
    let mut archive = ZipArchive::new(archive).unwrap();
    archive
        .by_name("META-INF/MANIFEST.MF")
        .ok()
        .and_then(|mut file| {
            let mut manifest_content = String::new();
            file.read_to_string(&mut manifest_content).unwrap();
            Some(manifest_content)
        })
}

fn get_local_jar(group_id: &str, artifact_id: &str, version: &str) -> Option<String> {
    let m2_home = dirs::home_dir().unwrap().join(".m2");
    let gradle_home = dirs::home_dir().unwrap().join(".gradle");
    if m2_home.exists() {
        let local_m2_jar = m2_home
            .join("repository")
            .join(group_id.replace('.', "/"))
            .join(artifact_id)
            .join(version)
            .join(format!("{}-{}.jar", artifact_id, version));
        if local_m2_jar.exists() {
            return Some(format!("file://{}", local_m2_jar.display()));
        }
    }
    if gradle_home.exists() {
        let artifact_dir = gradle_home
            .join("caches")
            .join("modules-2")
            .join("files-2.1")
            .join(group_id)
            .join(artifact_id)
            .join(version);
        if artifact_dir.exists() {
            for entry in WalkDir::new(artifact_dir)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_file()
                    && entry.path().extension().unwrap_or_default() == "jar"
                {
                    return Some(format!("file://{}", entry.path().display()));
                }
            }
        }
    }
    None
}

fn resolve_jar_endpoint(command_matches: &clap::ArgMatches) -> Option<String> {
    if let Some(gav) = command_matches.get_one::<String>("gav") {
        let parts = gav.split(":").collect::<Vec<&str>>();
        let group_id = parts[0];
        let artifact = parts[1];
        let version = parts[2];
        let local_jar = get_local_jar(&group_id, artifact, version);
        return if local_jar.is_some() {
            Some(format!("file://{}", local_jar.unwrap()))
        } else {
            let url = format!(
                "https://repo1.maven.org/maven2/{}/{}/{}/{}-{}.jar",
                group_id, artifact, version, artifact, version,
            );
            Some(url)
        };
    } else if let Some(file) = command_matches.get_one::<String>("file") {
        return Some(format!("file://{}", file));
    } else if let Some(url) = command_matches.get_one::<String>("url") {
        return Some(url.clone());
    } else if let Some(directory) = command_matches.get_one::<String>("directory") {
        return Some(format!("dir://{}", directory));
    } else if command_matches.get_flag("pom") {
        return Some("pom://".to_owned());
    } else if command_matches.get_flag("gradle") {
        return Some("gradle://".to_owned());
    }
    None
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

pub fn jar_manifest_show(command_matches: &clap::ArgMatches) {
    if let Some(jar_source) = resolve_jar_endpoint(command_matches) {
        let manifest = if jar_source.starts_with("file://") {
            let local_path = jar_source.trim_start_matches("file://");
            archive_manifest_local(local_path)
        } else {
            archive_manifest_url(&jar_source)
        };
        if let Some(manifest_content) = manifest {
            println!("{}", manifest_content);
        } else {
            println!("No manifest found in the JAR file.");
        }
    } else {
        println!("No JAR source provided.");
    }
}

pub fn jar_manifest_query(command_matches: &clap::ArgMatches) {
    if let Some(jar_source) = resolve_jar_endpoint(command_matches) {
        let manifest = if jar_source.starts_with("file://") {
            let local_path = jar_source.trim_start_matches("file://");
            archive_manifest_local(local_path)
        } else {
            archive_manifest_url(&jar_source)
        };
        if let Some(manifest_content) = manifest {
            let re = Regex::new(r"\r\n\s").unwrap();
            let cleaned_content = re.replace_all(&manifest_content, "");
            // Reading simple
            let properties = java_properties::read(cleaned_content.as_bytes()).unwrap();
            let attribute_name = command_matches.get_one::<String>("attribute-name");
            if let Some(key_name) = attribute_name {
                for (key, value) in properties {
                    if key.eq_ignore_ascii_case(key_name) {
                        println!("{}: {}", key, value);
                        return;
                    }
                }
            } else {
                println!("--attribute-name should be supplied.");
            }
        } else {
            println!("No manifest found in the JAR file.");
        }
    } else {
        println!("No JAR source provided.");
    }
}

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
    use regex::Regex;
    use std::path::PathBuf;

    #[test]
    fn test_jar_extract() {
        let path = home_dir()
            .unwrap()
            .join(".m2/repository/org/apache/commons/commons-csv/1.14.0/commons-csv-1.14.0.jar");
        let mut archive = build_archive_from_local(path);
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
        let mut archive = build_archive_from_url(url);
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

    #[test]
    fn test_manifest() {
        let archive = home_dir()
            .unwrap()
            .join(".m2/repository/org/apache/commons/commons-csv/1.14.0/commons-csv-1.14.0.jar");
        let re = Regex::new(r"\r\n\s").unwrap();
        let original_content = archive_manifest_local(archive).unwrap();
        let cleaned_content = re.replace_all(&original_content, "");
        println!("{}", cleaned_content);
        // Reading simple
        let properties = java_properties::read(cleaned_content.as_bytes()).unwrap();
        for (key, value) in properties {
            println!("{}: {}", key, value);
        }
    }



    #[test]
    fn test_find_local_jar() {
        let group_id = "org.slf4j";
        let artifact_id = "slf4j-api";
        let version = "1.7.30";
        if let Some(local_jar) = get_local_jar(group_id, artifact_id, version) {
            println!("Local JAR found: {}", local_jar);
        } else {
            println!(
                "No local JAR found for {}:{}:{}",
                group_id, artifact_id, version
            );
        }
    }
}
