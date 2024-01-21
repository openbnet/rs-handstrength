use crate::card::{Card, Suit};
use crate::sorting::get_same_value_map;
use itertools::Itertools;
use std::collections::HashSet;
use crate::can_libs::*;

pub fn can_have_trips(comcards: &Vec<Card>) -> CanHaveCombis {
    let mut can_have_combis: CanHaveCombis = Vec::new();
    let grouped_cards = get_same_value_map(comcards);
    let all_values: HashSet<u8> = (2..=14).collect(); // All possible card values
    let has_pair = grouped_cards.iter().any(|group| group.len() == 2);
    let has_trips = grouped_cards.iter().any(|group| group.len() == 3);
    let trips_value = grouped_cards.iter().find(|group| group.len() == 3).map(|group| group[0].value);
    let comcard_values: HashSet<u8> = comcards.iter().map(|card| card.value).collect(); 
    let pair_values: Vec<u8> = grouped_cards.iter()
    .filter(|group| group.len() == 2)
    .map(|group| group[0].value)
    .collect();

    for group in grouped_cards {
        let group_value = group[0].value;

        match group.len() {
            1 if !has_pair && !has_trips => {
                // Single card on board, need to find pairs in the remaining cards
                let possible_pairs = vec![vec![Card { value: group_value, suit: Suit::A }, Card { value: group_value, suit: Suit::A }]];
                can_have_combis.push((group.clone(), vec![possible_pairs]));
            },
            2 => {
                if !has_trips || trips_value.map_or(false, |val| group_value > val) {
                    let mut possible_kickers = all_values.iter()
                        .filter(|&&val| val != group_value && !comcard_values.contains(&val))
                        .map(|&val| Card { value: val, suit: Suit::A })
                        .collect::<Vec<Card>>();
            
                    // Sort kickers in descending order
                    possible_kickers.sort_by(|a, b| b.value.cmp(&a.value));
            
                    let possible_hands = possible_kickers.iter()
                        .map(|kicker| vec![vec![kicker.clone(), Card { value: group_value, suit: Suit::A }]])
                        .collect::<Vec<SameRankHands>>();
            
                    can_have_combis.push((group.clone(), possible_hands));
                }
            },
            
            3 | 4 => {
                // Process trips
                if let Some(t_val) = trips_value {
                    let mut possible_kickers = all_values.iter()
                        .filter(|&&val| val != t_val && (!has_pair || !pair_values.contains(&val)))
                        .map(|&val| Card { value: val, suit: Suit::A })
                        .collect::<Vec<Card>>();
            
                    if has_pair {
                        // Include the lower pair values in the possible kickers
                        for &p_val in &pair_values {
                            if p_val < t_val {
                                possible_kickers.push(Card { value: p_val, suit: Suit::A });
                            }
                        }
                    }
            
                    // Sort kickers in descending order (higher values first)
                    possible_kickers.sort_by(|a, b| b.value.cmp(&a.value));
            
                    // Generate all two-card combinations
                    let possible_hands = possible_kickers.iter()
                        .combinations(2)
                        .map(|combo| vec![combo.iter().map(|&card| *card).collect::<Vec<Card>>()])
                        .collect::<Vec<SameRankHands>>();
            
                    can_have_combis.push((group.clone(), possible_hands));
                }
            },
            
            _ => {} // Ignore groups of size 4 or more
        }
    }

    can_have_combis
}
