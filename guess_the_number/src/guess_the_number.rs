use std::rand;
use std::io;

fn read_uint(prompt: &str) -> uint {
    print!("Guess a number between 1 and 10,000: ");

    let line = io::stdin().read_line().ok().expect("failure to read stdin");
    let parsed_num: Option<uint> = line.trim().parse();

    match parsed_num {
        Some(guess) => guess,
        None => {
            println!("Please input a number!");
            read_uint(prompt)
        }
    }
}

fn main() {
    let hidden = rand::random::<uint>() % 10_000;
    let mut guess = read_uint("Guess a number between 1 and 10,000: ");

    while guess != hidden {
        if guess > hidden {
            println!("{} was too high, guess again.", guess);
        } else {
            println!("{} was too low, guess again.", guess);
        }

        guess = read_uint("Enter a number between 1 and 10,000: ");
    }

    println!("Congratulations, you've guessed the number!");
}
