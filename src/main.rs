use std::{env, fs};
use utilities::{get_command};

fn main() {
    
    //store the given args
    let args: Vec<String> = env::args().collect();

    //find what command to run
    let command = get_command(&args);

    println!("{command}");

}
