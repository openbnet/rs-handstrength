
#[cfg(test)]

mod tests {
    use crate::card::{Card, Suit};
    use crate::can_fullhouse::can_have_fullhouse;
    use insta::assert_debug_snapshot;
    // #[test]
    fn fh_with_trips_board() {
        let cards = vec![
            Card { value: 1, suit: Suit::S },
            Card { value: 10, suit: Suit::S },
            Card { value: 10, suit: Suit::H },
            Card { value: 10, suit: Suit::D },
            Card { value: 5, suit: Suit::C },
        ];

        let fh_combis = can_have_fullhouse(&cards);
        println!("fs_combinations, {:?}", fh_combis);
        // Check for 13 possible combinations (excluding 10)
        assert_eq!(fh_combis.len(), 2);
        println!("first board {:?}", fh_combis[1].1);
   
        assert_debug_snapshot!(fh_combis)
    }  
    // #[test]
    fn fh_with_trips_pair_board() {
        let cards = vec![
            Card { value: 5, suit: Suit::S },
            Card { value: 10, suit: Suit::S },
            Card { value: 10, suit: Suit::H },
            Card { value: 10, suit: Suit::D },
            Card { value: 5, suit: Suit::C },
        ];

        let fh_combis = can_have_fullhouse(&cards);
        println!("fs_combinations, {:?}", fh_combis);
        // Check for 13 possible combinations (excluding 10)
        assert_eq!(fh_combis.len(), 1);
        println!("first board {:?}", fh_combis[0].1);
   
        assert_debug_snapshot!(fh_combis)
    }  
    // #[test]
    fn fh_with_trips_pair_above_board() {
        let cards = vec![
            Card { value: 11, suit: Suit::S },
            Card { value: 10, suit: Suit::S },
            Card { value: 10, suit: Suit::H },
            Card { value: 10, suit: Suit::D },
            Card { value: 11, suit: Suit::C },
        ];

        let fh_combis = can_have_fullhouse(&cards);
        println!("fs_combinations, {:?}", fh_combis);
        // Check for 13 possible combinations (excluding 10)
        assert_eq!(fh_combis.len(), 1);
        println!("first board {:?}", fh_combis[0].1);
   
        assert_debug_snapshot!(fh_combis)
    }   
    // #[test]
    fn fh_with_pair_board() {
        let cards = vec![
            Card { value: 11, suit: Suit::S },
            Card { value: 10, suit: Suit::S },
            Card { value: 10, suit: Suit::H },
            Card { value: 8, suit: Suit::D },
            Card { value: 4, suit: Suit::C },
        ];

        let fh_combis = can_have_fullhouse(&cards);
        println!("fs_combinations, {:?}", fh_combis);
        // Check for 13 possible combinations (excluding 10)
        assert_eq!(fh_combis.len(), 6);
        println!("first board {:?}", fh_combis[0].1);
   
        assert_debug_snapshot!(fh_combis) 
    }
    #[test]
    fn fh_with_2pair_board() {
        let cards = vec![
            Card { value: 1, suit: Suit::S },
            Card { value: 10, suit: Suit::S },
            Card { value: 10, suit: Suit::H },
            Card { value: 1, suit: Suit::D },
            Card { value: 4, suit: Suit::C },
        ];

        let fh_combis = can_have_fullhouse(&cards);
        println!("fs_combinations, {:?}", fh_combis);
        // Check for 13 possible combinations (excluding 10)
        assert_eq!(fh_combis.len(), 4);
        println!("first board {:?}", fh_combis[0].1);
   
        assert_debug_snapshot!(fh_combis) 
    }
}
