use crate::game::Game;

pub fn display_hangman(game: &Game) {
    match game.max_errors() {
        7 => {
            println!("------ ");
            println!("|      ");
            println!("|      ");
            println!("|      ");
            println!("|      ");
        }
        6 => {
            println!("------ ");
            println!("|      ");
            println!("|   0  ");
            println!("|      ");
            println!("|      ");
        }
        5 => {
            println!("------ ");
            println!("|      ");
            println!("|   0  ");
            println!("|   |  ");
            println!("|      ");
        }
        4 => {
            println!("------ ");
            println!("|      ");
            println!("|   0  ");
            println!("|  /|  ");
            println!("|      ");
        }
        3 => {
            println!("------ ");
            println!("|      ");
            println!("|   0  ");
            println!("|  /|\\ ");
            println!("|      ");
        }
        2 => {
            println!("------ ");
            println!("|      ");
            println!("|   0  ");
            println!("|  /|\\ ");
            println!("|  /   ");
        }
        1 => {
            println!("------ ");
            println!("|      ");
            println!("|   0  ");
            println!("|  /|\\ ");
            println!("|  / \\ ");
        }
        _ => {}
    }
}
