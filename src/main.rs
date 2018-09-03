extern crate rand;

mod card;
mod player;
mod game;

use game::Game;

fn main() {
    Game::new(10);
}
