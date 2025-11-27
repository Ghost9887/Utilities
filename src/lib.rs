use std::{fs, error::Error};
use simply_colored::*;
use glob::glob;

pub enum Commands {
    Cat,
    LS,
}

pub fn get_command(command_arg: &str) -> Result<Commands, String> {
    match command_arg {
        "cat" => Ok(Commands::Cat),
        "ls" => Ok(Commands::LS),
        _ => Err(format!("Uknown command: '{}'", command_arg))
    }
}

pub fn check_cat_conditions(args: &Vec<String>) -> Result<(), String> {
    match args.len() {
        //range pattern
         3.. => Ok(()),
         _ => Err(format!("Invlaid number of arguments type 'cat -h' for help")), 
    }
}

pub fn check_ls_conditions(args: &Vec<String>) -> Result<(), String> {
    match args.len() {
        2..4 => Ok(()),
        _ => {
            Err(format!("Invalid number of arguments type 'ls -h' for help"))
        }
    }
}

pub fn run_ls(args: Vec<String>) -> Result<(), Box<dyn Error>> {

    let mut show_hidden_files = false;
    let mut show_total = false;

    if args.len() > 2{
        let second_argument = &args[2];

        if *second_argument == String::from("-h") {
            println!("\
Usage: ls -[arguments]
Arguments:
[-a] => list all file including hidden files
[-t] => show the total amount of files
[-ta] => both of the above get executed");
            return Ok(());
        }

        if args.len() > 2 {
            match second_argument.as_str() {
                "-a" => show_hidden_files = true,
                "-t" => show_total = true,
                "-ta" => {
                    show_hidden_files = true;
                    show_total = true;
                },
                _ => {
                    return Err("Argument not found type 'ls -h' for help".into());
                },
            }
        }
    }

    let mut index = 0;
    let mut amount = 0;
    for entry in glob("*")? {
        let path = entry?;
        let file_name = path.file_name().unwrap().to_string_lossy();
        if  file_name.starts_with('.') {
            if show_hidden_files {
                print!("{RED}{}{RESET}  ", file_name);
            }else {
                index += 1;
                continue;
            }
        }
        else if file_name.contains('.') {
            print!("{}  ", file_name);
        }
        else {
            print!("{BLUE}{}{RESET}  ", file_name);
        }
        if index % 9 == 0 && index > 1{
            println!("");
        }
        index += 1;
        amount +=1;
    }

    if show_total {
        println!("");
        println!("Entries found: {amount}");
        return Ok(());
    }

    println!("");
    Ok(())

}

pub fn run_cat(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let second_argument = &args[2];

    if *second_argument == String::from("-h") {
        println!("\
Usage: cat 'file' -[arguments] 
Arguments: 
[-c] 'red' => colour of output
[-b] => bold text
[-i] => italic text

Colours:
red
blue
green
yellow
black
white
cyan");
    return Ok(());
    }
    let mut contents = fs::read_to_string(second_argument)?;
    
    let mut index = 3;

    while index < args.len(){
        match args[index].as_str() {
            "-c" => {
                if index + 1 < args.len(){
                    index += 1;
                    let colour = &args[index].to_uppercase();
                    match colour.as_str() {
                        "GREEN" => {
                            contents = format!("{GREEN}{contents}");
                        },
                        "RED" => {
                            contents = format!("{RED}{contents}");
                        },
                        "BLUE" => {
                            contents = format!("{BLUE}{contents}");
                        },
                        "CYAN" => {
                            contents = format!("{CYAN}{contents}");
                        },
                        "YELLOW" => {
                            contents = format!("{YELLOW}{contents}");
                        },
                        "WHITE" => {
                            contents = format!("{WHITE}{contents}");
                        },
                        "BLACK" => {
                            contents = format!("{BLACK}{contents}");
                        },
                        _ => {
                            return Err("Invalid colour type 'cat' -h for help".into());
                        }
                    }
                }else{
                    return Err("Expected colour afte '-c' argument type cat -h for help".into());
                }
            },
            "-b" => {
                contents = format!("{BOLD}{contents}");
            },
            "-i" => {
                contents = format!("{ITALIC}{contents}");
            },
            _ => {
                return Err("Argument not found type cat '-h' for help".into());
            }
        }
        index += 1;
    }   
    println!("{contents}");
    Ok(())
}

