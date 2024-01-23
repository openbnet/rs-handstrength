#[cfg(test)]
mod tests {
    use crate::card::{Card, Suit};
    use crate::equity::*;
    use insta::assert_debug_snapshot;

    #[test]
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
        println!("equities {:?}", equities);
        // Assertions to check if the results are as expected
    

        // Here you can add assertions about the expected equity values.
        // This will depend on your game logic and get_nut_rank function.
        // For example:
        // assert_eq!(equities.get(&0).unwrap(), &0.5); 
    }
}