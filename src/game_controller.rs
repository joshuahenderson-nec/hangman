use game_model::GameModel;
use game_view::*;
use game_input::get_valid_user_input_blocking;

pub fn play(game: &mut GameModel) {
    display_start_hint(game.random_word.len());

    loop {
        let letter = get_valid_user_input_blocking();

        match game.submit_guess(letter) {
            true => display_guess_correct(),
            false => display_guess_incorrect(game.num_lives_left())
        }

        display_word_guessed_progress(&game.random_word, &game.guesses);

        if game.has_won() {
            display_game_won();
            break;
        } else if game.has_lost() {
            display_game_lost(&game.random_word);
            break;
        }
    }
}
