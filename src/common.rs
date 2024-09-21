use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use anyhow::anyhow;
use flate2::read::GzDecoder;
use reqwest::redirect::Policy;
use tar::Archive;
use zip::ZipArchive;

pub fn http_text(http_url: &str) -> String {
    let resp = reqwest::blocking::get(http_url).unwrap();
    if resp.status().is_success() {
        resp.text().unwrap()
    } else {
        "".to_owned()
    }
}

pub fn http_download(http_url: &str, target_file_path: &str) {
    oneio::download(http_url, target_file_path, None).unwrap();
}

pub fn get_redirect_url(http_url: &str) -> anyhow::Result<String> {
    let client = reqwest::blocking::Client::builder().redirect(Policy::none()).build()?;
    let response = client.get(http_url).send()?;
    // Check if the response status is a redirect
    if response.status().is_redirection() {
        if let Some(location) = response.headers().get("Location") {
            return Ok(location.to_str()?.to_string());
        }
    }
    Err(anyhow!("Failed to get redirect url: {}", http_url))
}

pub fn extract_zip<P: AsRef<Path>>(archive_file_path: P, target_dir: &PathBuf, root_excluded: bool) {
    let mut archive = ZipArchive::new(File::open(archive_file_path).unwrap()).unwrap();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        if file.is_file() {
            let enclosed_name = file.enclosed_name().unwrap();
            let mut relative_path = enclosed_name.to_str().unwrap();
            if root_excluded {  // exclude root path
                relative_path = &relative_path[(relative_path.find("/").unwrap() + 1)..];
            }
            let outpath = target_dir.join(relative_path);
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}

pub fn extract_tgz<P: AsRef<Path>>(archive_file_path: P, target_dir: &PathBuf, root_excluded: bool) {
    let tgz_file = File::open(archive_file_path).unwrap();
    let gz_decoder = GzDecoder::new(tgz_file);
    let mut archive = Archive::new(gz_decoder);
    archive
        .entries().unwrap()
        .filter_map(|e| e.ok())
        .for_each(|mut entry| {
            let entry_path = entry.path().unwrap();
            let mut relative_path = entry_path.to_str().unwrap();
            if root_excluded { // exclude root path
                relative_path = &relative_path[(relative_path.find("/").unwrap() + 1)..];
            }
            let path = target_dir.join(relative_path);
            entry.unpack(&path).unwrap();
        });
}


/// extract tgz from sub path, for example `Contents/Home/` from Mac JDK tgz
/// sub_path should end with `/`
pub fn extract_tgz_from_sub_path<P: AsRef<Path>>(archive_file_path: P, target_dir: &PathBuf, sub_path: &str) {
    let sub_path_len = sub_path.len();
    let tgz_file = File::open(archive_file_path).unwrap();
    let gz_decoder = GzDecoder::new(tgz_file);
    let mut archive = Archive::new(gz_decoder);
    archive
        .entries().unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().unwrap().to_str().unwrap().contains(sub_path))
        .for_each(|mut entry| {
            let entry_path = entry.path().unwrap();
            let mut relative_path = entry_path.to_str().unwrap();
            relative_path = &relative_path[(relative_path.find(sub_path).unwrap() + sub_path_len)..];
            let path = target_dir.join(relative_path);
            entry.unpack(&path).unwrap();
        });
}

pub fn run_command(command_name: &str, args: &[&str]) -> io::Result<Output> {
    run_command_with_env_vars(command_name, args, &None, &None, false)
}

pub fn run_command_line(command_line: &str) -> io::Result<Output> {
    let command_and_args = shlex::split(command_line).unwrap();
    let command_name = command_and_args[0].clone();
    let args = &command_and_args[1..].iter().map(AsRef::as_ref).collect::<Vec<&str>>();
    run_command(&command_name, args)
}

pub fn capture_command(command_name: &str, args: &[&str]) -> io::Result<Output> {
    run_command_with_env_vars(command_name, args, &None, &None, true)
}

pub fn run_command_with_env_vars(command_name: &str, args: &[&str],
                                 working_dir: &Option<String>,
                                 env_vars: &Option<HashMap<String, String>>,
                                 capture_output: bool) -> io::Result<Output> {
    let mut command = Command::new(command_name);
    if args.len() > 0 {
        command.args(args);
    }
    if let Some(current_dir) = working_dir {
        command.current_dir(current_dir);
    }
    if let Some(vars) = env_vars {
        for (key, value) in vars {
            command.env(key, value);
        }
    }
    if capture_output {
        command
            .envs(std::env::vars())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    } else {
        command
            .envs(std::env::vars())
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download() {
        let download_dir = dirs::download_dir().unwrap();
        let target_file_path = download_dir.join("apache-maven-3.9.9-bin.tar.gz");
        let http_url = "https://dlcdn.apache.org/maven/maven-3/3.9.9/binaries/apache-maven-3.9.9-bin.tar.gz";
        http_download(http_url, target_file_path.to_str().unwrap());
        assert!(target_file_path.exists());
    }


    #[test]
    fn test_extract_maven_zip() {
        let archive_file_path = "/Users/linux_china/temp/jdks/apache-maven-3.9.9-bin.zip";
        let target_dir = PathBuf::from("/Users/linux_china/temp/jdks/maven-3.9.9");
        extract_zip(archive_file_path, &target_dir, true);
    }

    #[test]
    fn test_extract_jdk_mac() {
        let archive_file_path = "/Users/linux_china/temp/jdks/jdk-21-mac.tgz";
        let target_dir = PathBuf::from("/Users/linux_china/temp/jdks/21");
        extract_tgz_from_sub_path(archive_file_path, &target_dir, "Contents/Home/")
    }

    #[test]
    fn test_redirect_url() {
        let download_url = "https://api.sdkman.io/2/broker/download/java/22.0.2-tem/darwinx64";
        let redirect_url = get_redirect_url(download_url).unwrap();
        let http_url = url::Url::parse(&redirect_url).unwrap();
        println!("{:?}", http_url)
    }
}
