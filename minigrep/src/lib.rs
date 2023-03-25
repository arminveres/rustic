use std::{env, error::Error, fs};

// I chose to try it with lifetimes, the book uses Strings. It argues, that while cloning the
// string slice references would be later, using the Strings is easier to handle... let's see !
#[derive(Debug)]
pub struct Config<'a> {
    query: &'a str,
    file_path: std::path::PathBuf,
    ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = &args[1];
        let file_path = &args[2];

        // NOTE: (aver) another possibility: https://rust-cli.github.io/book/tutorial/cli-args.html
        // let query = env::args().nth(1).expect("No query given");
        // let file_path = env::args().nth(2).expect("No path given");

        let mut ignore_case = env::var("MINIGREP_IGNORE_CASE").is_ok();

        match args.get(3) {
            Some(arg) => match arg.as_str() {
                "-i" | "--ignore-case" => ignore_case = true,
                _ => {}
            },
            None => {}
        }

        Ok(Self {
            query,
            file_path: std::path::PathBuf::from(file_path),
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        case_insensitive_search(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(
    query: &str,
    contents: &'a str, // contents will be needing the string slices for its lifetime, not query
) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn case_insensitive_search<'a>(
    query: &str,
    contents: &'a str, // contents will be needing the string slices for its lifetime, not query
) -> Vec<&'a str> {
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
        let query = "dUcT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["safe, fast, productive."],
            case_insensitive_search(query, contents)
        );
    }
}
