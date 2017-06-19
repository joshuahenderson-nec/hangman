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
    let input = match game_view::get_input_from_user() {
        Ok(x) => x,
        Err(_) => return false
    };

    let validated_input = match validate_input(input) {
        Ok(x) => x,
        Err(e) => {
            game_view::display_error(e);
            return false
        }
    };

    match g.take_guess(validated_input) {
        true => game_view::display_guess_correct(),
        false => game_view::display_guess_incorrect(g.num_lives_left())
    }

    true
}

fn validate_input(input: String) -> Result<char, String> {
    if input.len() != 2 { // 2 = letter + carrage return
        return Err("Please enter a single letter".to_string());
    }

    // Ok to unwrap as we've assured above that length is more than one
    let c = input.chars().nth(0).unwrap();

    if !c.is_alphabetic() {
        return Err("Please enter a letter and not a number".to_string());
    }

    Ok(c)
}

#[cfg(test)]
mod test {
    use game_controller;

    #[test]
    fn test_valid_input() {
        let result = game_controller::validate_input("a\n".to_string());

        assert!(result.is_ok());
        assert!(result.unwrap() == 'a');
    }

    #[test]
    fn test_invalid_input_more_than_one_letter() {
        let result = game_controller::validate_input("abc\n".to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_input_number() {
        let result = game_controller::validate_input("1\n".to_string());

        assert!(result.is_err());
    }
}
