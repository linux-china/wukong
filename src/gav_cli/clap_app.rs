use clap::{Arg, ArgAction, Command};

pub const VERSION: &str = "0.1.0";

// Apache Maven Dependencies Management: https://maven.apache.org/plugins/maven-dependency-plugin/examples/managing-dependencies.html
pub fn build_gav_app() -> Command {
    let gav_add_command = Command::new("add")
        .about("Add dependency by GAV")
        .arg(
            Arg::new("gav")
                .help("GAV with 'groupId:artifactId[:extension[:classifier]]:version' format")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::new("project")
                .help("project name in Multi-Module Projects")
                .long("pl")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("profile")
                .help("specific Maven profile")
                .long("profile")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("compile")
                .help("Add dependency as compile scope")
                .long("compile")
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            Arg::new("scope")
                .help("scope parameter: compile, test, import, provided, runtime, system")
                .long("scope")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("type")
                .help("dependency type: jar, pom, war etc.")
                .long("type")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("classifier")
                .help("dependency classifier: sources, javadoc, linux-x86_64, jdk21 etc.")
                .long("classifier")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("test")
                .alias("dev")
                .help("Add dependency as test scope")
                .long("test")
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            Arg::new("provided")
                .help("Add dependency as provided scope")
                .long("provided")
                .short('D')
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            Arg::new("managed")
                .help("Add dependency in <dependencyManagement>")
                .long("managed")
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            Arg::new("bom")
                .help("import a BOM (Bill of Materials) into <dependencyManagement>")
                .long("bom")
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            Arg::new("optional")
                .help("Add dependency as optional")
                .long("optional")
                .action(ArgAction::SetTrue)
                .required(false),
        );
    let gav_remove_command = Command::new("remove")
        .about("Remove dependency by GAV")
        .arg(
            Arg::new("gav")
                .help("GAV with 'groupId:artifactId[:extension[:classifier]]:version' format")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::new("project")
                .help("project name in Multi-Module Projects")
                .long("pl")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("profile")
                .help("specific Maven profile")
                .long("profile")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("type")
                .help("dependency type: jar, pom, war etc.")
                .long("type")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("classifier")
                .help("dependency classifier: sources, javadoc, linux-x86_64, jdk21 etc.")
                .long("classifier")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("bom")
                .help("Remove a BOM (Bill of Materials) from <dependencyManagement>")
                .long("bom")
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            Arg::new("managed")
                .help("remove dependency in <dependencyManagement>")
                .long("managed")
                .action(ArgAction::SetTrue)
                .required(false),
        );
    let tree_command = Command::new("tree")
        .alias("list")
        .about("Display dependency tree");
    Command::new("gav")
        .version(VERSION)
        .about("gav - Maven Dependency Management with Dependency Plugin 3.11+")
        .subcommand(gav_add_command)
        .subcommand(gav_remove_command)
        .subcommand(tree_command)
}
