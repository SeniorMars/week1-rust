// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

// const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    let mut counter = 5;
    let mut guesses: Vec<char> = Vec::new();
    let mut known_word_so_far: Vec<char> = Vec::new();
    let mut result = false;
    for _chr in secret_word_chars.iter() {
        known_word_so_far.push('-');
    }
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    println!("Welcome to CS110L Hangman!");

    while counter > 0 {
        print!("The word so fair is: ");
        for current_character in known_word_so_far.iter() {
            print!("{} ", current_character);
        }
        println!();

        // prints out what letters so far
        print!("You have guessed the following letters: ");
        for current_character in guesses.iter() {
            print!("{} ", current_character);
        }
        println!();
        println!("You have {} guesses left", counter);
        print!("Please guess a letter: ");

        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");

        // error checking
        let mut c = guess.chars().next().unwrap();
        while !c.is_alphabetic() || guesses.contains(&c) {
            print!("Enter a correct and new letter: ");
            io::stdout().flush().expect("Error flushing stdout.");
            guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Error reading line.");
            c = guess.chars().next().unwrap();
        }

        guesses.push(c); // update the vector
        let mut found = false;

        for (i, chr) in secret_word_chars.iter().enumerate() {
            if c == *chr {
                known_word_so_far[i] = *chr;
                found = true;
            }
        }

        if !found {
            counter -= 1;
        }

        if known_word_so_far == secret_word_chars {
            result = true;
            break;
        }
        println!("");
    }

    if result {
        println!(
            "Congratulations you guessed the secret word: {}!",
            secret_word
        );
    } else {
        println!("Sorry, you ran out of guesses!");
    }
}
