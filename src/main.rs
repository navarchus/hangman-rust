use reqwest;
use serde_json::Value;
use std::io;

fn main() {
    let body = reqwest::blocking::get("https://random-word-api.herokuapp.com/word")
        .unwrap()
        .text()
        .unwrap();

    let json_body: Value = serde_json::from_str(&body).unwrap();

    let random_word: String = json_body.as_array().unwrap()[0]
        .as_str()
        .unwrap()
        .trim()
        .to_lowercase()
        .to_string();

    let mut lives = 5;

    let mut guesses: Vec<String> = Vec::new();

    println!("Welcome to hangman!");
    // println!("The secret word is {}", random_word);

    loop {
        if lives <= 0 {
            println!("You Lose!");
            break;
        }

        println!("Lives: {}", lives);
        println!("Guess a letter");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        guess = guess.trim().to_lowercase().to_string();

        if !guesses.contains(&guess) {
            guesses.push(guess.clone());
        }

        let mut counter = 0;
        for char in random_word.chars() {
            if guesses.contains(&char.to_string()) {
                print!("{}", char.to_string());
                counter += 1;
            } else {
                print!(" _ ")
            }
        }
        println!("");
        if !random_word.contains(&guess) {
            println!("You lost a life!");
            lives = lives - 1;
        }
        println!("");

        if counter == random_word.len() {
            println!("You Win!");
            break;
        }
    }
}
