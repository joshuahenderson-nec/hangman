extern crate rand;

mod game_model;
mod game_view;
mod game_controller;
mod game_input;
mod dictionary;

use game_model::GameModel;
use game_controller::play;
use dictionary::Dictionary;

fn main() {
    let dictionary = Dictionary::new();
    let random_word = dictionary.get_random_word();

    let mut game = GameModel::new(random_word);

    play(&mut game);
}
