Wukong - Java Toolchain with Rust
====================================
Wukong is a Java toolchain written with Rust.

# Why write a Java toolchain with Rust?

- Startup time matters: Rust is fast, and Java takes ages to start.
- No environment dependencies: No need to install JDK with different versions.
- Binary file size matters: small size, easy to distribute. GraalVM native-image is not small enough.

# Get started

- Install: `cargo binstall wukong`. Please run `cargo install cargo-binstall` first.
- JBang-rs: `~/.cargo/bin/jbang --help`
- SDKMAN-rs: `~/.cargo/bin/sdk --help`
- jenv-rs: `~/.cargo/bin/jenv --help`
- Maven Toolchains: `~/.cargo/bin/mt --help`
- Maven Central Search: `~/.cargo/bin/mcs --help`
- JAR file analyzer: `~/.cargo/bin/jarviz --help`

# Java Toolchains

- JBang(Java): https://www.jbang.dev/
- SDKMAN(bash): https://sdkman.io/
- jenv(bash): https://github.com/jenv/jenv
- Maven Toolchains CLI: https://maven.apache.org/guides/mini/guide-using-toolchains.html
- Maven Central Search: https://search.maven.org/
- JAR file analyzer: https://github.com/kordamp/jarviz

# SDKMAN-rs

Please add `eval $(~/.cargo/bin/sdk init)` to your shell profile.

### enhancements

* CI friendly: `sdk install -y java` for auto-install
* Silent mode: `sdk -q install java`
* Major version support(Temurin by default): `sdk install java 21`, `sdk use java 21`

### Difference

* use version: `eval $(sdk use java 21)`

# Maven Toolchains CLI

- jdks: list all installed JDKs
- vendors: list all vendors and available JDK versions.
- list: list JDKs from `~/.m2/toolchains.xml`
- add: add JDK into `~/.m2/toolchains.xml`
- remove: remove JDK from `~/.m2/toolchains.xml`

### Add JDK

- list all vendors and jdk versions: `mt vendors`
- `mt add 21`: add JDK 21 from `$HOME/.jbang/cache/jdks/21`
- `mt add 17.0.4-tem`: add JDK from `$HOME/.sdk/candidates/java/17.0.4-tem`
- `mt add /path/to/java-home`: add JDK from `/path/to/java-home`

# jarviz

`jarviz` is a JAR file analyzer written in Rust, and inspired by [kordamp/jarviz](https://github.com/kordamp/jarviz).

- bytecode matric: `jarviz bytecode matrix`
- bytecode show: `jarviz bytecode show --pom`
- entries list: `jarviz entries list --file=path/to/jarfile.jar`
- services list: `jarviz services list --pom`

# direnv integration

Integration with [direnv](https://direnv.net/) by `sdk direnv init`:

- Java Home: `.java-version`, `.sdkmanrc`
- Aut candidate home and path for SDKMAN: `.sdkmanrc`

# References

* [SDKMAN CLI](https://github.com/sdkman/sdkman-cli-native): SDKMAN CLI Native with Rust
* [OneIO](https://github.com/bgpkit/oneio): all-in-one convenient IO library for Rust
* [startup-time](https://github.com/bdrung/startup-time): Measure startup time of different programming languages
* jbang(1): https://www.jbang.dev/documentation/guide/latest/cli/jbang.html

