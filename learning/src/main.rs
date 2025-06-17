use std::{collections::HashMap, io, thread, time::Duration};
use colored::*;


struct User {
    nickname: String,
    stats: HashMap<String, i32>
}


fn main() {
    println!("{}", "\n\n---------------------".green().bold());
    println!("{}", "Welcome to Rust Programm".red().bold());
    println!("{}", "Created by romaperec".green().bold());
    println!("{}", "---------------------\n\n".green().bold());

    thread::sleep(Duration::from_secs(2));

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
        } else if command == "-stats" {
            print_user_stats(&user);
        }
        
        else {
            println!("{}", "\n\n---------------------".green().bold());
            println!("{}", "Error | Unknown command. Type -h for help.".red().bold());
            println!("{}", "---------------------\n\n".green().bold());
        }
    }
}

fn print_help() {
    println!("{}", "\n\n---------------------".green().bold());
    println!("{}", "Help menu:".blue().bold());
    println!("{}", "User stats - -stats".green().bold());
    println!("{}", "---------------------\n\n".green().bold());
    thread::sleep(Duration::from_secs(2));
}

fn print_user_stats(user: &User) {
    println!("{}", "\n\n---------------------".green().bold());
    println!("{}", "Stats menu:".blue().bold());
    for (key, value ) in &user.stats {
        println!("{}: {}", key, value.to_string().green());
    }
    println!("{}", "---------------------\n\n".green().bold());
    thread::sleep(Duration::from_secs(2));
}