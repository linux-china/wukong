use read_url::*;
use std::io::Read;
use std::path::PathBuf;

#[test]
fn test_download() {
    println!("Hello OneIO!");
}

#[test]
fn test_url_read() {
    let context = UrlContext::new();
    let url = context.file_url(PathBuf::from("Cargo.toml"), None, None, None);
    dump(&url).unwrap();
}

pub fn dump(url: &UrlRef) -> Result<(), UrlError> {
    let mut reader = url.open()?; // io::Read
    let mut string = String::new();
    reader.read_to_string(&mut string)?;
    println!("    {:10}{:?}", "Content:", string);
    Ok(())
}
