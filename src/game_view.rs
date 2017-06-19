use std;
use game_model;

pub fn get_char_from_user() -> Option<char> {
    let mut buffer = String::new();
    let result: std::io::Result<usize> = std::io::stdin().read_line(&mut buffer);

    // TODO give feedback, have input requirements

    match result {
        Ok(_) => buffer.chars().nth(0),
        Err(_) => None
    }
}

pub fn display_start_hint(word_length: usize) {
    println!("Random word has {} characters, try to guess it", word_length);
}

pub fn display_guess_correct() {
    println!("Correct guess");
}

pub fn display_guess_incorrect(lives: u32) {
    println!("Incorrect guess. Chances left: {}", lives);
}

pub fn display_game_won() {
    println!("Congrats you won!");
}

pub fn display_game_lost(word: &String) {
    println!("You lost! The word was: {}", word)
}

pub fn display_word_guessed_progress(g: &game_model::GameModel) {
    for c in g.random_word.chars() {
        if g.guesses.contains(&c) {
            print!("{}", c);
        } else {
            print!("*");
        }
    }
    println!("");
}
