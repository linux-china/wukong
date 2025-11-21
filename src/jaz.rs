use dotenvx_rs::dotenvx;
use std::env;
use std::process::{Command, Stdio};

fn main() {
    let jaz_args = env::args().collect::<Vec<_>>();
    let mut java_args: Vec<String> = vec![];
    java_args.extend(jaz_args[1..].iter().map(|s| s.to_string()));
    // .env load with dotenvx
    dotenvx::dotenv().ok();
    // construct command
    let mut command = Command::new("java");
    command
        .envs(env::vars())
        .args(&java_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    let mut child = command.spawn().unwrap();
    let status = child.wait().unwrap();
    let exit_code = if let Some(code) = status.code() {
        code
    } else {
        // On Unix, process was terminated by signal
        #[cfg(unix)]
        {
            use std::os::unix::process::ExitStatusExt;
            if let Some(signal) = status.signal() {
                std::process::exit(128 + signal);
            }
        }
        1
    };
    std::process::exit(exit_code);
}
