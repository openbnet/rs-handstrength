use crate::card::{Card, Suit};
use crate::can_libs::*;
use itertools::Itertools;
use std::collections::HashSet;

pub fn can_have_str(comcards: &Vec<Card>) -> CanHaveCombis {
    let mut can_have_combis: CanHaveCombis = Vec::new();
    let card_set: HashSet<u8> = comcards.iter().map(|card| card.value).collect();

    for start in [10, 1, 9, 8, 7, 6, 5, 4, 3, 2] {
        let straight_values = if start == 1 {
            vec![1, 2, 3, 4, 5]  // Special case for Ace-low
        } else {
            (start..start + 5).collect::<Vec<u8>>()
        };

        let board_candidates: Vec<Card> = comcards.iter()
            .filter(|&card| straight_values.contains(&card.value))
            .cloned()
            .collect();

        let missing_values: Vec<u8> = straight_values.iter()
            .filter(|&val| !card_set.contains(val))
            .cloned()
            .collect();

            let possible_hands_combinations = match missing_values.len() {
                1 => {
                    // Pair the missing value with each card in the straight
                    let new_card = Card { value: missing_values[0], suit: Suit::A };
                    vec![board_candidates.iter()
                        .filter(|card| card.value != missing_values[0])
                        .map(|&existing_card| vec![new_card.clone(), existing_card])
                        .collect::<Vec<Vec<Card>>>()]
                },
                2 => {
                    // Create combinations with the missing values
                    vec![vec![missing_values.iter().map(|&val| Card { value: val, suit: Suit::A }).collect()]]
                },
                _ => Vec::new()
            };
            
        if !possible_hands_combinations.is_empty() {
            can_have_combis.push((board_candidates, possible_hands_combinations));
        }
    }

    can_have_combis
}
