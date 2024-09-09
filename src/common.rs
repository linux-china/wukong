use std::fs::File;
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
        println!("{}",dest_dir.to_str().unwrap());
    }
}
