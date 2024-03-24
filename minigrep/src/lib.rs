use std::{error::Error, fs, env};

pub fn run<'a>(config: &'a Config) -> Result<Vec<String>, Box<dyn Error>> {
    let query = &config.query; // the string that we have to look for in the file
    let file_path = &config.file_path; // the file to the path
    println!("Looking for String: {}", query);
    println!("In file: {}", file_path);

    let content = fs::read_to_string(file_path)?;

    // print the content
    //println!("Te content of the file is: {}", content);


    if config.ignore_case {
        Result::Ok(search_case_insensitive(query, content))
    } else {
        Result::Ok(search(query, content))
    }
}



pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool
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
            file_path: file_path.clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok()
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

// define the search function
pub fn search<'a>(query: &str, content: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            // add it to the result
            result.push(line.to_string());
        }
    } // end of for loop
    // return the vector
    result

} // end of search

pub fn search_case_insensitive(query: &str, content: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    // make the query lowercase
    let query = &query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(query) {
            // add it to the result
            result.push(line.to_string());
        }
    } // end of for loop
    // return the vector
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
        Rust:
safe, fast, productive.
        Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content.to_string()));
    } // end one_result()

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
            search_case_insensitive(query, contents.to_string())
        );
    }
}