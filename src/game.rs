
use std::collections::VecDeque;

use rand::{thread_rng, Rng};

use card::Card;
use player::Player;

pub struct Game {
    lines: Vec<VecDeque<Card>>,
    players: Vec<Player>
}

impl Game {
    pub fn new(player_count: u32) -> Game {
        let card_count = player_count * 10 + 4;
        let mut cards = create_cards(card_count);
        shuffle(&mut cards);

        let (hands, lines) = deal(cards, player_count);

        let mut players = Vec::new();
        for hand in hands {
            players.push(Player::new(hand));
        }

        Game {
            lines: lines,
            players: players
        }
    }

    pub fn check_lines(&self) -> &Vec<VecDeque<Card>> {
        &self.lines
    }

    pub fn check_hand(&self, player_id: usize) -> Option<&Vec<Card>> {
        self.players.get(player_id).map(|p| p.get_hand())
    }

    pub fn play_card(&mut self, player_id: usize, card_id: usize) -> bool {
        match self.players.get(player_id) {
            Some(ref player) => {
                match player.take_card(card_id) {
                    Some(card) => {
                        // TODO: put into the line
                        true
                    },
                    None => false
                }
            },
            None => false
        }
    }

    fn put_in_line(&mut self, card: Card) {
        // TODO
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

// Input: Deck, # of Players
// Output: (Players' Hands, Initial Lines on the Table)
fn deal(mut cards: Vec<Card>, player_count: u32) -> (Vec<Vec<Card>>, Vec<VecDeque<Card>>) {
    let mut lines = Vec::new();
    for _ in 0..4 {
        let mut line = VecDeque::new();
        line.push_back(cards.pop().expect("Dealing error"));
        lines.push(line);
    }

    let mut hands = Vec::new();
    for _ in 0 .. player_count {
        let mut hand = Vec::new();

        for _ in 0..10 {
            hand.push(cards.pop().expect("Dealing error"));
        }

        hands.push(hand)
    }

    (hands, lines)
}