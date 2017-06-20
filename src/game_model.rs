const DEFAULT_LIVES: u32 = 10;

pub struct GameModel {
    pub random_word: String,
    pub guesses: Vec<char>,
}

impl GameModel {
    pub fn new(word: String) -> GameModel {
        GameModel { random_word: word, guesses: Vec::new() }
    }

    pub fn submit_guess(&mut self, guess: char) -> bool {
        self.guesses.push(guess);

        self.random_word.contains(guess)
    }

    pub fn has_won(&self) -> bool {
        self.random_word.chars().all(|letter| self.guesses.contains(&letter))
    }

    pub fn has_lost(&self) -> bool {
        self.num_lives_left() == 0
    }

    pub fn num_lives_left(&self) -> u32 {
        self.guesses.iter().fold(DEFAULT_LIVES, |sum, &letter|
            if self.random_word.contains(letter) { sum } else { sum - 1 }
        )
    }
}

#[cfg(test)]
mod test {
    use game_model::GameModel;

    #[test]
    fn test_correct_guess() {
        let mut g = GameModel::new("ok".to_string());
        let start_lives = g.num_lives_left();

        assert!(g.submit_guess('o') == true);

        assert!(g.num_lives_left() == start_lives);
    }

    #[test]
    fn test_incorrect_guess() {
        let mut g = GameModel::new("ok".to_string());
        let start_lives = g.num_lives_left();

        assert!(g.submit_guess('z') == false);

        assert!(g.num_lives_left() == (start_lives - 1));
    }

    #[test]
    fn test_win() {
        let mut g = GameModel::new("ok".to_string());

        assert!(g.submit_guess('o') == true);
        assert!(g.submit_guess('k') == true);

        assert!(g.has_won() == true);
        assert!(g.has_lost() == false);
    }

    #[test]
    fn test_loss() {
        let mut g = GameModel::new("ok".to_string());
        let num_lives = g.num_lives_left();

        for _ in 0..num_lives {
            g.submit_guess('z');
        }

        assert!(g.has_won() == false);
        assert!(g.has_lost() == true);
    }
}
