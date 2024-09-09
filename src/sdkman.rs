mod common;

use clap::{Command, Arg};

pub const VERSION: &str = "0.1.0";

fn main() {
    let app = build_sdkman_app();
    let _matches = app.get_matches();
}

pub fn build_sdkman_app() -> Command {
    let install_command = Command::new("install")
        .about("install a candidate version.")
        .long_about(r#"Invoking this subcommand with only the candidate as parameter will install the currently known default version for that candidate. Provide a second qualifier to install a specific non-default version. Provide a third optional qualifier to add an already installed local version. This final qualifier is the absolute local path to the base directory of the SDK to be added. The local version will appear as an installed version of the candidate. The version may not conflict with an existing version, installed or not."#)
        .arg(
            Arg::new("candidate")
                .help("candidate name")
                .index(1)
                .required(true)
        )
        .arg(
            Arg::new("version")
                .help("candidate version.")
                .required(false)
                .index(2)
                .num_args(1)
        )
        .arg(
            Arg::new("path")
                .help("install path.")
                .required(false)
                .index(3)
                .num_args(1)
        );
    let uninstall_command = Command::new("uninstall")
        .about("uninstall a candidate version.")
        .long_about(r#"Always follow the subcommand with two qualifiers, the candidate and version to be uninstalled. The specified version will be removed from the corresponding candidate directory under $SDKMAN_DIR/candidates and will no longer be available for use on the system."#)
        .arg(
            Arg::new("candidate")
                .help("candidate name")
                .index(1)
                .required(true)
        )
        .arg(
            Arg::new("version")
                .help("candidate version.")
                .required(false)
                .index(2)
                .num_args(1)
        );
    let list_command = Command::new("list")
        .about("list all candidates or candidate versions.")
        .long_about(r#"Invoke the subcommand without a candidate to see a comprehensive list of all
candidates with name, URL, detailed description and an installation command.
If the candidate qualifier is specified, the subcommand will display a list
of all available and local versions for that candidate. In addition, the
version list view marks all versions that are local, installed or currently
in use. They appear as follows:

+ - local version
* - installed
> - currently in use

Java has a custom list view with vendor-specific details. "#)
        .arg(
            Arg::new("candidate")
                .help("candidate name")
                .index(1)
                .required(true)
        );
    let use_command = Command::new("use")
        .about("use a specific version only in the current shell.")
        .long_about(r#"The mandatory candidate and version follow the subcommand to specify what to use in the shell. This subcommand only operates on the current shell. It does not affect other shells running different versions of the same candidate. It also does not change the default version set for all subsequent shells."#)
        .arg(
            Arg::new("candidate")
                .help("candidate name")
                .index(1)
                .required(true)
        )
        .arg(
            Arg::new("version")
                .help("candidate version.")
                .required(false)
                .index(2)
                .num_args(1)
        );
    Command::new("sdk")
        .version(VERSION)
        .about("sdk - The command line interface (CLI) for SDKMAN!")
        .long_about(r#"SDKMAN! is a tool for managing parallel versions of multiple JVM related Software Development Kits on most Unix based systems. It provides a convenient Command Line Interface (CLI) and API for installing, switching, removing and listing Candidates."#)
        .subcommand(install_command)
        .subcommand(uninstall_command)
        .subcommand(list_command)
        .subcommand(use_command)
}
