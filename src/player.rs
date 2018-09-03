
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
}