# Unreleased

Nothing Yet!

# Version 0.2.9 (2025-04-30)

* jbang-rs: add `DEEPSEEK_API_KEY` support to generate code from AI
* sdkman: add `sdk env update` to update Java version from env

# Version 0.2.8 (2025-01-24)

* mcs-rs(Maven Central Search): add jar file support for info -
  `mcs info ~/.m2/repository/commons-io/commons-io/2.18.0/commons-io-2.18.0.jar`

# Version 0.2.7 (2024-12-28)

* mcs-rs(Maven Central Search): add info sub command - `mcs info 'org.apache.commons:commons-lang3:3.17.0'`
* sdkman: add jdk alias for java - `sdk list jdk`

# Version 0.2.6 (2024-12-02)

* mcs-rs(Maven Central Search): initial version

# Version 0.2.5 (2024-11-12)

* sdkman-rs: fix `sdk install java 17`

# Version 0.2.4 (2024-10-29)

* sdkman-rs: add Java major version support: `sdk install java 17`, `sdk use java 17`
* maven-toolchain: extract vendor from Java version, such as `17.0.4-tem`

# Version 0.2.2 (2024-10-15)

* jbang-rs: implement `version` sub command
* jbang-rs: add `-V`, `-v` and `--version` options
* jbang-rs: implement `template` sub command
* jbang-rs: implement `completion` sub command

# Version 0.2.1 (2024-10-07)

* jbang-rs: implement `cache` with Rust, `jdk list --available`.
* jbang-rs: add jdk install for `jdk java-env`
* jbang-rs: OpenAI for `init` to generate code from AI
* sdkman: candidates auto-install from `.sdkmanrc`

# Version 0.2.0 (2024-09-25)

* jbang-rs: implement most of the commands. `build`, `run`, `export`, `info`, `cache`, `edit` are delegated to Java
  implementation.
* sdkman: implement commands and add `direnv` command
* jenv: implement commands
* mt(Maven Toolchains): initial version

# Version 0.1.3 (2024-09-19)

* jbang-rs: initial version
* sdkman: initial version
