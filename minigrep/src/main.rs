use minigrep::{run, Config};
use std::{env, process};

fn main() {
    // Use args instead of args_os (which would accept utf-8 strings), because it returs Strings,
    // which are easier to work with
    let args = env::args().collect::<Vec<String>>(); // collect is one of those functions that need explicit type
                                                     // annotations

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // we are using if-let, because the () in case of success is irrelevant for us
    if let Err(e) = run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
}
