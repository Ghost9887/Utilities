pub enum Commands {
    Cat,
}

pub fn get_command(command_arg: &str) -> Result<Commands, String> {
    println!("{:?}", command_arg);

    match command_arg {
        "cat" => Ok(Commands::Cat),
        _ => Err(format!("Uknown command: '{}'", command_arg))
    }
}

pub fn check_cat_conditions(args: &Vec<String>) -> Result<(), String> {
    match args.len() {
        3 => Ok(()),
        _ => Err(format!("Invalid number of arguments type 'cat -h' for help")),
    }
}

pub fn run_cat(args: &Vec<String>){
    unimplemented!();
}

