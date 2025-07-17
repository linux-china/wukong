use crate::jarviz_cli::bytecode::{resolve_gradle_dependencies, resolve_pom_dependencies};
use crate::jarviz_cli::{
    build_archive_from_local, build_archive_from_url, get_local_jar, get_output_format,
    resolve_jar_endpoint,
};
use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;

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

fn scan_classes(jar_path_or_url: &str) -> HashSet<String> {
    let file_name = jar_path_or_url.split('/').last().unwrap_or("");
    let mut classes: HashSet<String> = HashSet::new();
    if jar_path_or_url.starts_with("https://") || jar_path_or_url.starts_with("http://") {
        let mut archive = build_archive_from_url(jar_path_or_url);
        for i in 0..archive.len() {
            let mut zip_file = archive.by_index(i).unwrap();
            if zip_file.is_file() && zip_file.name().ends_with(".class") {
                let class_name = zip_file.name().to_string();
                classes.insert(format!("{},{}", file_name, class_name));
            }
        }
    } else {
        let local_path = jar_path_or_url.trim_start_matches("file://");
        if Path::new(local_path).exists() {
            let mut archive = build_archive_from_local(local_path);
            for i in 0..archive.len() {
                let mut zip_file = archive.by_index(i).unwrap();
                if zip_file.is_file() && zip_file.name().ends_with(".class") {
                    let class_name = zip_file.name().to_string();
                    classes.insert(format!("{},{}", file_name, class_name));
                }
            }
        }
    }
    classes
}

pub fn entries_find(command_matches: &clap::ArgMatches) {
    let mut classes: HashSet<String> = HashSet::new();
    if let Some(jar_source) = resolve_jar_endpoint(command_matches) {
        if jar_source.starts_with("file://") {
            classes.extend(scan_classes(&jar_source));
        } else if jar_source.starts_with("https://") || jar_source.starts_with("http://") {
            classes.extend(scan_classes(&jar_source));
        } else if jar_source.starts_with("dir://") {
            let dir_path = jar_source.trim_start_matches("dir://");
            for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file()
                    && entry.path().extension().unwrap_or_default() == "jar"
                {
                    classes.extend(scan_classes(&entry.path().to_str().unwrap()));
                }
            }
        } else if jar_source.starts_with("classpath://") {
            let classpath = jar_source.trim_start_matches("classpath://");
            for path in classpath.split(':') {
                if path.ends_with(".jar") {
                    classes.extend(scan_classes(path));
                }
            }
        } else if jar_source.starts_with("pom://") {
            let dependencies = resolve_pom_dependencies("");
            for dep in dependencies {
                let parts = dep.split(':').collect::<Vec<&str>>();
                let group_id = parts.get(0).unwrap();
                let artifact = parts.get(1).unwrap();
                let version = parts.get(3).unwrap();
                let jar_path = get_local_jar(group_id, artifact, version);
                if let Some(jar_path) = jar_path {
                    println!("jar_path: {}", jar_path);
                    classes.extend(scan_classes(&jar_path));
                }
            }
        } else if jar_source.starts_with("gradle://") {
            let dependencies = resolve_gradle_dependencies("");
            for dep in dependencies {
                let parts = dep.split(':').collect::<Vec<&str>>();
                let group_id = parts.get(0).unwrap();
                let artifact = parts.get(1).unwrap();
                let version = parts.get(2).unwrap();
                let jar_path = get_local_jar(group_id, artifact, version);
                if let Some(jar_path) = jar_path {
                    classes.extend(scan_classes(&jar_path));
                }
            }
        } else {
            println!("Unknown JAR source: {}", jar_source);
        }
    } else {
        println!("No JAR source provided.");
    }
    if classes.is_empty() {
        println!("No classes found in the specified JAR.");
    } else {
        let output_format = get_output_format(command_matches);
        if output_format == "csv" {
            println!("jar_name,package_name,class_full_name,class_name");
            for class in classes {
                let parts: Vec<&str> = class.split(',').collect();
                let jar_name = parts.get(0).unwrap();
                let class_full_name = parts.get(1).unwrap();
                let class_full_name = class_full_name.trim_end_matches(".class").replace('/', ".");
                let mut class_name = class_full_name.as_str();
                let package_name = if let Some(pos) = class_full_name.rfind('.') {
                    class_name = &class_full_name[pos + 1..];
                    class_full_name[..pos].to_string()
                } else {
                    "".to_string()
                };
                if !(class_name == "package-info" || class_name == "module-info") {
                    println!(
                        "{},{},{},{}",
                        jar_name, package_name, class_full_name, class_name
                    );
                }
            }
        } else {
            for class in classes {
                println!("{}", class);
            }
        }
    }
}
