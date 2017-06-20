use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rand::Rng;

const DEFAULT_WORDS_FILENAME: &str = "words.txt";

pub struct Dictionary {
    pub words: Vec<String>
}

impl Dictionary {
    pub fn new() -> Dictionary {
        let mut dictionary = Dictionary { words: Vec::new() };

        dictionary.load(DEFAULT_WORDS_FILENAME);

        dictionary
    }

    pub fn get_random_word(&self) -> String {
        ::rand::thread_rng().choose(&self.words).unwrap().to_owned()
    }

    pub fn load(&mut self, file_name: &str) {
        let path = Path::new(file_name);
        let display = path.display();

        let result = File::open(&path);

        match result {
            Ok(mut file) => {
                let mut s = String::new();
                match file.read_to_string(&mut s) {
                    Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
                    Ok(_) => self.words = s.lines().map(|x| x.to_string()).collect()
                }
            }
            Err(why) => panic!("Couldn't open {}: {}", display, why.description())
        }
    }
}

#[cfg(test)]
mod test {
    use dictionary::Dictionary;
    use dictionary::DEFAULT_WORDS_FILENAME;

    #[test]
    fn test_open_words_file_success() {
        let mut dictionary = Dictionary::new();
        dictionary.load(DEFAULT_WORDS_FILENAME);
    }

    #[test]
    #[should_panic]
    fn test_open_words_file_failure() {
        let mut dictionary = Dictionary::new();
        dictionary.load("afilethatdoesntexist.txt");
    }
}
