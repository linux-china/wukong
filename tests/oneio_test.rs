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

#[test]
fn test_read_url() {
    let context = UrlContext::new();
    let url = context.url("https://httpbin.org/ip").unwrap();
    dump(&url).unwrap();
}
#[test]
fn test_read_git() {
    let context = UrlContext::new();
    let url = context.url("git:https://github.com/linux-china/task-keeper.git!/justfile").unwrap();
    dump(&url).unwrap();
}

#[test]
fn test_read_jar() {
    let context = UrlContext::new();
    let url = context.url("zip:https://repo1.maven.org/maven2/commons-io/commons-io/2.18.0/commons-io-2.18.0.jar!/META-INF/maven/commons-io/commons-io/pom.xml").unwrap();
    dump(&url).unwrap();
}

pub fn dump(url: &UrlRef) -> Result<(), UrlError> {
    let mut reader = url.open()?; // io::Read
    let mut string = String::new();
    reader.read_to_string(&mut string)?;
    println!("    {:10}{:?}", "Content:", string);
    Ok(())
}
