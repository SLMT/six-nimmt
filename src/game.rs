
use std::collections::VecDeque;

use rand::{thread_rng, Rng};

use card::Card;
use player::Player;

pub struct Game {
    table: [VecDeque<Card>; 4],
    players: Vec<Player>
}

impl Game {
    pub fn new(player_count: u32)  {
        let card_count = player_count * 10 + 4;
        let mut cards = create_cards(card_count);
        shuffle(&mut cards);
        println!("{:?}", cards);
    }
}

fn create_cards(card_count: u32) -> Vec<Card> {
    let mut cards = Vec::new();

    for number in 1 .. card_count + 1 {
        cards.push(Card::new(number))
    }

    cards
}

fn shuffle(cards: &mut Vec<Card>) {
    let mut rng = thread_rng();
    rng.shuffle(cards);
}