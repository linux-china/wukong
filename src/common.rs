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
