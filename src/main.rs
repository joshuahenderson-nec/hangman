extern crate rand;

mod game_model;
mod game_view;
mod game_controller;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rand::Rng;

const DEFAULT_WORDS_FILENAME: &str = "words.txt";

fn main() {
    let mut word = "hello".to_string();
    let result = get_words(DEFAULT_WORDS_FILENAME);

    match result {
        Ok(words) => word = rand::thread_rng().choose(&words).unwrap().to_owned(),
        Err(e) => println!("{}", e)
    }

    let mut g = game_model::GameModel::new(word);

    game_controller::play(&mut g);
}

fn get_words(file_name: &str) -> Result<Vec<String>, String> {
    let path = Path::new(file_name);
    let display = path.display();

    let result = File::open(&path);

    match result {
        Ok(mut file) => {
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => Err(format!("Couldn't read {}: {}", display, why.description())),
                Ok(_) => Ok(s.lines().map(|x| x.to_string()).collect())
            }
        }
        Err(why) => Err(format!("Couldn't open {}: {}", display, why.description()))
    }
}

#[cfg(test)]
mod test {
    use get_words;
    use DEFAULT_WORDS_FILENAME;

    #[test]
    fn test_open_words_file_success() {
        let result = get_words(DEFAULT_WORDS_FILENAME);

        assert!(result.is_ok());
    }

    #[test]
    fn test_open_words_file_failure() {
        let result = get_words("afilethatdoesntexist.txt");

        assert!(result.is_err());
    }
}
