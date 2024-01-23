#[cfg(test)]
mod tests {
    use crate::card::{Card, Suit};
    use crate::can_trips::can_have_trips;
    use insta::assert_debug_snapshot;
    #[test]
    fn test_can_have_trips_with_trips_on_board() {
        let cards = vec![
            Card { value: 10, suit: Suit::H },
            Card { value: 10, suit: Suit::D },
            Card { value: 10, suit: Suit::C },
            Card { value: 2, suit: Suit::S },
            Card { value: 3, suit: Suit::S },
        ];

        let trip_combis = can_have_trips(&cards);
        assert!(!trip_combis.is_empty()); // Ensure we have some combinations
        assert_eq!(trip_combis[0].0.len(), 3); // Check if the board has exactly 3 cards (the trips)
        assert_eq!(trip_combis[0].0[0].value, 10); // Check if the board cards are the trips (value 10)
        // Add more assertions as needed
        assert_debug_snapshot!(trip_combis);
    }

    #[test]
    fn test_can_have_trips_without_trips_on_board() {
        let cards = vec![
            Card { value: 10, suit: Suit::H },
            Card { value: 9, suit: Suit::D },
            Card { value: 8, suit: Suit::C },
            Card { value: 2, suit: Suit::S },
            Card { value: 3, suit: Suit::S },
        ];

        let trip_combis = can_have_trips(&cards);
        // assert!(trip_combis.is_empty()); // Expecting no combinations
        // println!("no pair trips {:?}", trip_combis);
        assert_debug_snapshot!(trip_combis);
    }

    #[test]
    fn test_can_have_trips_with_various_card_combinations() {
        let cards = vec![
            Card { value: 2, suit: Suit::H },
            Card { value: 2, suit: Suit::D },
            Card { value: 2, suit: Suit::C },
            Card { value: 5, suit: Suit::S },
            Card { value: 7, suit: Suit::S },
        ];

        let trip_combis = can_have_trips(&cards);
        assert!(!trip_combis.is_empty()); // Expecting some combinations
        // Add specific checks for expected combinations
        assert_debug_snapshot!(trip_combis);
    }
    #[test]
    fn test_can_have_trips_with_fh() {
        let cards = vec![
            Card { value: 2, suit: Suit::H },
            Card { value: 2, suit: Suit::D },
            Card { value: 2, suit: Suit::C },
            Card { value: 5, suit: Suit::S },
            Card { value: 5, suit: Suit::S },
        ];

        let trip_combis = can_have_trips(&cards);
        assert!(!trip_combis.is_empty()); // Expecting some combinations
        // Add specific checks for expected combinations
        assert_debug_snapshot!(trip_combis);
    }
    #[test]
    fn test_can_have_trips_with_over_fh() {
        let cards = vec![
            Card { value: 2, suit: Suit::H },
            Card { value: 2, suit: Suit::D },
            Card { value: 5, suit: Suit::C },
            Card { value: 5, suit: Suit::S },
            Card { value: 5, suit: Suit::S },
        ];

        let trip_combis = can_have_trips(&cards);
        assert!(!trip_combis.is_empty()); // Expecting some combinations
        // Add specific checks for expected combinations
        assert_debug_snapshot!(trip_combis);
    }
}
