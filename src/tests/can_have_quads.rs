
#[cfg(test)]

mod tests {
    use crate::card::{Card, Suit};
    use crate::can_quads::can_have_quads;
    #[test]
    fn quads_with_trips_board() {
        let cards = vec![
            Card { value: 10, suit: Suit::S },
            Card { value: 10, suit: Suit::H },
            Card { value: 10, suit: Suit::D },
            Card { value: 8, suit: Suit::S },
            Card { value: 5, suit: Suit::C },
        ];

        let quads_combinations = can_have_quads(&cards);
        println!("quads_combinations, {:?}", quads_combinations);
        // Check for 13 possible combinations (excluding 10)
        assert_eq!(quads_combinations.len(), 13);

        for combo in &quads_combinations {
            assert_eq!(combo[0].value, 10); // The value of the trips
            assert_ne!(combo[1].value, 10); // The additional card is not the same as the trips' value
        }
    }
    #[test]
    fn quads_with_no_pair_board() {
        let cards = vec![
            Card { value: 11, suit: Suit::S },
            Card { value: 10, suit: Suit::H },
            Card { value: 8, suit: Suit::D },
            Card { value: 3, suit: Suit::S },
            Card { value: 5, suit: Suit::C },
        ];

        let quads_combinations = can_have_quads(&cards);
        assert_eq!(quads_combinations.len(), 0, "Should be no quads combinations");
    }
    #[test]
    fn quads_with_one_pair_board() {
        let cards = vec![
            Card { value: 10, suit: Suit::S },
            Card { value: 10, suit: Suit::H },
            Card { value: 8, suit: Suit::D },
            Card { value: 3, suit: Suit::S },
            Card { value: 5, suit: Suit::C },
        ];
    
        let quads_combinations = can_have_quads(&cards);
        assert_eq!(quads_combinations.len(), 1, "Should be one quads combination");
        assert_eq!(quads_combinations[0].len(), 2, "Combination should have two cards");
    }
    #[test]
    fn quads_with_two_pair_board() {
        let cards = vec![
            Card { value: 10, suit: Suit::S },
            Card { value: 10, suit: Suit::H },
            Card { value: 8, suit: Suit::D },
            Card { value: 8, suit: Suit::S },
            Card { value: 5, suit: Suit::C },
        ];
    
        let quads_combinations = can_have_quads(&cards);
        assert_eq!(quads_combinations.len(), 2, "Should be two quads combinations");
    }
     
}
