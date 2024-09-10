pub mod config;
pub mod jdk;


use std::path::PathBuf;

pub fn jbang_home() -> PathBuf {
    if let Ok(jbang_home) = std::env::var("JBANG_DIR") {
        PathBuf::from(jbang_home)
    } else {
        dirs::home_dir().unwrap().join(".jbang")
    }
}

pub fn jbang_exec() -> PathBuf {
    let jbang_bin_dir = jbang_home().join("bin");
    if cfg!(windows) {
        jbang_bin_dir.join("jbang.cmd")
    } else {
        jbang_bin_dir.join("jbang")
    }
}
