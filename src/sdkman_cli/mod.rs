pub mod list;

const SDKMAN_CANDIDATES_API: &str = "https://api.sdkman.io/2";
pub fn get_sdkman_platform() -> String {
    let os_name = match std::env::consts::OS {
        "macos" => "darwin",
        "windows" => "windows",
        "linux" => "linux",
        &_ => ""
    };
    let arch_name = match std::env::consts::ARCH {
        "aarch64" => "arm64",
        "x86_64" => "x64",
        &_ => "exotic"
    };
    if arch_name == "exotic" {
        "exotic".to_owned()
    } else {
        format!("{}{}", os_name, arch_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdkman_platform() {
        println!("{}", get_sdkman_platform());
    }
}
