#![allow(unused)]
use anyhow::{Context, Result};
pub mod regex_feature;
use clap::Parser;
use std::io::{self, read_to_string, BufRead, BufReader, Write};
use std::{error::Error, fs::File, path::PathBuf};
#[derive(Parser, Debug)]
#[command(author, version, about)]

/// Structure to store the given arguments from the terminal.
pub struct Config {
    /// Pattern to search in the file.
    #[arg(short, long)]
    pub query: String,
    /// Take pattern as regular expression.
    #[arg(short = 'r', long)]
    pub regex_match: bool,
    /// Path to the file.
    #[arg(short, long)]
    pub file_path: PathBuf,
    /// Ignore case distinctions while searching QUERY in FILE_PATH.
    #[arg(short, long)]
    pub ignore_case: bool,
    /// Print the lines without QUERY pattern in the FILE_PATH.
    #[arg(short = 'v', long)]
    pub invert_match: bool,
}
/// Checks if pattern is regular expression or not.
pub fn run(config: Config) -> Result<()> {
    if config.regex_match {
        regex_feature::run_regex(config)
    } else {
        run_string(config)
    }
}

/// Search for the string query in the given file and display the lines that contain it.
pub fn run_string(config: Config) -> Result<()> {
    // Open the file
    let file = File::open(&config.file_path)
        .with_context(|| format!("could not open the file `{:?}`", &config.file_path))?;
    // Read the file in buffer (8 KB).
    let reader = BufReader::new(file);
    let contents = read_to_string(reader)
        .with_context(|| format!("could not read the file `{:?}`", &config.file_path))?;
    // Searching for the query string with ignore_case option in the contents.
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents, config.invert_match)
    } else {
        search(&config.query, &contents, config.invert_match)
    };

    // Writing buffer of lines on the terminal which satisfies the command.
    for line in results {
        writeln!(io::BufWriter::new(io::stdout().lock()), "{line}");
    }
    Ok(())
}
/// This function search for the query string with exact case.
pub fn search<'a>(query: &str, contents: &'a str, invert: bool) -> Vec<&'a str> {
    let mut results = Vec::new();
    //Branches if invert_match option is active or not.
    if invert {
        for line in contents.lines() {
            if !line.contains(query) {
                results.push(line);
            }
        }
    } else {
        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }
    }
    results
}
/// This function search for the query string without case distinction.
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str, invert: bool) -> Vec<&'a str> {
    // Lower the case of query string.
    let query = query.to_lowercase();
    let mut results = Vec::new();
    if invert {
        for line in contents.lines() {
            if !(line.to_lowercase().contains(&query)) {
                results.push(line);
            }
        }
    } else {
        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }
    }
    results
}
/// Small unitest to check the functionality of the program.
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // Test for case sensitive `search` function.
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duckt three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, false)
        );
    }

    #[test]
    // Test for `case search_case_insensitive function`.
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents, false)
        );
    }
    #[test]
    // Test for the `invert_match` option.
    fn invert_search() {
        let query = "Duckt";
        let contents = "\
safe, fast, productive.
Duckt three.
duckt";
        assert_ne!(vec!["Duckt three."], search(query, contents, true));
        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_insensitive(query, contents, true)
        );
    }
}
