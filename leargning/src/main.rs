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

    println!("{}", "Info | Enter your name: ".red());

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

    println!("Good morning, {}!", user.nickname.trim().red().bold());
    
    println!("\nStats: {}", user.nickname.trim().red().bold());

    for (key, value) in &user.stats {
        println!("{}: {}", key.green(), value.to_string().yellow().bold());
    }
}