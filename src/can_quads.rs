use crate::card::{Card, Suit};
use crate::sorting::get_same_value_map;
use crate::can_libs::*;
pub fn can_have_quads(cards: &[Card]) -> CanHaveCombis {
    let grouped_cards = get_same_value_map(cards);
    let mut quads_combinations: CanHaveCombis = Vec::new();

    for group in grouped_cards {
        let card_value = group[0].value;
        match group.len() {
            3 => {
                // Trips on the board
                let mut hand_matchers = Vec::new();
                for val in (2..=14).rev() {
                    if val != card_value {
                        hand_matchers.push(
                            vec![vec![
                            Card { value: card_value, suit: Suit::A },
                            Card { value: if val == 14 { 1 } else { val }, suit: Suit::A }]
                        ]);
                    }
                }
                println!("hand_matchers {:?}", hand_matchers);
                quads_combinations.push((
                    group,
                    hand_matchers 
                ));
            }
            2 => {
                // One pair on the board
                quads_combinations.push((
                    group,
                    vec![vec![vec![
                    Card { value: card_value, suit: Suit::A },
                    Card { value: card_value, suit: Suit::A }
                ]]]));
            }
            _ => {}
        }

    }

    quads_combinations
}
