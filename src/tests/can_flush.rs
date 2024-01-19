
#[cfg(test)]

mod tests {
    use crate::card::{Card, Suit};
    use crate::can_flush::can_have_flush;
    use insta::assert_debug_snapshot;
    // #[test]
    fn no_flush() {
        let comcards = vec![
            Card { value: 2, suit: Suit::H },
            Card { value: 3, suit: Suit::D },
            Card { value: 4, suit: Suit::S },
        ];
        let flushes = can_have_flush(&comcards);
        assert_eq!(flushes.len(), 0);
        
    }

    #[test]
    fn flush_exists() {
        let comcards = vec![
            Card { value: 2, suit: Suit::H },
            Card { value: 3, suit: Suit::H },
            Card { value: 4, suit: Suit::H },
        ];
        let flushes = can_have_flush(&comcards);
        println!("flushes {:?}", flushes.len());
        assert_eq!(flushes.len(), 1);
        assert_eq!(flushes[0].1.len(), 45);
        assert_debug_snapshot!(flushes);
    }
    #[test]
    fn flush_exists4() {
        let comcards = vec![
            Card { value: 2, suit: Suit::H },
            Card { value: 3, suit: Suit::H },
            Card { value: 4, suit: Suit::H },
            Card { value: 5, suit: Suit::H }
        ];
        let flushes = can_have_flush(&comcards);
        println!("flushes {:?}", flushes.len());
        assert_eq!(flushes.len(), 1);
        assert_eq!(flushes[0].1.len(), 36);
        assert_debug_snapshot!(flushes);
    }
    // // #[test]
    // fn multiple_suits_no_flush() {
    //     let comcards = vec![
    //         Card { value: 2, suit: Suit::H },
    //         Card { value: 3, suit: Suit::D },
    //         Card { value: 4, suit: Suit::H },
    //         Card { value: 5, suit: Suit::C },
    //     ];
    //     assert_eq!(can_have_flush(&comcards), None);
    // }

    // // #[test]
    // fn empty_array() {
    //     let comcards: Vec<Card> = vec![];
    //     assert_eq!(can_have_flush(&comcards), None);
    // }

    // // #[test]
    // fn all_same_suit() {
    //     let comcards = vec![
    //         Card { value: 2, suit: Suit::C },
    //         Card { value: 3, suit: Suit::C },
    //         Card { value: 4, suit: Suit::C },
    //         Card { value: 5, suit: Suit::C },
    //     ];
    //     assert_eq!(can_have_flush(&comcards), Some(Suit::C));
    // }
    
}
