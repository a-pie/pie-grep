use std::error::Error;
use std::fs;
use std::env;


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// Box<dyn Error> means the function will return a type that implements the Error trait, 
// but we don’t have to specify what particular type the return value will be. 
// This gives us flexibility to return error values that may be of different types in 
// different error cases. The dyn keyword is short for “dynamic.”
// We’ve declared the run function’s success type as () in the signature "-> Result<(), Box<dyn Error>>"", 
// which means we need to wrap the unit type value in the Ok value. 
// This Ok(()) syntax might look a bit strange at first, but using () like this is the 
// idiomatic way to indicate that we’re calling run for its side effects only; it doesn’t return a value we need.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// We need an explicit lifetime 'a defined in the signature of search and used with the contents argument and the return value. 
// Lifetime parameters specify which argument lifetime is connected to the lifetime of the return value. 
// In this case, we indicate that the returned vector should contain string slices that reference slices of the argument contents (rather than the argument query).
// In other words, we tell Rust that the data returned by the search function will live as long as the data passed into the search function in the contents argument. 
// This is important! The data referenced by a slice needs to be valid for the reference to be valid; if the compiler assumes we’re making string slices of query 
// rather than contents, it will do its safety checking incorrectly.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
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
}