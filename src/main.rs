mod game_model;
mod game_view;
mod game_controller;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let mut g = game_model::GameModel::new("hello".to_string()); // load word from file

    game_controller::play(&mut g);

    /*
    let words = get_words();

    for word in words {
        println!("word = {}", word);
    }
    */
}

/*
fn get_words() -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let path = Path::new("words.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => {
            result = s.split('\n').map(|x| x.to_string()).collect();

            match result.last() {
                Some(x) => {
                    if x.len() == 0 {
                        result.pop();
                    }
                }
                None => ()
            }
        }
    }

    result
}
*/

#[cfg(test)]
mod test {
    use game_model;

    #[test]
    fn test_model_correct_guess() {
        let mut g = game_model::GameModel::new("ok".to_string());
        let start_lives = g.num_lives_left();

        assert!(g.take_guess('o') == true);

        assert!(g.num_lives_left() == start_lives);
    }

    #[test]
    fn test_model_incorrect_guess() {
        let mut g = game_model::GameModel::new("ok".to_string());
        let start_lives = g.num_lives_left();

        assert!(g.take_guess('z') == false);

        assert!(g.num_lives_left() == (start_lives - 1));
    }

    #[test]
    fn test_model_win() {
        let mut g = game_model::GameModel::new("ok".to_string());

        assert!(g.take_guess('o') == true);
        assert!(g.take_guess('k') == true);

        assert!(g.has_won() == true);
        assert!(g.has_lost() == false);
    }

    #[test]
    fn test_model_loss() {
        let mut g = game_model::GameModel::new("ok".to_string());
        let num_lives = g.num_lives_left();

        for _ in 0..num_lives {
            g.take_guess('z');
        }

        assert!(g.has_won() == false);
        assert!(g.has_lost() == true);
    }
}
