use std::fs;
use std::error::Error;
use std::env;


pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for result in results {
        println!("{}", result);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<String> {
    let mut results = Vec::new();
    for (idx, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query){
            results.push(format!("On line {}: {}", idx+1, line))
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<String> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for (idx, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query){
            results.push(format!("On line {}: {}", idx+1, line))
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["On line 1: safe, fast, productive."], search(query, contents));
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
            vec!["On line 0: Rust:", "On line 3: Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}