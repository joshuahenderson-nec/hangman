const DEFAULT_LIVES: u32 = 10;

pub struct GameModel {
    pub random_word: String,
    pub guesses: Vec<char>,
}

impl GameModel {
    pub fn new(word: String) -> GameModel {
        GameModel { random_word: word, guesses: Vec::new() }
    }

    pub fn take_guess(&mut self, guess: char) -> bool {
        self.guesses.push(guess);

        self.random_word.contains(guess)
    }

    pub fn has_won(&self) -> bool {
        let mut count = 0;

        for g in &self.guesses {
            count += self.random_word.matches(*g).count();
        }

        count == self.random_word.len()
    }

    pub fn has_lost(&self) -> bool {
        self.num_lives_left() == 0
    }

    pub fn num_lives_left(&self) -> u32 {
        let mut count = 0;

        for g in &self.guesses {
            if self.random_word.matches(*g).count() == 0 {
                count += 1;
            }
        }

        if count > DEFAULT_LIVES {
            0
        } else {
            DEFAULT_LIVES - count
        }
    }
}
