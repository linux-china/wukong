Wukong - Java Toolchain with Rust
====================================
Wukong is a Java toolchain written with Rust.

# Get started

- Install: `cargo binstall wukong`. Please run `cargo install cargo-binstall` first.
- JBang-rs: `~/.cargo/bin/jbang --version`
- SDKMAN-rs: `~/.cargo/bin/sdk --version`
- jenv-rs: `~/.cargo/bin/jenv --version`
- Maven Toolchains: `~/.cargo/bin/mt --version`

# Toolchain

- JBang(Java): https://www.jbang.dev/
- SDKMAN(bash): https://sdkman.io/
- jenv(bash): https://github.com/jenv/jenv
- Maven Toolchains CLI: https://maven.apache.org/guides/mini/guide-using-toolchains.html

# Maven Toolchains CLI

- jdks: list all installed JDKs
- list: list JDKs from `~/.m2/toolchains.xml`

### Add JDK

- `mt add 18`: add JDK 18 from `$HOME/.jbang/cache/jdks/18`
- `mt add 17.0.4-tem`: add JDK from `$HOME/.sdk/candidates/java/17.0.4-tem`
- `mt add /path/to/java-home`: add JDK from `/path/to/java-home`

# References

* [SDKMAN CLI](https://github.com/sdkman/sdkman-cli-native): SDKMAN CLI Native with Rust
* [OneIO](https://github.com/bgpkit/oneio): all-in-one convenient IO library for Rust
