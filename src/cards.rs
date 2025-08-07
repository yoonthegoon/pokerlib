use crate::card::Card;

pub struct Cards<const N: usize> {
    pub cards: [Card; N],
}

impl<const N: usize> Cards<N> {
    pub fn new(cards: [Card; N]) -> Self {
        if 2 > N || N > 7 {
            panic!("Invalid number of cards: {}", N);
        }
        Self { cards }
    }
}
