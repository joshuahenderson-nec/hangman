pub fn display_input_error() {
    println!("Please enter a single letter");
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

pub fn display_word_guessed_progress(word: &String, guesses: &Vec<char>) {
    for c in word.chars() {
        if guesses.contains(&c) {
            print!("{}", c);
        } else {
            print!("*");
        }
    }
    println!("");
}
