use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let query = config.query; // the string that we have to look for in the file
    let file_path = config.file_path; // the file to the path
    println!("Looking for String: {}", query);
    println!("In file: {}", file_path);

    let content = fs::read_to_string(file_path)?;

    // print the content
    println!("Te content of the file is: {}", content);

    Result::Ok(())
}



pub struct Config {
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

    // create another "constructor" for the Config struct
    pub fn build(args: &[String])-> Result<Config, &str> {
        // check if there are at least three elements in the args array
        if args.len() < 3 {
            return Err("Not enough CLI arguments");
        }
        let query = &args[1];
        let file_path = &args[2];
    
        Result::Ok(        Config {
            query: query.clone(),
            file_path: file_path.clone()
        })
    }
}