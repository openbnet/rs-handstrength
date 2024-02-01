#[cfg(test)]
mod tests {
    use crate::card::{Card, Suit};
    use crate::can_str::can_have_str;
    use insta::assert_debug_snapshot;
    #[test]
    fn cannot_str() {
        let cards = vec![
            Card { value: 2, suit: Suit::S },
            Card { value: 10, suit: Suit::S },
            Card { value: 11, suit: Suit::H },
            Card { value: 10, suit: Suit::D },
            Card { value: 5, suit: Suit::C },
        ];

        let str_combis = can_have_str(&cards);
        // println!("fs_combinations, {:?}", str_combis);
        // Check for 13 possible combinations (excluding 10)
        assert_eq!(str_combis.len(), 0);
    }
    #[test]
    fn can_str1() {
        let cards = vec![
            Card { value: 1, suit: Suit::S },
            Card { value: 2, suit: Suit::S },
            Card { value: 3, suit: Suit::H },
            Card { value: 8, suit: Suit::C },
            Card { value: 9, suit: Suit::C },
        ];

        let str_combis = can_have_str(&cards);
        // println!("fs_combinations, {:?}", str_combis);
        // Check for 13 possible combinations (excluding 10)
        assert_eq!(str_combis.len(), 1);
        assert_debug_snapshot!(str_combis);
    }

    #[test]
    fn can_str2() {
        let cards = vec![
            Card { value: 1, suit: Suit::S },
            Card { value: 2, suit: Suit::S },
            Card { value: 3, suit: Suit::H },
            Card { value: 4, suit: Suit::C },
            Card { value: 9, suit: Suit::C },
        ];

        let str_combis = can_have_str(&cards);
        // println!("fs_combinations, {:?}", str_combis);
        // println!("test len {:?}", cards.len());
        // Check for 13 possible combinations (excluding 10)
        assert_eq!(str_combis.len(), 2);
        assert_debug_snapshot!(str_combis);
    }
    #[test]
    fn can_str3() {
        let cards = vec![
            Card { value: 1, suit: Suit::S },
            Card { value: 2, suit: Suit::S },
            Card { value: 3, suit: Suit::H },
            Card { value: 4, suit: Suit::C },
            Card { value: 5, suit: Suit::C },
        ];

        let str_combis = can_have_str(&cards);
        // println!("fs_combinations, {:?}", str_combis);
        // println!("test len {:?}", cards.len());
        // Check for 13 possible combinations (excluding 10)
        assert_eq!(str_combis.len(), 3);
        assert_debug_snapshot!(str_combis);
    }
    #[test]
    fn can_str4() {
        let cards = vec![
            Card { value: 1, suit: Suit::S },
            Card { value: 2, suit: Suit::S },
            Card { value: 3, suit: Suit::H },
            Card { value: 10, suit: Suit::C },
            Card { value: 11, suit: Suit::C },
        ];

        let str_combis = can_have_str(&cards);
        // println!("fs_combinations, {:?}", str_combis);
        assert_eq!(str_combis.len(), 2);
        assert_debug_snapshot!(str_combis);
    }
    #[test]
    fn can_str5() {
        let cards = vec![
            Card { value: 10, suit: Suit::S },
            Card { value: 9, suit: Suit::S },
            Card { value: 7, suit: Suit::H },
            Card { value: 6, suit: Suit::C },
            Card { value: 5, suit: Suit::C },
        ];

        let str_combis = can_have_str(&cards);
        // println!("fs_combinations, {:?}", str_combis);
        assert_eq!(str_combis.len(), 5);
        assert_debug_snapshot!(str_combis);
    }
}