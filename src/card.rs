#[derive(Debug, Clone, Copy)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

pub struct Card(u32);
