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

