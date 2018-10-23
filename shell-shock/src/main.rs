use std::io;
use std::io::prelude::*;
use std::process::Command;

fn main() {
    println!("Starting Bored Again Shell");

    loop {
        print!("> ");
        io::stdout().flush().expect("failed to flush");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Grab user input");
        tokens_to_arguments(user_input);
    }
}

fn tokens_to_arguments(user_input: String) {
    let commands_input: Vec<String> = user_input.split(";").map(|s| s.to_string()).collect();
    for command_input in commands_input {
        let mut tokens = command_input.split_whitespace();
        let cmd: String = tokens.next().unwrap().to_string();
        let args: Vec<String> = tokens.map(|s| s.to_string()).collect();
        Command::new(cmd).args(args).output().map(|output| {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }).map_err(|_|{
            println!("OOPSIE WOOPSIE!! UwU We made a fucky wucky!! A wittle fucko boingo! The code monkeys at our headquaters are working VEWY HAWD to fix this.")
        });
    }
}