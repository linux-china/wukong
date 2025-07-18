use crate::jarviz_cli::bytecode::{resolve_gradle_dependencies, resolve_pom_dependencies};
use crate::jarviz_cli::{
    build_archive_from_local, build_archive_from_url, get_local_jar, get_output_format,
    resolve_jar_endpoint,
};
use std::collections::HashSet;
use std::io::Read;
use std::path::Path;
use walkdir::WalkDir;

pub fn services(command_matches: &clap::ArgMatches) {
    if let Some((command, command_matches)) = command_matches.subcommand() {
        match command {
            "list" => services_list(command_matches),
            &_ => println!("Unknown command"),
        }
    }
}

pub fn services_list(command_matches: &clap::ArgMatches) {
    let service_name = command_matches.get_one::<String>("service-name");
    let mut services: HashSet<String> = HashSet::new();
    if let Some(jar_source) = resolve_jar_endpoint(command_matches) {
        if jar_source.starts_with("file://") {
            services.extend(scan_services(&jar_source, service_name));
        } else if jar_source.starts_with("https://") || jar_source.starts_with("http://") {
            services.extend(scan_services(&jar_source, service_name));
        } else if jar_source.starts_with("dir://") {
            let dir_path = jar_source.trim_start_matches("dir://");
            for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file()
                    && entry.path().extension().unwrap_or_default() == "jar"
                {
                    services.extend(scan_services(&entry.path().to_str().unwrap(), service_name));
                }
            }
        } else if jar_source.starts_with("classpath://") {
            let classpath = jar_source.trim_start_matches("classpath://");
            for path in classpath.split(':') {
                if path.ends_with(".jar") {
                    services.extend(scan_services(path, service_name));
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
                    services.extend(scan_services(&jar_path, service_name));
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
                    services.extend(scan_services(&jar_path, service_name));
                }
            }
        } else {
            println!("Unknown JAR source: {}", jar_source);
        }
    } else {
        println!("No JAR source provided.");
    }
    if services.is_empty() {
        println!("No SPI found in the specified JAR.");
    } else {
        let output_format = get_output_format(command_matches);
        if output_format == "csv" {
            println!("jar_name,service_name,service_impl");
            for service in services {
                println!("{}", service);
            }
        } else {
            for service in services {
                println!("{}", service);
            }
        }
    }
}

fn scan_services(jar_path_or_url: &str, required_service_name: Option<&String>) -> HashSet<String> {
    let file_name = jar_path_or_url.split('/').last().unwrap_or("");
    let mut services: HashSet<String> = HashSet::new();
    if jar_path_or_url.starts_with("https://") || jar_path_or_url.starts_with("http://") {
        let mut archive = build_archive_from_url(jar_path_or_url);
        for i in 0..archive.len() {
            let mut zip_file = archive.by_index(i).unwrap();
            if zip_file.is_file() && zip_file.name().starts_with("META-INF/services/") {
                let service_name = zip_file
                    .name()
                    .trim_start_matches("META-INF/services/")
                    .to_string();
                if let Some(name) = required_service_name {
                    if name != &service_name {
                        continue;
                    }
                }
                let mut content = String::new();
                zip_file.read_to_string(&mut content).unwrap();
                // split content by newlines and insert each service
                for line in content.lines() {
                    let service_impl = line.trim();
                    if !service_impl.is_empty() && !service_impl.starts_with('#') {
                        services.insert(format!("{},{},{}", file_name, service_name, service_impl));
                    }
                }
            }
        }
    } else {
        let local_path = jar_path_or_url.trim_start_matches("file://");
        if Path::new(local_path).exists() {
            let mut archive = build_archive_from_local(local_path);
            for i in 0..archive.len() {
                let mut zip_file = archive.by_index(i).unwrap();
                if zip_file.is_file() && zip_file.name().starts_with("META-INF/services/") {
                    let service_name = zip_file
                        .name()
                        .trim_start_matches("META-INF/services/")
                        .to_string();
                    // read zip_file content to string
                    let mut content = String::new();
                    zip_file.read_to_string(&mut content).unwrap();
                    // split content by newlines and insert each service
                    for line in content.lines() {
                        let service_impl = line.trim();
                        if !service_impl.is_empty() && !service_impl.starts_with('#') {
                            services
                                .insert(format!("{},{},{}", file_name, service_name, service_impl));
                        }
                    }
                }
            }
        }
    }
    services
}
