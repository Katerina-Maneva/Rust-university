use crate::game::Game;
use crate::user_input;
use crate::terminal_art;


pub fn play(game: &mut Game) {
    while !(game.succes() || game.failed()) {
        println!("{}", game.public_word());
        terminal_art::display_hangman(&game);        
        let guess = user_input::prompt_char("Enter your next guess or <Enter> to give up");
        match guess {
            Some(ch) => game.guess(ch),
            None => break,
        }
    }
}