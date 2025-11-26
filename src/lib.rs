use std::{fs, error::Error};
use simply_colored::*;

pub enum Commands {
    Cat,
}

pub fn get_command(command_arg: &str) -> Result<Commands, String> {
    match command_arg {
        "cat" => Ok(Commands::Cat),
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

pub fn run_cat(args: Vec<String>) -> Result<(), Box<dyn Error>> {

    let arg_count = args.len();
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

