use std::io;

fn letter_already_guessed(letter: char, past_guesses: &Vec<char>) -> bool {
    for previous_guess in past_guesses.iter() {
        if letter == *previous_guess {
            return true;
        }
    }
    false
}

fn main () {
    println!("Let's play Hangman!");

    let hidden_word = "rustacean";

    let mut masked_word = String::with_capacity(hidden_word.len());
    for _ in range(0, hidden_word.len()) {
        masked_word.push('_');
    }

    let mut past_guesses: Vec<char> = Vec::new();

    loop {
        let full_input = io::stdin().read_line().ok().expect("Failed to get input.");
        let input = full_input.trim();

        if input.len() == 1 {
            let guessed_letter = match input.chars().next() {
                Some(letter) => letter,
                None => ' '
            };

            if letter_already_guessed(guessed_letter, &past_guesses) {
                println!("You've already guessed that letter, try again.");
            } else {
                past_guesses.push(guessed_letter);
                println!("You guessed the letter {}.", guessed_letter);
            }
        } else {
            println!("You guessed the word {}.", input);
        }
    }
}
