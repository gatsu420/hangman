use std::{io, process};
use rand::Rng;

struct Guess {
    letter: String,
}

struct Life {
    n_life: i32,
}

fn input_form() -> Result<Guess, String> {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("You must provide input");
    println!("You typed '{}'", input.trim());

    let str_len: usize = input.trim().len();   

    match str_len {
        1 => Ok(
            Guess {
                letter: input.trim().to_string()
            }
        ),
        _ if str_len > 1 => {
            Err("You entered more than one character. Try again.".to_string())
        },
        _ => Err("You entered nothing".to_string()),
    }
}

fn choose_word() -> String {
    let choices: [&str; 5] = [
        "kambing", "kelinci", "jerapah", "harimau", "beruang"
    ];
    
    let mut rng = rand::thread_rng();
    let i: usize = rng.gen_range(0..choices.len());

    choices[i].to_string()
}

fn main() {
    let word: String = choose_word();
    
    let mut current_life: Life = Life {
        n_life: 3,
    };
    let mut saved_guess: String = String::new();
    let mut correct_guess: String = String::new();    
    
    println!("Life remaining: {}", current_life.n_life);
    println!("Input a letter:");

    for _ in word.chars() {
        if saved_guess.is_empty() {
            print!("[_] ");
        }
    }
    println!();
        
    loop {
        let guesses: Result<Guess, String> = input_form();

        match guesses {
            Ok(guess) => {
                if !saved_guess.contains(&guess.letter) {
                    if word.contains(&guess.letter) {
                        saved_guess.push_str(&guess.letter);
                        correct_guess.push_str(&guess.letter);
                        println!("Right letter");
                    } else {
                        current_life.n_life -= 1;
                        saved_guess.push_str(&guess.letter);
                        println!("Wrong letter");
                    }
                } else {
                    println!("You already guessed that letter");
                }

                println!("Life remaining: {}", current_life.n_life);

                let mut unique_string_word: String = String::new();
                for c in word.chars() {
                    if !unique_string_word.contains(c) {
                        unique_string_word.push(c);
                    }
                }

                let mut similarity_count: usize = 0;
                for c in unique_string_word.chars() {
                    if correct_guess.contains(c) {
                        similarity_count += 1;
                    }
                }

                if current_life.n_life == 0 {
                    println!("Game over. The word is: {}", word);
                    process::exit(0);
                } else if similarity_count == unique_string_word.len() {
                    println!("Congratulations");
                    process::exit(0);
                }
            }
            
            Err(e) => {
                println!("{}", e)
            }
        }

        println!("Input a letter:");
        for c in word.chars() {
            if saved_guess.is_empty() {
                print!("[_] ");
            } else if saved_guess.contains(c) {
                print!("[{}] ", c);
            } else if !saved_guess.contains(c) {
                print!("[_] ");
            }
        }
        println!();
    }
}
