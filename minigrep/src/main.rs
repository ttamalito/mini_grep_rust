use std::{env, error::Error, fs, process};

fn main() {
    // collect all the command line arguments for the program
    let args: Vec<String> = env::args().collect();
    //dbg!(args); // use the deug macro to print the arguments
    let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
        println!("Problem Parsing arguments: {err}");
        process::exit(1);
    });

    // call the run function
    if let Err(e) = minigrep::run(&config) {
        println!("Application Error: {e}");
        process::exit(1);
    }
    
} // end of main



