mod game;
mod utils;
mod playing_field;

use crate::game::Game;

fn get_filename() -> String {
    if USE_EXAMPLE {
        return String::from("example.txt");
    }
    String::from("input.txt")
}

const USE_EXAMPLE: bool = false;

fn main() {
    let mut game = Game::from_file(get_filename());
    game.setup_verbosity(if USE_EXAMPLE { true } else { false });
    game.play();

    println!("Visited spaces: {}", game.get_num_visited());
}
