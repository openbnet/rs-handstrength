use crate::card::{Card, Suit};
use crate::sorting::get_same_value_map;
pub fn can_have_quads(cards: &[Card]) -> Vec<Vec<Card>> {
    let grouped_cards = get_same_value_map(cards);
    let mut quads_combinations = Vec::new();

    for group in grouped_cards {
        let card_value = group[0].value;
        match group.len() {
            3 => {
                // Trips on the board
                for val in 1..=14 {
                    if val != card_value {
                        quads_combinations.push(vec![
                            Card { value: card_value, suit: Suit::A },
                            Card { value: val, suit: Suit::A }
                        ]);
                    }
                }
            }
            2 => {
                // One pair on the board
                quads_combinations.push(vec![
                    Card { value: card_value, suit: Suit::A },
                    Card { value: card_value, suit: Suit::A }
                ]);
            }
            _ => {}
        }
    }

    quads_combinations
}
