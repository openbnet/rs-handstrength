use crate::card::{Card, Suit};
use crate::sorting::get_same_value_map;
use crate::can_libs::*;
use itertools::Itertools;
use std::collections::HashSet;

pub fn can_have_str(comcards: &Vec<Card>) -> CanHaveCombis {
    let mut can_have_combis: CanHaveCombis = Vec::new();
    let grouped_cards = get_same_value_map(comcards);
    let unique_cards: Vec<Card> = grouped_cards.iter()
        .filter(|group| group.len() == 1)
        .map(|group| group[0].clone())
        .collect();
    let card_set: HashSet<u8> = unique_cards.iter().map(|card| card.value).collect();

    for start in [10, 1, 9, 8, 7, 6, 5, 4, 3, 2] {
        let straight_values = (start..start + 5).collect::<Vec<u8>>();

        let board_candidates: Vec<Card> = unique_cards.iter()
            .filter(|&card| straight_values.contains(&card.value))
            .cloned()
            .collect();

        let missing_values: Vec<u8> = straight_values.iter()
            .filter(|&val| !card_set.contains(val))
            .cloned()
            .collect();

        let possible_hands_combinations = match board_candidates.len() {
            5 => {
                // All five cards form a straight
                vec![board_candidates.iter().combinations(2).map(|combo| {
                    combo.into_iter().cloned().collect::<Vec<Card>>()
                }).collect::<Vec<Vec<Card>>>()] // This is a Vec<SameRankHands>
            },
            4 => {
                // Pair the missing value with each card in the straight
                let missing_card = Card { value: missing_values[0], suit: Suit::A };
                vec![board_candidates.iter()
                    .filter(|card| card.value != missing_values[0])
                    .map(|&existing_card| vec![missing_card.clone(), existing_card])
                    .collect::<Vec<Vec<Card>>>()] // This is a Vec<SameRankHands>
            },
            3 => {
                // Create combinations with the two missing values
                vec![missing_values.iter().combinations(2).map(|pair| {
                    pair.into_iter().map(|&val| Card { value: val, suit: Suit::A }).collect::<Vec<Card>>()
                }).collect::<Vec<Vec<Card>>>()] // This is a Vec<SameRankHands>
            },
            _ => Vec::new()
        };
        
        if !possible_hands_combinations.is_empty() {
            can_have_combis.push((board_candidates, possible_hands_combinations));
        }
    }

    can_have_combis
}
