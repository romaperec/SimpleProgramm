use std::{collections::HashMap, io};
use colored::*;


struct User {
    nickname: String,
    stats: HashMap<String, i32>
}


fn main() {
    println!("{}", "Welcome to Rust Programm".red().bold());
    println!("{}", "Created by romaperec".green().bold());
    println!("{}", "---------------------\n\n".green().bold());

    println!("{}", "Info | Enter your name:".red());

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("msg");

    let mut user = User {
        nickname: name.to_string(),
        stats: HashMap::new(),
    };

    user.stats.insert("money".to_string(), 1000);
    user.stats.insert("level".to_string(), 1);

    
    loop {
        println!("Good morning, {}!", user.nickname.trim().red().bold());
        println!("{}", "Menu".red().bold());
        println!("{}", "Help commands - -h.");

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("msg");

        let command = command.trim();

        if command == "-h" {
            print_help();
        } else {
            println!("{}", "Error | Unknown command. Type -h for help.".red().bold());
        }
    }
}

fn print_help() {
    println!("{}", "Help menu:".blue().bold());
}