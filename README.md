## Newsletter in rust

An email newsletter service that supports what you need to get off the ground if you are willing to add an email subscription page to your blog, nothing more, nothing less.

### Set faster Linking 
To speed up the linking phase you have to install the alternative linker on your machine and add this configuration file to the project:
```
# .cargo/config.toml
# On Linux:
# - Ubuntu, `sudo apt-get install lld clang`
rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]
# On MacOS, `brew install michaeleisel/zld/zld`
[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]
```
### Set tools for Continuous Integration
For Code Coverage
```
cargo install cargo-tarpaulin
cargo tarpaulin --ignore-tests // computes code coverage ignoring your test functions
```
For Linting
```
rustup component add clippy
cargo clippy
cargo clippy -- -D warnings // fail the linter check if clippy emits any warnings
```
For Formatting
```
rustup component add rustfmt
cargo fmt -- --check // formatting step in CI pipeline
```
For Security Vulnerabilities
```
cargo install cargo-audit
cargo audit
```
