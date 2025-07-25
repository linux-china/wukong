# build project and copy to ~/bin
build:
    cargo build
    cp -rf target/debug/jbang ~/bin/jbang
    cp -rf target/debug/sdk ~/bin/sdk
    cp -rf target/debug/jenv ~/bin/jenv
    cp -rf target/debug/mt ~/bin/mt
    cp -rf target/debug/jarviz ~/bin/jarviz

release:
    cargo build --release
    cp -rf target/debug/jbang ~/bin/jbang
    cp -rf target/debug/sdk ~/bin/sdk
    cp -rf target/debug/jenv ~/bin/jenv
    cp -rf target/debug/mt ~/bin/mt
    cp -rf target/debug/jarviz ~/bin/jarviz

# local install
local-install:
    cargo install --path .

# display jbang help
jbang-help:
    cargo run --bin jbang -- --help

# display jbang version
jbang-version:
    cargo run --bin jbang -- --version

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

# jbang run hello.java
jbang-run:
    cargo run --bin jbang -- scripts/hello.java first second

# jbang run by java
jbang-raw-hello:
    java -classpath $HOME/.jbang/bin/jbang.jar dev.jbang.Main run scripts/hello.java first second

# jbang run hello.java
jbang-alias-list:
    cargo run --bin jbang -- alias list

# jbang run hello.java
jbang-catalog-list:
    cargo run --bin jbang -- catalog list

# jbang info tools
jbang-info-tools:
    cargo run --bin jbang -- info tools scripts/hello.java

# jbang wrapper
jbang-wrapper--help:
    cargo run --bin jbang -- wrapper --help

# display sdkman help
sdkman-help:
    cargo run --bin sdk -- help

# sdk list
sdkman-list:
    cargo run --bin sdk -- list

# sdk list local
sdkman-list-local:
    cargo run --bin sdk -- list --local

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

# sdk current
sdkman-current:
    cargo run --bin sdk -- current

# jenv help
jenv-help:
    cargo run --bin jenv -- --help

# jenv help
jenv-init:
    cargo run --bin jenv -- init -

# jenv help
jenv-versions:
    cargo run --bin jenv -- versions

# mt list all jdks
mt-list:
    cargo run --bin mt -- list

# mt list all jdks
mt-vendors:
    cargo run --bin mt -- vendors

# mt list all jdks
mt-jdks:
    cargo run --bin mt -- jdks

# search by class name
mcs-class-search:
    cargo run --bin mcs -- class-search ApplicationContext

# search by artifact
mcs-search:
    cargo run --bin mcs -- search spring-messaging

# artifact info
mcs-info:
    cargo run --bin mcs -- info 'org.apache.commons:commons-lang3:3.18.0'

# jar info
mcs-jar-info:
    cargo run --bin mcs -- info ~/.m2/repository/commons-io/commons-io/2.18.0/commons-io-2.18.0.jar

# jarviz info
jarviz-help:
    cargo run --bin jarviz -- --help

# jarviz bytecode matrix
jarviz-bytecode-matrix:
    cargo run --bin jarviz -- bytecode matrix

# jarviz bytecode
jarviz-bytecode-show:
    cargo run --bin jarviz -- bytecode show --details --file=$HOME/.m2/repository/org/apache/commons/commons-csv/1.14.0/commons-csv-1.14.0.jar

# jarviz manifest
jarviz-manifest:
    cargo run --bin jarviz -- manifest show --file=$HOME/.m2/repository/org/apache/commons/commons-csv/1.14.0/commons-csv-1.14.0.jar

# jarviz manifest
jarviz-manifest-query:
    cargo run --bin jarviz -- manifest query --attribute-name=bundle-name --file=$HOME/.m2/repository/org/apache/commons/commons-csv/1.14.0/commons-csv-1.14.0.jar

# jarviz entries
jarviz-entries-find:
    cargo run --bin jarviz -- entries find --output-format=csv --file=$HOME/.m2/repository/org/apache/commons/commons-csv/1.14.0/commons-csv-1.14.0.jar

# jarviz services list
jarviz-services-list:
    cargo run --bin jarviz -- services list --output-format=csv --file=$HOME/.m2/repository/ch/qos/logback/logback-classic/1.5.18/logback-classic-1.5.18.jar
