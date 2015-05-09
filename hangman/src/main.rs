use std::io;

fn mask_word<'a>(mask: &'a mut String, hidden: &str, letters: &Vec<char>) -> &'a mut String {
    mask.clear();

    for c in hidden.chars() {
        if letters.contains(&c) {
            mask.push(c);
        } else {
            mask.push('_');
        }
    }

    mask
}

fn main() {
    println!("Welcome to Hangman!");

    let mut guessed_letters: Vec<char> = Vec::new();
    let hidden = "rustacean";
    let mut mask = String::new();

    let mut chances_remaining = 8;

    while chances_remaining >= 0 {
        println!("chances: {}", chances_remaining);
        println!("word: {}", mask_word(&mut mask, hidden, &guessed_letters));

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line");

        let guess = guess.trim();

        if guess.chars().count() == 1 {
            let letter = match guess.chars().next() {
                Some(char) => char,
                None => ' ',
            };

            if !guessed_letters.contains(&letter) {
                let occurrences = hidden.chars()
                    .filter( |&c| c == letter).count();

                if occurrences > 0 {
                    println!("There are {} {}'s in the word.",
                             occurrences, letter);
                } else {
                    println!("Sorry, there are no {}'s in the word.", letter);
                    chances_remaining -= 1;
                }

                guessed_letters.push(letter);
            } else {
                println!("You've already guessed the letter {}", letter);
            }

        } else {
            if guess == hidden {
                println!("You've guessed the word!");
            } else {
                println!("Sorry, the word was {}", hidden);
            }

            break;
        }
    }
}
