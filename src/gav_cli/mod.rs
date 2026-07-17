use std::env;
use std::process::{Command, Stdio};

pub mod clap_app;

pub fn gav_add(command_matches: &clap::ArgMatches) {
    let mut gav_params: Vec<String> = vec![];
    // gav
    if let Some(gav_param) = command_matches.get_one::<String>("gav") {
        gav_params.push(format!("-Dgav={}", gav_param));
    }
    // profile
    if let Some(profile_param) = command_matches.get_one::<String>("profile") {
        gav_params.push(format!("-Dprofile={}", profile_param));
    }
    // project
    if let Some(project_param) = command_matches.get_one::<String>("project") {
        gav_params.push("-pl".to_owned());
        gav_params.push(project_param.to_owned());
    }
    // optional
    if command_matches.get_flag("optional") {
        gav_params.push("-Doptional=true".to_owned());
    }

    let mut scope = "".to_owned();
    if let Some(scope_param) = command_matches.get_one::<String>("scope") {
        if !scope.is_empty() {
            scope = scope_param.to_owned();
        }
    }
    // bom or managed(<dependencyManagement>)
    if command_matches.get_flag("bom") {
        gav_params.push("-Dtype=pom".to_owned());
        gav_params.push("-Dscope=import".to_owned());
        gav_params.push("-Dmanaged".to_owned());
    } else if command_matches.get_flag("managed") {
        gav_params.push("-Dmanaged".to_owned());
    }
    // scope
    if command_matches.get_flag("test") {
        scope = "test".to_owned();
    } else if command_matches.get_flag("runtime") {
        scope = "runtime".to_owned();
    }
    if scope != "" {
        gav_params.push(format!("-Dscope={}", scope));
    }
    if let Some(type_param) = command_matches.get_one::<String>("type") {
        gav_params.push(format!("-Dtype={}", type_param));
    }
    if let Some(type_param) = command_matches.get_one::<String>("classifier") {
        gav_params.push(format!("-Dclassifier={}", type_param));
    }
    run_command("add", &gav_params);
}

pub fn gav_remove(command_matches: &clap::ArgMatches) {
    let mut gav_params: Vec<String> = vec![];
    // gav
    if let Some(gav_param) = command_matches.get_one::<String>("gav") {
        gav_params.push(format!("-Dgav={}", gav_param));
    }
    // profile
    if let Some(profile_param) = command_matches.get_one::<String>("profile") {
        gav_params.push(format!("-Dprofile={}", profile_param));
    }
    // project
    if let Some(project_param) = command_matches.get_one::<String>("project") {
        gav_params.push("-pl".to_owned());
        gav_params.push(project_param.to_owned());
    }
    if command_matches.get_flag("bom") {
        gav_params.push("-Dtype=pom".to_owned());
        gav_params.push("-Dscope=import".to_owned());
        gav_params.push("-Dmanaged".to_owned());
    } else if command_matches.get_flag("managed") {
        gav_params.push("-Dmanaged".to_owned());
    }
    if let Some(type_param) = command_matches.get_one::<String>("type") {
        gav_params.push(format!("-Dtype={}", type_param));
    }
    if let Some(type_param) = command_matches.get_one::<String>("classifier") {
        gav_params.push(format!("-Dclassifier={}", type_param));
    }
    run_command("remove", &gav_params);
}

pub fn dependency_tree(command_matches: &clap::ArgMatches) {
    run_mvn_command(&["dependency:3.11.0:tree".to_string()])
}

fn get_mvn_command() -> &'static str {
    if cfg!(target_os = "windows") {
        let wrapper_available = env::current_dir()
            .map(|dir| dir.join("mvnw.cmd").exists())
            .unwrap_or(false);
        return if wrapper_available {
            ".\\mvnw.cmd"
        } else {
            "mvn"
        };
    }
    let wrapper_available = env::current_dir()
        .map(|dir| dir.join("mvnw").exists())
        .unwrap_or(false);
    if wrapper_available { "./mvnw" } else { "mvn" }
}

fn run_command(sub_command: &str, misc_args: &[String]) {
    let mut gav_args = vec![format!("dependency:3.11.0:{}", sub_command)];
    gav_args.extend_from_slice(&misc_args);
    run_mvn_command(&gav_args)
}

fn run_mvn_command(mvn_args: &[String]) {
    // construct command
    let mvn_command = get_mvn_command();
    let mut command = Command::new(mvn_command);
    //println!("{} {}", mvn_command, mvn_args.join(" "));
    command
        .envs(env::vars())
        .args(mvn_args)
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
