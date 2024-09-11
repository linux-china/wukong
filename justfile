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

# display sdkman help
sdkman-help:
  cargo run --bin sdk -- help
