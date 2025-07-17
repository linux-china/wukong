use clap::{Arg, Command};

pub const VERSION: &str = "0.3.0";

pub fn build_jarviz_app() -> Command {
    Command::new("jarviz")
        .version(VERSION)
        .about("jarviz - JAR file analyzer")
        .subcommand(
            Command::new("bytecode")
                .about("Commands for the JAR's bytecode")
                .subcommand(
                    Command::new("show")
                        .about("Show the JAR's bytecode version(s)")
                        .arg(
                            Arg::new("gav")
                                .help("Maven GAV coordinates, i.e, com.fasterxml.jackson.core:jackson-core:2.19.0")
                                .long("gav")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("url")
                                .help("URL to target resource")
                                .long("url")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("file")
                                .help("Path to a local JAR file")
                                .long("file")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("classpath")
                                .help("Platform specific set of file paths i.e, /opt/jars/file.jar:/opt/jars/file2.jar")
                                .long("classpath")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("directory")
                                .help("Path to a directory that contains JAR files. Jarviz will recursively walk the tree looking for **/*.jar")
                                .long("directory")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("pom")
                                .help("Show bytecode version(s) from the Maven project")
                                .long("pom")
                                .num_args(0)
                                .required(false),
                        )
                        .arg(
                            Arg::new("gradle")
                                .help("Show bytecode version(s) from the Gradle file")
                                .long("gradle")
                                .num_args(0)
                                .required(false),
                        )
                        .arg(
                            Arg::new("output-format")
                                .help("Output format to use, such as text, csv, json, and default is text")
                                .long("output-format")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("details")
                                .help("Show matching class names.")
                                .long("details")
                                .num_args(0)
                                .required(false),
                        )
                        .arg(
                            Arg::new("bytecode-version")
                                .help("Bytecode version to search.")
                                .long("bytecode-version")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("java-version")
                                .help("Java version to search.")
                                .long("java-version")
                                .num_args(1)
                                .required(false),
                        ),
                )
                .subcommand( Command::new("matrix")
                    .about("Show matrix for java and bytecode versions"))
        )
        .subcommand(
            Command::new("entries")
                .about("Commands for JAR entries")
                .subcommand(
                    Command::new("extract")
                        .about("Extract a given JAR entry or entries")
                        .arg(
                            Arg::new("gav")
                                .help("Maven GAV coordinates, i.e, com.fasterxml.jackson.core:jackson-core:2.19.0")
                                .long("gav")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("url")
                                .help("URL to target resource")
                                .long("url")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("file")
                                .help("Path to a local JAR file")
                                .long("file")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("classpath")
                                .help("Platform specific set of file paths i.e, /opt/jars/file.jar:/opt/jars/file2.jar")
                                .long("classpath")
                                .num_args(1)
                                .required(false),
                        ).arg(
                        Arg::new("directory")
                            .help("Path to a directory that contains JAR files. Jarviz will recursively walk the tree looking for **/*.jar")
                            .long("directory")
                            .num_args(1)
                            .required(false),
                    ),
                )
                .subcommand(
                    Command::new("find")
                        .about("Find a given JAR entry or entries")
                        .arg(
                            Arg::new("gav")
                                .help("Maven GAV coordinates, i.e, com.fasterxml.jackson.core:jackson-core:2.19.0")
                                .long("gav")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("url")
                                .help("URL to target resource")
                                .long("url")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("file")
                                .help("Path to a local JAR file")
                                .long("file")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("classpath")
                                .help("Platform specific set of file paths i.e, /opt/jars/file.jar:/opt/jars/file2.jar")
                                .long("classpath")
                                .num_args(1)
                                .required(false),
                        ).arg(
                        Arg::new("directory")
                            .help("Path to a directory that contains JAR files. Jarviz will recursively walk the tree looking for **/*.jar")
                            .long("directory")
                            .num_args(1)
                            .required(false),
                    ),
                )
        )
        .subcommand(
            Command::new("manifest")
                .about("Commands for the JAR's manifest")
                .subcommand(
                    Command::new("show")
                        .about("Show the JAR's manifest")
                        .arg(
                            Arg::new("gav")
                                .help("Maven GAV coordinates, i.e, com.fasterxml.jackson.core:jackson-core:2.19.0")
                                .long("gav")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("url")
                                .help("URL to target resource")
                                .long("url")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("file")
                                .help("Path to a local JAR file")
                                .long("file")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("classpath")
                                .help("Platform specific set of file paths i.e, /opt/jars/file.jar:/opt/jars/file2.jar")
                                .long("classpath")
                                .num_args(1)
                                .required(false),
                        ).arg(
                        Arg::new("directory")
                            .help("Path to a directory that contains JAR files. Jarviz will recursively walk the tree looking for **/*.jar")
                            .long("directory")
                            .num_args(1)
                            .required(false),
                    ),
                )
                .subcommand(
                    Command::new("query")
                        .about("Query manifest attributes")
                        .arg(
                            Arg::new("gav")
                                .help("Maven GAV coordinates, i.e, com.fasterxml.jackson.core:jackson-core:2.19.0")
                                .long("gav")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("url")
                                .help("URL to target resource")
                                .long("url")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("file")
                                .help("Path to a local JAR file")
                                .long("file")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("classpath")
                                .help("Platform specific set of file paths i.e, /opt/jars/file.jar:/opt/jars/file2.jar")
                                .long("classpath")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("directory")
                                .help("Path to a directory that contains JAR files. Jarviz will recursively walk the tree looking for **/*.jar")
                                .long("directory")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("attribute-name")
                                .help("Name of a manifest attribute.")
                                .long("attribute-name")
                                .num_args(1)
                                .required(true),
                    ),
                )
        )
        .subcommand(
            Command::new("module")
                .about("Commands for modular JARs")
                .subcommand(
                    Command::new("name")
                        .about("Show the module name")
                        .arg(
                            Arg::new("gav")
                                .help("Maven GAV coordinates, i.e, com.fasterxml.jackson.core:jackson-core:2.19.0")
                                .long("gav")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("url")
                                .help("URL to target resource")
                                .long("url")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("file")
                                .help("Path to a local JAR file")
                                .long("file")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("classpath")
                                .help("Platform specific set of file paths i.e, /opt/jars/file.jar:/opt/jars/file2.jar")
                                .long("classpath")
                                .num_args(1)
                                .required(false),
                        ).arg(
                        Arg::new("directory")
                            .help("Path to a directory that contains JAR files. Jarviz will recursively walk the tree looking for **/*.jar")
                            .long("directory")
                            .num_args(1)
                            .required(false),
                    ),
                )
                .subcommand(
                    Command::new("descriptor")
                        .about("Show the module descriptor")
                        .arg(
                            Arg::new("gav")
                                .help("Maven GAV coordinates, i.e, com.fasterxml.jackson.core:jackson-core:2.19.0")
                                .long("gav")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("url")
                                .help("URL to target resource")
                                .long("url")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("file")
                                .help("Path to a local JAR file")
                                .long("file")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("classpath")
                                .help("Platform specific set of file paths i.e, /opt/jars/file.jar:/opt/jars/file2.jar")
                                .long("classpath")
                                .num_args(1)
                                .required(false),
                        ).arg(
                        Arg::new("directory")
                            .help("Path to a directory that contains JAR files. Jarviz will recursively walk the tree looking for **/*.jar")
                            .long("directory")
                            .num_args(1)
                            .required(false),
                    ),
                )
        )
        .subcommand(
            Command::new("packages")
                .about("Commands for packages")
                .subcommand(
                    Command::new("split")
                        .about("Display split packages")
                        .arg(
                            Arg::new("gav")
                                .help("Maven GAV coordinates, i.e, com.fasterxml.jackson.core:jackson-core:2.19.0")
                                .long("gav")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("url")
                                .help("URL to target resource")
                                .long("url")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("file")
                                .help("Path to a local JAR file")
                                .long("file")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("classpath")
                                .help("Platform specific set of file paths i.e, /opt/jars/file.jar:/opt/jars/file2.jar")
                                .long("classpath")
                                .num_args(1)
                                .required(false),
                        ).arg(
                        Arg::new("directory")
                            .help("Path to a directory that contains JAR files. Jarviz will recursively walk the tree looking for **/*.jar")
                            .long("directory")
                            .num_args(1)
                            .required(false),
                    ),
                )
                .subcommand(
                    Command::new("validate")
                        .about("Validate package names")
                        .arg(
                            Arg::new("gav")
                                .help("Maven GAV coordinates, i.e, com.fasterxml.jackson.core:jackson-core:2.19.0")
                                .long("gav")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("url")
                                .help("URL to target resource")
                                .long("url")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("file")
                                .help("Path to a local JAR file")
                                .long("file")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("classpath")
                                .help("Platform specific set of file paths i.e, /opt/jars/file.jar:/opt/jars/file2.jar")
                                .long("classpath")
                                .num_args(1)
                                .required(false),
                        ).arg(
                        Arg::new("directory")
                            .help("Path to a directory that contains JAR files. Jarviz will recursively walk the tree looking for **/*.jar")
                            .long("directory")
                            .num_args(1)
                            .required(false),
                    ),
                )
        )
        .subcommand(
            Command::new("services")
                .about("Commands for declarative services")
                .subcommand(
                    Command::new("list")
                        .about("Display registered services")
                        .arg(
                            Arg::new("gav")
                                .help("Maven GAV coordinates, i.e, com.fasterxml.jackson.core:jackson-core:2.19.0")
                                .long("gav")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("url")
                                .help("URL to target resource")
                                .long("url")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("file")
                                .help("Path to a local JAR file")
                                .long("file")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("classpath")
                                .help("Platform specific set of file paths i.e, /opt/jars/file.jar:/opt/jars/file2.jar")
                                .long("classpath")
                                .num_args(1)
                                .required(false),
                        ).arg(
                        Arg::new("directory")
                            .help("Path to a directory that contains JAR files. Jarviz will recursively walk the tree looking for **/*.jar")
                            .long("directory")
                            .num_args(1)
                            .required(false),
                    ),
                )
                .subcommand(
                    Command::new("show")
                        .about("Display service implementations")
                        .arg(
                            Arg::new("gav")
                                .help("Maven GAV coordinates, i.e, com.fasterxml.jackson.core:jackson-core:2.19.0")
                                .long("gav")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("url")
                                .help("URL to target resource")
                                .long("url")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("file")
                                .help("Path to a local JAR file")
                                .long("file")
                                .num_args(1)
                                .required(false),
                        )
                        .arg(
                            Arg::new("classpath")
                                .help("Platform specific set of file paths i.e, /opt/jars/file.jar:/opt/jars/file2.jar")
                                .long("classpath")
                                .num_args(1)
                                .required(false),
                        ).arg(
                        Arg::new("directory")
                            .help("Path to a directory that contains JAR files. Jarviz will recursively walk the tree looking for **/*.jar")
                            .long("directory")
                            .num_args(1)
                            .required(false),
                    ),
                )
        )
        .subcommand(
            Command::new("checksum")
                .about("Verify JAR checksums")
                .arg(
                    Arg::new("gav")
                        .help("Maven GAV coordinates, i.e, com.fasterxml.jackson.core:jackson-core:2.19.0")
                        .long("gav")
                        .num_args(1)
                        .required(false),
                )
                .arg(
                    Arg::new("url")
                        .help("URL to target resource")
                        .long("url")
                        .num_args(1)
                        .required(false),
                )
                .arg(
                    Arg::new("file")
                        .help("Path to a local JAR file")
                        .long("file")
                        .num_args(1)
                        .required(false),
                )
                .arg(
                    Arg::new("classpath")
                        .help("Platform specific set of file paths i.e, /opt/jars/file.jar:/opt/jars/file2.jar")
                        .long("classpath")
                        .num_args(1)
                        .required(false),
                ).arg(
                Arg::new("directory")
                    .help("Path to a directory that contains JAR files. Jarviz will recursively walk the tree looking for **/*.jar")
                    .long("directory")
                    .num_args(1)
                    .required(false),
            )
        )
}
