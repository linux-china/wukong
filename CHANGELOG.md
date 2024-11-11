# Unreleased

Nothing Yet!

# Version 0.2.5 (2024-11-12)

* sdkman-rs: fix `sdk install java 17`

# Version 0.2.4 (2024-10-29)

* sdkman-rs: add Java major version support: `sdk install java 17`, `sdk use java 17`
* maven-toolchain: extract vendor from java version, such as `17.0.4-tem`

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
