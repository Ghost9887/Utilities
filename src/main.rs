use std::{env, fs, process};
use utilities::{get_command, Commands};

fn main() {
    
    //store the given args
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No arguments passed.");
        process::exit(1);
    }

    //find what command to run
    let command = match get_command(&args[1]) {
        Ok(command) => command,
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        } 
    };

    //execute the command
    process_command(command, args);
}

fn process_command(command: Commands, args: Vec<String>) {
    println!("processing the command");
}
