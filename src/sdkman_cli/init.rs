use crate::sdkman_cli::sdkman_home;

pub fn shell_hook() {
    let sdkman_home = sdkman_home();
    let mut paths: Vec<String> = vec![];
    let candidates_dir = sdkman_home.join("candidates");
    candidates_dir.read_dir().unwrap().for_each(|entry| {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            let candidate_name = entry.file_name().into_string().unwrap();
            let candidate_home = entry.path();
            let candidate_current_link = candidate_home.join("current");
            if candidate_current_link.exists() && candidate_current_link.is_symlink() {
                println!("export {}_HOME={}", candidate_name.to_uppercase(), candidate_home.to_str().unwrap());
                let candidate_bin_path = candidate_current_link.join("bin");
                if candidate_bin_path.exists() {
                    paths.push(candidate_bin_path.to_str().unwrap().to_string());
                } else {
                    paths.push(candidate_home.to_str().unwrap().to_string());
                }
            }
        }
    });
    if !paths.is_empty() {
        println!("export PATH={}:$PATH", paths.join(":"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        shell_hook();
    }
}
