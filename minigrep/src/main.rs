use std::{env, fs};

fn main() {
    // collect all the command line arguments for the program
    let args: Vec<String> = env::args().collect();
    //dbg!(args); // use the deug macro to print the arguments
    let query = &args[1]; // the string that we have to look for in the file
    let file_path = &args[2]; // the file to the path
    println!("Looking for String: {}", query);
    println!("In file: {}", file_path);

    let content = fs::read_to_string(file_path).expect("Should be able to read the given file {}");

    // print the content
    println!("Te content of the file is: {}", content);
}
