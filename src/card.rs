#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Suit {
    S,
    H,
    C,
    D,
    A, // any *
    E, // TrainerSuit first suit in cards
    F, //2nd suit
    G, // 3rd suit
    X, // Except E - 3 possible
    Y, // Except E and F - 2 possible
    Z, // Except E, F, and G -4th suit

}

#[derive(Clone, Copy, Debug, Eq, Hash,PartialOrd, PartialEq, Ord)]
pub struct Card {
    pub value: u8,
    pub suit: Suit,
}
use std::fmt;

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let suit = match self {
            Suit::C => "c",
            Suit::D => "d",
            Suit::H => "h",
            Suit::S => "s",
            Suit::A => "*",
            Suit::E => "e",
            Suit::F => "f",
            Suit::G => "g",
            Suit::X => "x",
            Suit::Y => "y",
            Suit::Z => "z",
        };
        write!(f, "{}", suit)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.value, self.suit)
    }
}
impl Default for Card {
    fn default() -> Self {
        Card {
            value: 0,
            suit: Suit::S,
        }
    }
}