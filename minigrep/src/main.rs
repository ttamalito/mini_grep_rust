use std::{env, fs};

fn main() {
    // collect all the command line arguments for the program
    let args: Vec<String> = env::args().collect();
    //dbg!(args); // use the deug macro to print the arguments
    let config = parse_config(&args);
    let query = config.query; // the string that we have to look for in the file
    let file_path = config.file_path; // the file to the path
    println!("Looking for String: {}", query);
    println!("In file: {}", file_path);

    let content = fs::read_to_string(file_path).expect("Should be able to read the given file {}");

    // print the content
    println!("Te content of the file is: {}", content);
}


fn parse_config(args: &[String]) -> Config {
    Config::new(args)
}

struct Config {
    query: String,
    file_path: String
}

impl Config {
    // create a constructor for the struct
    fn new(args: &[String])-> Config {
        // check if there are at least three elements in the args array
        if args.len() < 3 {
            panic!("There are not enoughparameters");
        }
        let query = &args[1];
        let file_path = &args[2];
    
        Config {
            query: query.clone(),
            file_path: file_path.clone()
        }
    }
}

