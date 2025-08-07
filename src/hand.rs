use crate::card::Card;

pub struct Hand {
    pub cards: [Card; 5],
}

impl Hand {
    pub fn new(cards: [Card; 5]) -> Self {
        Self { cards }
    }
}
