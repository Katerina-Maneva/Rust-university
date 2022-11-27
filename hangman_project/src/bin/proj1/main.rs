mod user_input;
mod gamer;
mod game;
mod controler;
mod terminal_art;


fn main() {
    let name_player = user_input::prompt_input("What is your name? ... ");
    let gamer = gamer::login(&name_player);

    let secret_word = user_input::prompt_input("Please type a word to be guessed ...");
    let mut game = game::Game::new(secret_word, &gamer);   
    
    controler::play(&mut game);
    match (game.succes(), game.failed()) {
        (true, _) => println!("You have won!"),
        (_, true) => println!("You have lost!"),
        (_, _) => println!("Why have you given up?!"),
    }
}
