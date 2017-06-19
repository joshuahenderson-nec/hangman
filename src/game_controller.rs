use game_model;
use game_view;

pub fn play(g: &mut game_model::GameModel) {
    game_view::display_start_hint(g.random_word.len());

    loop {
        if !take_turn(g) {
            continue;
        }

        game_view::display_word_guessed_progress(&g);

        if g.has_won() {
            game_view::display_game_won();
            break;
        } else if g.has_lost() {
            game_view::display_game_lost(&g.random_word);
            break;
        }
    }
}

fn take_turn(g: &mut game_model::GameModel) -> bool {
    match game_view::get_char_from_user() {
        Some(c) => {
            let result = g.take_guess(c);

            match result {
                true => game_view::display_guess_correct(),
                false => game_view::display_guess_incorrect(g.num_lives_left())
            }

            true
        }
        None => false
    }
}
