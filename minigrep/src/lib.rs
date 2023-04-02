use std::{env, error::Error, fs};

// I chose to try it with lifetimes, the book uses Strings. It argues, that while cloning the
// string slice references would be later, using the Strings is easier to handle... let's see !
#[derive(Debug)]
pub struct Config {
    query: String,
    file_path: std::path::PathBuf,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip first position

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = match args.next() {
            Some(arg) => match arg.as_str() {
                "-i" | "--ignore-case" => true,
                _ => env::var("MINIGREP_IGNORE_CASE").is_ok(), // or eprintln!("Error, unknown key")
            },
            None => env::var("MINIGREP_IGNORE_CASE").is_ok(),
        };

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
    // NOTE: Here we require mutable states, while with the lower iterator and closure based we don't and
    // directly return the results
    /*
     let mut results = Vec::new();
     for line in contents.lines() {
         if line.contains(query) {
             results.push(line);
         }
     }
     results
    */
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn case_insensitive_search<'a>(
    query: &str,
    contents: &'a str, // contents will be needing the string slices for its lifetime, not query
) -> Vec<&'a str> {
    /*
     let query = query.to_lowercase();
     let mut results = Vec::new();

     for line in contents.lines() {
         if line.to_lowercase().contains(&query) {
             results.push(line);
         }
     }

     results
    */
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query.to_lowercase().as_str()))
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
