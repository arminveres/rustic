use minigrep::{run, Config};
use std::{env, process};

// TODO: could use clap to parse our arguments
// use clap::Parser;
// #[derive(clap::Parser)]
// pub struct Config {
//     /// Query to look for
//     query: String, //&'a str,
//     /// Path to look in for
//     file_path: std::path::PathBuf, //&'a str,
//     #[arg(long,short)]
//     ignore_case: Option<bool>
// }

fn main() {
    // let args = Config::parse();

    // Use args instead of args_os (which would accept utf-8 strings), because it returs Strings,
    // which are easier to work with
    // Pass the env args iterator to the config!
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!(
            "Problem parsing arguments: {err}.
Usage: [query] [path] ..
    --ignore-case, -i: case insensitive search"
        );
        process::exit(1);
    });

    // we are using if-let, because the () in case of success is irrelevant for us
    if let Err(e) = run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}
