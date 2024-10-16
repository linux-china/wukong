pub fn manage_completion(_completion_matches: &clap::ArgMatches) {
    if let Ok(shell_name) = std::env::var("SHELL") {
        if shell_name.contains("zsh") || shell_name.contains("bash") {
            println!("{}", include_str!("completion/jbang.sh"));
            return;
        }
    }
}
