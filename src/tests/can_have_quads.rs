
#[cfg(test)]

mod tests {
    use crate::card::{Card, Suit};
    use crate::can_quads::can_have_quads;
    use insta::assert_debug_snapshot;
    use crate::get_nut_rank::*;
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
        // println!("quads_combinations, {:?}", quads_combinations);
        // Check for 13 possible combinations (excluding 10)
        assert_eq!(quads_combinations.len(), 1);
        // println!("first board {:?}", quads_combinations[0].1);
        assert_debug_snapshot!(quads_combinations);
        let hand = vec![
            Card { value: 10, suit: Suit::H },
            Card { value: 11, suit: Suit::D },
            Card { value: 2, suit: Suit::S },
            Card { value: 2, suit: Suit::C },
        ];
        let nr = get_nut_rank(&hand, &cards);
        // println!("nr {:?}", nr);
        assert_eq!(nr.0, 3);
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
        // println!("quads_combinations {:?}", quads_combinations[0].1);
        assert_eq!(quads_combinations[0].1.len(), 1, "Combination one hand");
        assert_debug_snapshot!(quads_combinations)
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
        // println!("quads_combinations {:?}", quads_combinations);
        assert_debug_snapshot!(quads_combinations)
    }
     
}
