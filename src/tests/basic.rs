
#[cfg(test)]

mod tests {
    use crate::card::{Card, Suit};
    use crate::sorting::{sort_cards, get_same_value_map};
    #[test]
    fn test_sort_fixed_size_array() {
        let cards = vec![
            Card { value: 10, suit: Suit::H },
            Card { value: 4, suit: Suit::D },
            Card { value: 5, suit: Suit::C },
            Card { value: 5, suit: Suit::S },
        ];
        let sorted_cards = sort_cards(&cards);

        // Adjust the expected order to match the sorting logic
        let expected = vec![
            Card { value: 10, suit: Suit::H },
            Card { value: 5, suit: Suit::S },
            Card { value: 5, suit: Suit::C },
            Card { value: 4, suit: Suit::D },
        ];
        assert_eq!(sorted_cards, expected);
    }
    #[test]
    fn test_get_same_value_map() {
        let cards = vec![
            Card { value: 10, suit: Suit::H },
            Card { value: 5, suit: Suit::D },
            Card { value: 5, suit: Suit::C },
            Card { value: 3, suit: Suit::S },
        ];

        let grouped_cards = get_same_value_map(&cards);
        println!("grouped_cards, {:?}", grouped_cards);
        // Test the number of groups
        assert_eq!(grouped_cards.len(), 3);

        // Test the contents of the first group (should be the 10 of Hearts)
        assert_eq!(grouped_cards[0].len(), 1);
        assert_eq!(grouped_cards[0][0], Card { value: 10, suit: Suit::H });

        // Test the contents of the second group (should be the two 5s)
        assert_eq!(grouped_cards[1].len(), 2);
        assert!(grouped_cards[1].contains(&Card { value: 5, suit: Suit::D }));
        assert!(grouped_cards[1].contains(&Card { value: 5, suit: Suit::C }));
    }
}
