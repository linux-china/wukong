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
