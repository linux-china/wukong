Wukong - Java Toolchain with Rust
====================================
Wukong is a Java toolchain written with Rust.

# Why write Java toolchain with Rust?

- startup time matter: Rust is fast, and Java takes ages to start.
- No environment dependencies: No need to install JDK, Maven, Gradle, etc.
- Binary file size matter: small size, easy to distribute. GraalVM native-image is not small enough.

# Get started

- Install: `cargo binstall wukong`. Please run `cargo install cargo-binstall` first.
- JBang-rs: `~/.cargo/bin/jbang --version`
- SDKMAN-rs: `~/.cargo/bin/sdk --version`
- jenv-rs: `~/.cargo/bin/jenv --version`
- Maven Toolchains: `~/.cargo/bin/mt --version`

# Java Toolchains

- JBang(Java): https://www.jbang.dev/
- SDKMAN(bash): https://sdkman.io/
- jenv(bash): https://github.com/jenv/jenv
- Maven Toolchains CLI: https://maven.apache.org/guides/mini/guide-using-toolchains.html

# Maven Toolchains CLI

- jdks: list all installed JDKs
- list: list JDKs from `~/.m2/toolchains.xml`
- add: add JDK into `~/.m2/toolchains.xml`
- remove: remove JDK from `~/.m2/toolchains.xml`

### Add JDK

- `mt add 21`: add JDK 21 from `$HOME/.jbang/cache/jdks/21`
- `mt add 17.0.4-tem`: add JDK from `$HOME/.sdk/candidates/java/17.0.4-tem`
- `mt add /path/to/java-home`: add JDK from `/path/to/java-home`

# direnv integration

Integration with [direnv](https://direnv.net/) by `sdk direnv init`:

- Java Home: `.java-version`, `.sdkmanrc`
- Aut candidate home and path for SDKMAN: `.sdkmanrc`

# References

* [SDKMAN CLI](https://github.com/sdkman/sdkman-cli-native): SDKMAN CLI Native with Rust
* [OneIO](https://github.com/bgpkit/oneio): all-in-one convenient IO library for Rust
* [startup-time](https://github.com/bdrung/startup-time): Measure startup time of different programming languages
