
use card::Card;

pub struct Player {
    hand: Vec<Card>
}

impl Player {
    pub fn new(cards: Vec<Card>) -> Player {
        Player {
            hand: cards
        }
    }

    pub fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }

    pub fn take_card(&mut self, card_id: usize) -> Option<Card> {
        if card_id < self.hand.len() {
            Some(self.hand.remove(card_id))
        } else {
            None
        }
    }
}