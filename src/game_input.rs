use std;
use game_view::display_input_error;

pub fn get_valid_user_input_blocking() -> char {
    loop {
        let input = get_user_input();

        if is_input_valid(&input) {
            return extract_character(&input);
        }

        display_input_error();
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    let result = std::io::stdin().read_line(&mut input);

    match result {
        Ok(_) => input.trim().to_string(),
        Err(_) => panic!("Failed to get input of any length")
    }
}

fn extract_character(input: &String) -> char {
    debug_assert!(input.len() >= 1);

    // Return the first character the user typed
    input.chars().nth(0).unwrap()
}

fn is_input_valid(input: &String) -> bool {
    if input.len() != 1 {
        return false;
    }

    let character = extract_character(&input);

    if !character.is_alphabetic() {
        return false;
    }

    true
}

#[cfg(test)]
mod test {
    use game_input::is_input_valid;
    use game_input::extract_character;

    #[test]
    fn test_valid_input() {
        assert!(is_input_valid(&"a".to_string()) == true);
    }

    #[test]
    fn test_invalid_input_more_than_one_character() {
        assert!(is_input_valid(&"abc".to_string()) == false);
    }

    #[test]
    fn test_invalid_input_number() {
        assert!(is_input_valid(&"1".to_string()) == false);
    }

    #[test]
    fn test_invalid_input_special_character() {
        assert!(is_input_valid(&"$".to_string()) == false);
    }

    #[test]
    fn test_extract_character_success() {
        let character = extract_character(&"abc".to_string());

        assert!(character == 'a');
    }

    #[test]
    #[should_panic]
    fn test_extract_character_failure() {
        extract_character(&"".to_string());
    }
}
