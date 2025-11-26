use std::process;

pub enum Commands {
    Cat,
}

pub fn get_command(args: &Vec<String>) -> String {
    println!("{:?}", args);

    let mut command = String::new();

    command = String::from("cat");
    
    command
}

