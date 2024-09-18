# display jbang help
jbang-help:
  cargo run --bin jbang -- help

# display jbang help
jbang-jdk-list:
  cargo run --bin jbang -- jdk list

# display jbang help
jbang-jdk-java-env:
  cargo run --bin jbang -- jdk java-env 21

# display jbang config
jbang-config-list:
  cargo run --bin jbang -- config list

# display jbang config
jbang-init-hello:
  cargo run --bin jbang -- init hello

# jbang trust lit
jbang-trust-list:
  cargo run --bin jbang -- trust list

# display jbang version
jbang-version:
  cargo run --bin jbang -- version

# jbang run hello.java
jbang-run:
  cargo run --bin jbang -- scripts/hello.java first second

# display sdkman help
sdkman-help:
  cargo run --bin sdk -- help

# sdk list
sdkman-list:
  cargo run --bin sdk -- list

# sdk list java
sdkman-list-java:
  cargo run --bin sdk -- list java

# sdk install java
sdkman-install-java:
  cargo run --bin sdk -- install java

# sdk use java
sdkman-use-java:
  cargo run --bin sdk -- use java 22.0.2-tem

# sdk home java
sdkman-home-java:
  cargo run --bin sdk -- home java 22.0.2-tem
