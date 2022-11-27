use crate::gamer::Gamer;

type Word = Vec<char>;

pub struct Game<'a> {
    gamer: &'a Gamer,
    secret_word: Word,
    public_word: Word,
    max_errors: usize,
}

impl<'a> Game<'a> {
    pub fn new(secret_word: String, gamer: &Gamer) -> Game {
        let public_word = "_".repeat(secret_word.chars().count());
        Game{secret_word: to_word(&secret_word), gamer: gamer, public_word: to_word(&public_word), max_errors: 7}
    }

    pub fn max_errors(&self) -> usize {
        self.max_errors
    }

    pub fn public_word(&self) -> String {
        self.public_word.iter().collect()
    }

    pub fn guess(&mut self, guess: char) {
        if self.failed() {
            return
        }
        if !self.guess_apply(guess) {
            self.max_errors -= 1;
        }
    }
    
    pub fn failed(&self) -> bool {
        self.max_errors == 0
    }

    pub fn succes(&self) -> bool {
        self.secret_word == self.public_word
    }

    fn guess_apply(&mut self, guess: char) -> bool {
        let secret = self.secret_word.clone();
        let mut applied = false;
        secret.iter().enumerate().filter_map(|(i, val)| {
        if val == &guess {
            Some(i)
        } else {
            None
        }  
        }).for_each(|i| {
            applied = true;
            self.public_word[i] = guess;
        });
        applied
    }
    
}

fn to_word(from: &String) -> Word {
    from.chars().collect()
}

mod test {
    use crate::gamer;

    #[test]
    fn missing_guess() {
        let player = gamer::login("test");
        let mut game = super::Game::new("ябълка".to_string(), &player);
        let public_before = game.public_word();
        game.guess('й');
        game.guess('j');
        game.guess('s');
        game.guess('_');
        game.guess('4');
        game.guess('!');
        assert!(!game.failed());
        game.guess('w');
        assert!(!game.succes());
        assert!(game.failed());
        assert_eq!(public_before, game.public_word());
    }
    #[test]
    fn correct_guess() {
        let guess = 'л';
        let player = gamer::login("test");
        let mut game = super::Game::new("люляк".to_string(), &player);
        let public_before = game.public_word();
        game.guess(guess);
        assert_eq!(public_before.chars().count(), game.public_word().chars().count());
        assert_ne!(public_before, game.public_word());
        assert_eq!(Some(0), game.public_word().find(guess));
        assert_eq!(Some(3), game.public_word().rfind(guess));
        
    }
}
