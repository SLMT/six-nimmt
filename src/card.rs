
#[derive(Debug)]
pub struct Card {
    number: u32,
    head_count: u32
}

impl Card {
    pub fn new(number: u32) -> Card {
        Card {
            number: number,
            head_count: Card::count_heads(number)
        }
    }

    pub fn get_number(&self) -> u32 {
        self.number
    }

    pub fn get_head_count(&self) -> u32 {
        self.head_count
    }

    fn count_heads(number: u32) -> u32 {
        if number == 55 {
            7
        } else if number % 11 == 0 {
            5
        } else if number % 10 == 0 {
            3
        } else if number % 5 == 0 {
            2
        } else {
            1
        }
    }
}