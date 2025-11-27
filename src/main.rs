use std::{env, process};
use utilities::{
    get_command, 
    check_cat_conditions, 
    run_cat, 
    check_ls_conditions,
    run_ls,
    Commands,
};

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
    match command {
        Commands::Cat => {
            match check_cat_conditions(&args) {
                Ok(()) => {
                    if let Err(e) = run_cat(args) {
                        eprintln!("{e}");
                        process::exit(1);
                    }
                },
                Err(error) => {
                    eprintln!("{error}");
                    process::exit(1);
                }
            } 
        },
        Commands::LS => {
            match check_ls_conditions(&args){
                Ok(()) => {
                    if let Err(e) = run_ls(args){
                        eprintln!("{e}");
                        process::exit(1);
                    }
                },
                Err(error) => {
                    eprintln!("{error}");
                    process::exit(1);
                }
            }
        },
    }
}
