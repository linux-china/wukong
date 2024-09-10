use std::fs::File;
use std::os;
use std::path::PathBuf;
use flate2::read::GzDecoder;
use oneio::download;
use tar::Archive;
use zip::ZipArchive;
use crate::common::unpack_tgz;

pub fn get_jdk_download_url(java_version: &str) -> String {
    let distro = "temurin";
    let mut os = "linux";
    let arch = match std::env::consts::ARCH {
        "x86_64" => "x64",
        "aarch64" => "aarch64",
        _ => panic!("Unsupported architecture"),
    };
    let mut file_type = "tar.gz";
    let mut libc_type = "glibc";
    if cfg!(target_os = "linux") {
        os = "linux";
    } else if cfg!(target_os = "macos") {
        os = "mac";
        libc_type = "libc";
    } else if cfg!(target_os = "windows") {
        os = "windows";
        libc_type = "c_std_lib";
        file_type = "zip";
    } else {
        panic!("Unsupported OS");
    };
    format!("https://api.foojay.io/disco/v3.0/directuris?distro={distro}&javafx_bundled=false&libc_type={libc_type}&archive_type={file_type}&operating_system={os}&package_type=jdk&version={java_version}&architecture={arch}&latest=available")
}

pub fn extract_jdk(java_version: &str, target_dir: &PathBuf) {
    let download_url = get_jdk_download_url(java_version);
    let temp_dir = std::env::temp_dir();
    let mut archive_file_name = format!("jdk-{}.tar.gz", java_version);
    if cfg!(target_os = "windows") {
        archive_file_name = format!("jdk-{}.zip", java_version);
    }
    let target_file_path = temp_dir.join(archive_file_name);
    download(&download_url, target_file_path.to_str().unwrap(), None).unwrap();
    if cfg!(target_family = "windows") {
        let mut archive = ZipArchive::new(File::open(&target_file_path).unwrap()).unwrap();
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            if file.is_file() {
                let enclosed_name = file.enclosed_name().unwrap();
                let mut relative_path = enclosed_name.to_str().unwrap();
                relative_path = &relative_path[(relative_path.find("/").unwrap() + 1)..];
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
    } else {
        let tgz_file = File::open(&target_file_path).unwrap();
        let gz_decoder = GzDecoder::new(tgz_file);
        let mut archive = Archive::new(gz_decoder);
        if cfg!(target_os = "macos") {
            archive
                .entries().unwrap()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().unwrap().to_str().unwrap().contains("Contents/Home"))
                .for_each(|mut entry| {
                    let entry_path = entry.path().unwrap();
                    let mut relative_path = entry_path.to_str().unwrap();
                    relative_path = &relative_path[(relative_path.find("Contents/Home").unwrap() + 14)..];
                    let path = target_dir.join(relative_path);
                    entry.unpack(&path).unwrap();
                });
        } else {
            archive
                .entries().unwrap()
                .filter_map(|e| e.ok())
                .for_each(|mut entry| {
                    let entry_path = entry.path().unwrap();
                    let mut relative_path = entry_path.to_str().unwrap();
                    relative_path = &relative_path[(relative_path.find("/").unwrap() + 1)..];
                    let path = target_dir.join(relative_path);
                    entry.unpack(&path).unwrap();
                });
        }
    }
    std::fs::remove_file(&target_file_path).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_download_url() {
        let java_version = "21";
        println!("{}", get_jdk_download_url(java_version));
    }
}
