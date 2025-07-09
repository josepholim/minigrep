use std::fs;
use std::io;
use std::error::Error;

use regex::Regex;

pub mod args;
use args::Args;

pub mod output;
use output::print_matches;

pub fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    let contents = if args.path == "-" {
        io::read_to_string(io::stdin())?
    } else {
        fs::read_to_string(&args.path)?
    };

    let matcher = |line: &str| {
        let hit = if args.regex {
            Regex::new(&args.query).unwrap().is_match(line)
        } else if args.ignore_case {
            line.to_lowercase().contains(&args.query.to_lowercase())
        } else {
            line.contains(&args.query)
        };
        if args.invert { !hit } else { hit }
    };

    let matches: Vec<(usize, &str)> = contents
        .lines()
        .enumerate()
        .filter(|(_, line)| matcher(line))
        .collect();

    print_matches(&matches, args);

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

pub fn search_regex<'a>(pattern: &Regex, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| pattern.is_match(line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn regex_basic() {
        let query = r"duct";
        let pattern = Regex::new(query).unwrap();
        let text = "\
Rust:
safe, productive, and fast-paced.
Pick three.";
        assert_eq!(
            vec!["safe, productive, and fast-paced."],
            search_regex(&pattern, text)
        );
    }

    #[test]
    fn regex_word_boundary() {
        let query = r"\bfoo\b";
        let pattern = Regex::new(query).unwrap();
        let text = "\
foobar
 foo bar
foo";
        assert_eq!(
            vec![" foo bar", "foo"],
            search_regex(&pattern, text)
        );
    }
}