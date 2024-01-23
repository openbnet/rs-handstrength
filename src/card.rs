#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Suit {
    S,
    H,
    C,
    D,
    A // any *
}

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
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
        };
        write!(f, "{}", suit)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.value, self.suit)
    }
}