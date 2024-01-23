#[cfg(test)]
mod tests {
    use crate::card::{Card, Suit};
    use crate::equity::*;
    use insta::assert_debug_snapshot;

    // #[test]
    fn test_equity_basic() {
        // Example set-up for a basic test
        let hand1 = vec![Card { value: 2, suit: Suit::D }, Card { value: 3, suit: Suit::D }];
        let hand2 = vec![Card { value: 4, suit: Suit::D }, Card { value: 5, suit: Suit::D }];

        let community_cards = vec![
            Card { value: 6, suit: Suit::S }, 
            Card { value: 7, suit: Suit::C }, 
            Card { value: 8, suit: Suit::H }
        ];

        let hands = vec![hand1, hand2];
        let equities = equity(hands, community_cards);
        // println!("equities {:?}", equities);

    }
    #[test]
    fn test_equity() {
        // Example set-up for a basic test
        let hand1 = vec![Card { value: 7, suit: Suit::S }, Card { value: 1, suit: Suit::D }, Card { value: 13, suit: Suit::S}, Card {value: 12, suit: Suit::S}];
        let hand2 = vec![Card { value: 13, suit: Suit::H }, Card { value: 8, suit: Suit::C }, Card {value: 7, suit: Suit::C}, Card {value: 10, suit: Suit::C}];

        let community_cards = vec![
            Card { value: 9, suit: Suit::D }, 
            Card { value: 5, suit: Suit::S }, 
            Card { value: 5, suit: Suit::C },
            Card { value: 10, suit: Suit::D}
        ];

        let hands = vec![hand1, hand2];
        let equities = equity(hands, community_cards);
        // println!("equities {:?}", equities);

    }
}