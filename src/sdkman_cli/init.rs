use crate::sdkman_cli::{list_candidate_names, sdkman_home};

pub fn shell_hook() {
    let candidates_dir_home = sdkman_home().join("candidates");
    let mut paths: Vec<String> = vec![];
    let candidate_names = list_candidate_names();
    for candidate_name in &candidate_names {
        let candidate_home = candidates_dir_home.join(candidate_name);
        let candidate_current_link = candidate_home.join("current");
        if candidate_current_link.exists() && candidate_current_link.is_symlink() {
            println!("export {}_HOME={}", candidate_name.to_uppercase(), candidate_current_link.to_str().unwrap());
            let candidate_bin_path = candidate_current_link.join("bin");
            if candidate_bin_path.exists() {
                paths.push(candidate_bin_path.to_str().unwrap().to_string());
            } else {
                paths.push(candidate_home.to_str().unwrap().to_string());
            }
        }
    }
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
