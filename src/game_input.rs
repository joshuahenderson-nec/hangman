use std;
use game_view::display_input_error;

pub fn get_valid_user_input_blocking() -> char {
    loop {
        let mut input = String::new();
        let result: std::io::Result<usize> = std::io::stdin().read_line(&mut input);

        match result {
            Ok(_) => {
                let validated_input = match validate_input(input) {
                    Ok(letter) => letter,
                    Err(_) => {
                        display_input_error();
                        continue
                    }
                };

                validated_input
            }
            Err(_) => {
                display_input_error();
                continue
            }
        };
    }
}

fn validate_input(input: String) -> Result<char, ()> {
    if input.len() != 2 { // 2 = letter + carrage return
        return Err(());
    }

    // Ok to unwrap as we've assured above that length is more than one
    let character = input.chars().nth(0).unwrap();

    if !character.is_alphabetic() {
        return Err(());
    }

    Ok(character)
}

#[cfg(test)]
mod test {
    use game_input::validate_input;

    #[test]
    fn test_valid_input() {
        let result = validate_input("a\n".to_string());

        assert!(result.is_ok());
        assert!(result.unwrap() == 'a');
    }

    #[test]
    fn test_invalid_input_more_than_one_letter() {
        let result = validate_input("abc\n".to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_input_number() {
        let result = validate_input("1\n".to_string());

        assert!(result.is_err());
    }
}
