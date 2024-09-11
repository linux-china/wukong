use std::fs::File;
use oneio::download;
use tar::Archive;
use crate::jbang_cli::jbang_home;

pub fn upgrade_jbang() {
    println!("Upgrading jbang...");
    install_jbang();
}

pub fn install_jbang() {
    let download_url = "https://github.com/jbangdev/jbang/releases/latest/download/jbang.tar";
    let temp_dir = std::env::temp_dir();
    let target_file_path = temp_dir.join("jbang.tar");
    download(&download_url, target_file_path.to_str().unwrap(), None).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upgrade_jbang() {
        upgrade_jbang();
    }
}
