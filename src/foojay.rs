use std::os;

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
       os =  "linux";
    } else if cfg!(target_os = "macos") {
        os = "mac";
        libc_type = "libc";
    } else if cfg!(target_os = "windows") {
        os= "windows";
        libc_type = "c_std_lib";
        file_type = "zip";
    } else {
        panic!("Unsupported OS");
    };
    format!("https://api.foojay.io/disco/v3.0/directuris?distro={distro}&javafx_bundled=false&libc_type={libc_type}&archive_type={file_type}&operating_system={os}&package_type=jdk&version={java_version}&architecture={arch}&latest=available")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_download_url() {
        let java_version = "21" ;
        println!("{}", get_jdk_download_url(java_version));
    }
}
