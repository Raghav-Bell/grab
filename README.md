![build](https://img.shields.io/github/actions/workflow/status/Raghav-Bell/graby/rust.yml)
![version](https://img.shields.io/crates/v/graby.svg)
![version_downloades](https://img.shields.io/crates/d/graby/0.1.0)
![License](https://img.shields.io/crates/l/graby/0.1.0)


# Introduction
This is a small implementation of `grep` command line tool in rust (see References). 
Unlike `grep` this implementation does not support `regex`. 
For complete implementation of `grep` in rust, check <a href="https://github.com/BurntSushi/ripgrep"> `ripgrep`</a>.

## Installation
Updated soon.
## Usage
For searching `QUERY` pattern in `FILE_PATH` use following command:
```
graby --q QUERY --f FILE_PATH
```
For more options run
```
graby --help
```
It is licensed under MIT.
## How to Contribute
All small or large contributions are welcomed .
## References
<a href ="https://doc.rust-lang.org/book/ch12-00-an-io-project.html"> ch-12 rust-lang book</a>, <a href="https://rust-cli.github.io/book/index.html">Command Line Applications in Rust</a>
