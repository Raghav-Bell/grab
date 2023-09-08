#![allow(unused)]
use crate::Config;
use anyhow::{Context, Result};
use regex::Regex;
use std::io::{self, read_to_string, BufRead, BufReader, Write};
use std::{error::Error, fs::File, path::PathBuf};

/// Search for the query regex in the given file and display the lines that contain it.
pub fn run_regex(config: Config) -> Result<()> {
    // Open the file
    let file = File::open(&config.file_path)
        .with_context(|| format!("could not open the file `{:?}`", &config.file_path))?;
    // Read the file in buffer (8 KB).
    let reader = BufReader::new(file);
    let contents = read_to_string(reader)
        .with_context(|| format!("could not read the file `{:?}`", &config.file_path))?;

    // Searching for the query regex with ignore_case option in the contents.
    let results = if config.ignore_case {
        // adding regex ignore case flag in query string.
        let query_formatted = format!("(?i){}", &config.query);
        // Making regex from given `query_formatted`.
        let query_re = Regex::new(&query_formatted[..])
            .with_context(|| format!("given regular expression `{:?}` is wrong", &config.query))?;
        search_regex(&query_re, &contents, config.invert_match)
    } else {
        // Making regex from given query.
        let query_re = Regex::new(&config.query)
            .with_context(|| format!("given regular expression `{:?}` is wrong", &config.query))?;
        search_regex(&query_re, &contents, config.invert_match)
    };

    // Writing buffer of lines on the terminal which satisfies the command.
    for line in results {
        writeln!(io::BufWriter::new(io::stdout().lock()), "{line}");
    }
    Ok(())
}
/// This function search for the query regex with exact case.
pub fn search_regex<'a>(query_re: &Regex, contents: &'a str, invert: bool) -> Vec<&'a str> {
    let mut results = Vec::new();

    //Branches if `invert_match` option is active or not.
    if invert {
        for line in contents.lines() {
            if !query_re.is_match(line) {
                results.push(line);
            }
        }
    } else {
        for line in contents.lines() {
            if query_re.is_match(line) {
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
    // Test for exact case regex search function.
    fn regex_case_sensitive() {
        let query = Regex::new("HR[0-9]{3}-[0-9]{3}-[0-9]{4}").unwrap();
        let contents = "\
Rust:
safe, fast, productive.
Duckt three.
phone: HR111-222-3333";
        assert_eq!(
            vec!["phone: HR111-222-3333"],
            search_regex(&query, contents, false)
        );
    }

    #[test]
    // Test for ignore case regex search.
    fn regex_case_insensitive() {
        let query = Regex::new("(?i)rUsT[0-9]{3}").unwrap();
        let contents = "\
Rust123:
RUst324
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust123:", "RUst324"],
            search_regex(&query, contents, false)
        );
    }
    #[test]
    // Test for the `invert_match` option.
    fn regex_invert_search() {
        let query = Regex::new("Duckt[0-9]{3}").unwrap();
        let query_insensitive = Regex::new("(?i)Duckt[0-9]{3}").unwrap();
        let contents = "\
safe, fast, productive.
Duckt567 three.
duckt980";
        assert_ne!(
            vec!["Duckt567 three."],
            search_regex(&query, contents, true)
        );
        assert_eq!(
            vec!["safe, fast, productive."],
            search_regex(&query_insensitive, contents, true)
        );
    }
}
