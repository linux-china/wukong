use colored::Colorize;
use itertools::Itertools;
use pad::PadStr;
use prettytable::{row, Table};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
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

fn get_bytecode_version(java_version: f32) -> u16 {
    match java_version {
        1.1 => 45,
        1.2 => 46,
        1.3 => 47,
        1.4 => 48,
        1.5 => 49,
        5.0 => 49,
        6.0 => 50,
        1.6 => 50,
        7.0 => 51,
        1.7 => 51,
        8.0 => 52,
        1.8 => 52,
        9.0 => 53,
        10.0 => 54,
        11.0 => 55,
        12.0 => 56,
        13.0 => 57,
        14.0 => 58,
        15.0 => 59,
        16.0 => 60,
        17.0 => 61,
        18.0 => 62,
        19.0 => 63,
        20.0 => 64,
        21.0 => 65,
        22.0 => 66,
        23.0 => 67,
        24.0 => 68,
        25.0 => 69,
        26.0 => 70,
        _ => (java_version - 44.0) as u16,
    }
}

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

pub fn bytecode(command_matches: &clap::ArgMatches) {
    if let Some((command, command_matches)) = command_matches.subcommand() {
        match command {
            "show" => bytecode_show(command_matches),
            &_ => println!("Unknown command"),
        }
    }
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

fn resolve_jar_source(command_matches: &clap::ArgMatches) -> Option<String> {
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
    }
    None
}

fn resolve_pom_dependencies(output: &str) -> HashSet<String> {
    let start_placeholder = "--- dependency:";
    let end_placeholder = "------";
    let mut dependencies: HashSet<String> = HashSet::new();
    let mut in_dependencies_section = false;
    for line in output.lines() {
        if line.contains(start_placeholder) && line.contains("tree") {
            in_dependencies_section = true;
            continue;
        }
        if line.contains(end_placeholder) {
            in_dependencies_section = false;
            continue;
        }
        if in_dependencies_section {
            let mut trimmed_line = line.trim();
            if trimmed_line.ends_with("(optional)") {
                trimmed_line = &trimmed_line[..trimmed_line.len() - 10].trim();
            }
            if let Some(pos) = trimmed_line.rfind(" ") {
                trimmed_line = &trimmed_line[pos + 1..];
            }
            if trimmed_line.contains(":") {
                dependencies.insert(trimmed_line.to_string());
            }
        }
    }
    dependencies
}

fn resolve_gradle_dependencies(output: &str) -> HashSet<String> {
    let start_placeholder = "compileClasspath";
    let mut dependencies: HashSet<String> = HashSet::new();
    let mut in_dependencies_section = false;
    for line in output.lines() {
        if line.starts_with(start_placeholder) || line.starts_with("runtimeClasspath") {
            in_dependencies_section = true;
            continue;
        }
        if line.trim().is_empty() {
            in_dependencies_section = false;
            continue;
        }
        if in_dependencies_section {
            let mut trimmed_line = line.trim();
            if trimmed_line.ends_with(")") {
                let offset = trimmed_line.rfind("(").unwrap();
                trimmed_line = &trimmed_line[..offset].trim();
            }
            if let Some(pos) = trimmed_line.rfind(" ") {
                trimmed_line = &trimmed_line[pos + 1..];
            }
            if trimmed_line.contains(":") {
                dependencies.insert(trimmed_line.to_string());
            }
        }
    }
    dependencies
}

pub fn bytecode_show(command_matches: &clap::ArgMatches) {
    let mut class_info = HashMap::<u16, u32>::new();
    let include_details: bool = command_matches.get_flag("details");
    let mut class_details = HashMap::<u16, Vec<String>>::new();
    let mut default_bytecode_version = command_matches
        .get_one::<String>("java-version")
        .map(|s| s.parse::<u16>().unwrap_or(0))
        .unwrap_or(0);
    if let Some(java_version) = command_matches.get_one::<String>("java-version") {
        if let Ok(version) = java_version.parse::<f32>() {
            default_bytecode_version = get_bytecode_version(version);
        }
    }
    if let Some(jar_source) = resolve_jar_source(command_matches) {
        if jar_source.starts_with("file://") {
            let local_path = jar_source.trim_start_matches("file://");
            scan_local_archive(
                local_path,
                &mut class_info,
                include_details,
                &mut class_details,
            );
        } else if jar_source.starts_with("dir://") {
            let directory = jar_source.trim_start_matches("dir://");
            for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    let path = entry.path();
                    if path.to_str().unwrap().ends_with(".jar") {
                        scan_local_archive(
                            path,
                            &mut class_info,
                            include_details,
                            &mut class_details,
                        );
                    }
                }
            }
        } else {
            scan_remote_archive(
                &jar_source,
                &mut class_info,
                include_details,
                &mut class_details,
            );
        }
    }
    if class_info.is_empty() {
        println!("No class files found in the provided input.");
        return;
    }
    let mut table = Table::new();
    table.add_row(row!["Major Version", "Java Version", "Class Count"]);
    // iterate through the class_info map and print the major version and count
    for (major_version, count) in class_info.iter().sorted_by_key(|x| x.1) {
        let java_version = get_java_version(*major_version);
        table.add_row(row![
            major_version.to_string(),
            java_version,
            count.to_string()
        ]);
    }
    let default_format = "text".to_owned();
    let output_format = command_matches
        .get_one::<String>("output-format")
        .unwrap_or(&default_format);
    if output_format == "csv" {
        table.to_csv(io::stdout()).unwrap();
    } else {
        table.printstd();
        if include_details {
            if default_bytecode_version > 0 {
                println!(
                    "\nClasses with major version {} (Java {}):",
                    default_bytecode_version,
                    get_java_version(default_bytecode_version)
                );
                if let Some(classes) = class_details.get(&default_bytecode_version) {
                    for class_full_name in classes {
                        println!("  - {}", class_full_name);
                    }
                } else {
                    println!(
                        "  No classes found for major version:{}",
                        default_bytecode_version
                    );
                }
            } else {
                for (major_version, classes) in class_details.iter().sorted_by_key(|x| x.1) {
                    println!(
                        "### {}(Java {})",
                        major_version,
                        get_java_version(*major_version)
                    );
                    for class_full_name in classes {
                        println!("  - {}", class_full_name);
                    }
                }
            }
        }
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

pub fn jar_manifest_show(command_matches: &clap::ArgMatches) {
    if let Some(jar_source) = resolve_jar_source(command_matches) {
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
    if let Some(jar_source) = resolve_jar_source(command_matches) {
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
    fn test_bytecode() {
        let mcs_app = build_jarviz_app();
        let mcs_matches = mcs_app.get_matches_from(&vec!["mcs", "class-search", "VelocityEngine"]);
        let class_search_matches = mcs_matches.subcommand_matches("class-search").unwrap();
        bytecode(class_search_matches);
    }

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
    fn test_resolve_pom_dependencies() {
        let path = PathBuf::from("tests/dependencies/maven-dependencies.txt");
        let output = std::fs::read_to_string(path).unwrap();
        let dependencies = resolve_pom_dependencies(&output);
        for dependency in dependencies {
            println!("{}", dependency);
        }
    }

    #[test]
    fn test_resolve_gradle_dependencies() {
        let path = PathBuf::from("tests/dependencies/gradle-dependencies.txt");
        let output = std::fs::read_to_string(path).unwrap();
        let dependencies = resolve_gradle_dependencies(&output);
        for dependency in dependencies {
            println!("{}", dependency);
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
