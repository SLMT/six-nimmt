
use std::io::{self, Write};
use std::str::FromStr;

use game::Game;

const PLAYER_ID: usize = 0;

pub struct UI {
    game: Game
}

impl UI {
    pub fn new(game: Game) -> UI {
        UI {
            game: game
        }
    }

    pub fn run_game_loop(&mut self) {
        println!("A new game starts!\n");
        loop {
            self.show_lines();
            self.show_hand();
            self.select_card();
            self.show_result();
        }
    }

    fn show_lines(&self) {
        println!("Lines on the table:");
        for line in self.game.check_lines() {
            println!("{:?}", line);
        }
        println!("\n");
    }

    fn show_hand(&self) {
        println!("Your hand:");
        let hand = self.game.check_hand(PLAYER_ID).expect("check hand error");
        for card_id in 0 .. hand.len() {
            println!("Card {}: {:?}", card_id, hand[card_id]);
        }
        println!("\n");
    }

    fn select_card(&mut self) {
        let chosen_hand = read_card_choice();
        // TODO: implement
    }

    fn show_result(&mut self) {

    }
}

fn read_card_choice() -> usize {
    print!("Choose a card: ");
    io::stdout().flush().expect("flush error");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("read error");
    
    // XXX: Maybe we should show the error then proceed?
    usize::from_str(&input).expect("parse input error")
}