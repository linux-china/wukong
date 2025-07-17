use crate::jarviz_cli::{resolve_jar_source, scan_local_archive, scan_remote_archive};
use itertools::Itertools;
use prettytable::format::Alignment;
use prettytable::{row, Cell, Row, Table};
use std::collections::{HashMap, HashSet};
use std::io;
use std::io::Read;
use walkdir::WalkDir;

pub fn get_major_version(zip_file: &mut dyn Read) -> u16 {
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

pub fn get_java_version(major_version: u16) -> &'static str {
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

pub fn bytecode(command_matches: &clap::ArgMatches) {
    if let Some((command, command_matches)) = command_matches.subcommand() {
        match command {
            "show" => bytecode_show(command_matches),
            "matrix" => bytecode_matrix(command_matches),
            &_ => println!("Unknown command"),
        }
    }
}

pub fn bytecode_matrix(_: &clap::ArgMatches) {
    let mut table = Table::new();
    table.add_row(row!["Bytecode Version", "Java Version"]);
    for major_version in 45..70 {
        let java_version = get_java_version(major_version);
        table.add_row(row![major_version.to_string(), java_version]);
    }
    table.printstd();
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
    let jar_source = resolve_jar_source(command_matches);
    if let Some(jar_url) = &jar_source {
        if jar_url.starts_with("file://") {
            let local_path = jar_url.trim_start_matches("file://");
            scan_local_archive(
                local_path,
                &mut class_info,
                include_details,
                &mut class_details,
            );
        } else if jar_url.starts_with("dir://") {
            let directory = jar_url.trim_start_matches("dir://");
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
                &jar_url,
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
        let jar_url = jar_source.unwrap();
        if jar_url.ends_with(".jar") {
            let jar_name = jar_url.split("/").last().unwrap().to_string();
            table.set_titles(Row::new(vec![Cell::new_align(
                &jar_name,
                Alignment::CENTER,
            )
            .with_hspan(3)]));
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jarviz_cli::clap_app::build_jarviz_app;

    #[test]
    fn test_bytecode() {
        let jarviz_app = build_jarviz_app();
        let matrix_matches = jarviz_app.get_matches_from(&vec!["bytecode", "matrix"]);
        bytecode(&matrix_matches);
    }
}
