use std::{io, process};

struct Guess {
    letter: String,
}

struct Life {
    n_life: i32,
}

fn input_form() -> Result<Guess, String> {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("ssss");
    println!("You typed {}", input.trim());

    let str_len: usize = input.trim().len();   

    match str_len {
        1 => Ok(
            Guess {
                letter: input.trim().to_string()
            }
        ),
        _ if str_len > 1 => {
            Err("You entered more than one character".to_string())
        },
        _ => Err("You entered nothing".to_string()),
    }
}

fn main() {
    let word: &str = "tempe";
    let mut current_life: Life = Life {
        n_life: 3,
    };
    let mut saved_guess: String = String::new();

    println!("Life remaining: {}", &current_life.n_life);
    println!("Input a letter:");
    
    loop {
        let guesses: Result<Guess, String> = input_form();

        match guesses {
            Ok(guess) => {
                if !saved_guess.contains(&guess.letter) {
                    if word.contains(&guess.letter) {
                        saved_guess.push_str(&guess.letter);
                        println!("Right letter");
                        println!("Life remaining: {}", &current_life.n_life);
                    } else {
                        current_life.n_life -= 1;
                        saved_guess.push_str(&guess.letter);
                        println!("Wrong letter");
                        println!("Life remaining: {}", &current_life.n_life);
                    }
                } else {
                    println!("You already guessed that letter");
                    println!("Life remaining: {}",&current_life.n_life);
                }

                if current_life.n_life == 0 {
                    println!("Game over");
                    println!("The word is: {}", &word);
                    process::exit(0);
                }
            }

            Err(e) => {
                println!("{}", e)
            }
        }
    }
}
