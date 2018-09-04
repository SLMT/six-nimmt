extern crate rand;

mod card;
mod player;
mod game;
mod ui;

use game::Game;
use ui::UI;

fn main() {
    let game = Game::new(10);
    let mut ui = UI::new(game);
    ui.run_game_loop();
}
