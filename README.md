![build](https://img.shields.io/github/actions/workflow/status/Raghav-Bell/graby/rust.yml)
![version](https://img.shields.io/crates/v/graby.svg)
![version_downloades](https://img.shields.io/crates/d/graby/0.1.0)
![License](https://img.shields.io/crates/l/graby/0.1.0)


# Introduction
This is a small implementation of `grep` command line tool in rust (see References). 
Unlike `grep` this implementation does not support `regex`. 
For complete implementation of `grep` in rust, check <a href="https://github.com/BurntSushi/ripgrep"> `ripgrep`</a>.

## Installation
For running or installing `graby`, install [`rust`](https://www.rust-lang.org/tools/install).
To add `graby` run the following `cargo` command in your project directory:
```
cargo add graby
```
or manually add following in `Cargo.toml` file.
```
graby = "0.1.1" # graby = "version"
```
To build `graby` from source you need to install rust on your device and run the following commands:
```
git clone https://github.com/Raghav-Bell/graby.git
cd graby
cargo run -- --help
```
## Usage
For searching `QUERY` pattern in `FILE_PATH` use following command:
```
graby --q QUERY --f FILE_PATH
```
For more options run
```
graby --help
```
or check [documentation](https://docs.rs/graby/0.1.0/graby/).
<br>It is licensed under MIT.
## How to Contribute
All small or large contributions are welcomed .
## References
<a href ="https://doc.rust-lang.org/book/ch12-00-an-io-project.html"> ch-12 rust-lang book</a>, <a href="https://rust-cli.github.io/book/index.html">Command Line Applications in Rust</a>
