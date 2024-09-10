use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::process::{Command, Output, Stdio};
use flate2::read::GzDecoder;
use tar::Archive;

fn http_download(http_url: &str, target_file_path: &str) {
    let http_url = http_url;
    oneio::download(http_url, target_file_path, None).unwrap();
}

fn unpack_tgz(tgz_file: &str, target_dir: &str) {
    let tar_gz = File::open(tgz_file).unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(target_dir).unwrap();
}

pub fn run_command(command_name: &str, args: &[&str]) -> io::Result<Output> {
    run_command_with_env_vars(command_name, args, &None, &None)
}

pub fn run_command_with_env_vars(command_name: &str, args: &[&str], working_dir: &Option<String>, env_vars: &Option<HashMap<String, String>>) -> io::Result<Output> {
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
     command
        .envs(std::env::vars())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
}

#[cfg(test)]
mod tests {
    use std::env;
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
    fn test_unpack_tgz() {
        let dir = env::temp_dir();
        let download_dir = dirs::download_dir().unwrap();
        let target_file_path = download_dir.join("apache-maven-3.9.9-bin.tar.gz");
        let dest_dir = dir.join("maven-20240910");
        unpack_tgz(target_file_path.to_str().unwrap(), dest_dir.to_str().unwrap());
        println!("{}", dest_dir.to_str().unwrap());
    }
}
