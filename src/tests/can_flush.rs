
#[cfg(test)]

mod tests {
    use crate::card::{Card, Suit};
    use crate::can_strflush::can_flush;
    #[test]
    fn no_flush() {
        let comcards = vec![
            Card { value: 2, suit: Suit::H },
            Card { value: 3, suit: Suit::D },
            Card { value: 4, suit: Suit::S },
        ];
        assert_eq!(can_flush(&comcards), None);
    }

    #[test]
    fn flush_exists() {
        let comcards = vec![
            Card { value: 2, suit: Suit::H },
            Card { value: 3, suit: Suit::H },
            Card { value: 4, suit: Suit::H },
        ];
        assert_eq!(can_flush(&comcards), Some(Suit::H));
    }

    #[test]
    fn multiple_suits_no_flush() {
        let comcards = vec![
            Card { value: 2, suit: Suit::H },
            Card { value: 3, suit: Suit::D },
            Card { value: 4, suit: Suit::H },
            Card { value: 5, suit: Suit::C },
        ];
        assert_eq!(can_flush(&comcards), None);
    }

    #[test]
    fn empty_array() {
        let comcards: Vec<Card> = vec![];
        assert_eq!(can_flush(&comcards), None);
    }

    #[test]
    fn all_same_suit() {
        let comcards = vec![
            Card { value: 2, suit: Suit::C },
            Card { value: 3, suit: Suit::C },
            Card { value: 4, suit: Suit::C },
            Card { value: 5, suit: Suit::C },
        ];
        assert_eq!(can_flush(&comcards), Some(Suit::C));
    }
    
}
