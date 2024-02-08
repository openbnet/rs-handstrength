#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Suit {
    S,
    H,
    C,
    D,
    A, // any *
    E, // Equals
    X, // Except

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
            Suit::E => "=",
            Suit::X => "!"
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