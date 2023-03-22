use std::{env, error::Error, fs, process};

// I chose to try it with lifetimes, the book uses Strings. It argues, that while cloning the
// string slice references would be later, using the Strings is easier to handle... let's see !
#[derive(Debug)]
pub struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let file_path = &args[2];
        Ok(Self { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &contents) {
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
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
