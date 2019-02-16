# bad-jvm-class-files
This is a Rust script for creating some JVM class files that violate different parts of the JVM 11 standards.

## Usage
### Setup
```
$ git clone git@github.com:ExcaliburZero/jvm-class-file-parser.git
$ git clone git@github.com:ExcaliburZero/bad-jvm-class-files.git
$ cd bad-jvm-class-files
```

### Running
```
$ cargo +nightly run
```

### Inpsecting class files
```
# Change this to point to your JDK 11 installation
$ export JAVA_HOME=/usr/lib/jvm/java-1.11.0-openjdk-amd64

# Inspect a bad class file using `javap`
$ $JAVA_HOME/bin/javap -c -v AllAccessFlags.class
```
