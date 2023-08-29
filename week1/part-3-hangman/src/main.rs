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

const NUM_INCORRECT_GUESSES: u32 = 5;
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
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    // Your code here! :)
    println!("Welcome to CS110L Hangman!");
    let mut so_far_word_chars: Vec<char> = Vec::new();
    let mut gussed_letters: Vec<char> = Vec::new();
    let mut rest_chances = NUM_INCORRECT_GUESSES;
    for i in &secret_word_chars {
        so_far_word_chars.push('-');
    }

    loop {
        print!("The word so far is ");
        for i in &so_far_word_chars {
            print!("{}", i);
        }
        print!("\nYou have guessed the following letters:");
        for i in &gussed_letters {
            print!("{}", i);
        }
        println!("\nYou have {} guesses left", rest_chances);
        let input_letter: char = get_letter();
        gussed_letters.push(input_letter);
        let mut letter_is_not_in = true;

        for i in 0..secret_word.len() {
            if input_letter != secret_word_chars[i] {
                continue;
            } else {
                so_far_word_chars[i] = input_letter;
                letter_is_not_in = false;
            }
        }
        if letter_is_not_in {
            println!("Sorry, that letter is not in the word\n");
            rest_chances -= 1;
        }
        if rest_chances == 0 {
            print!("Sorry, you ran out of guesses!");
            break;
        }
    }
}

fn get_letter() -> char {
    println!("Please guess a letter:  ");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string);
    input_string.chars().next().unwrap()
}
