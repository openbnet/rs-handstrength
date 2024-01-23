use crate::card::{Card, Suit};
use itertools::Itertools;
use std::collections::HashSet;
use crate::can_libs::*;
use crate::sorting::*;

// Function to check if there are pairs on the board
pub fn has_pairs(comcards: &Vec<Card>) -> bool {
    let card_values: HashSet<u8> = comcards.iter().map(|card| card.value).collect();
    comcards.iter().any(|card| card_values.contains(&card.value) && comcards.iter().filter(|&c| c.value == card.value).count() == 2)
}
// ASSUME trips on board handled by can_have_trips, we are ignoring that use case.
// Function to generate two-pair combinations
pub fn can_have_twopairs(comcards: &Vec<Card>) -> CanHaveCombis {
    let mut can_have_combis: CanHaveCombis = Vec::new();
    let sorted = sort_cards(comcards);
    match has_pairs(&sorted) {
        false => {
            // println!("has pairs false {:?}", sorted);
            let board_values: Vec<u8> = sorted.iter().map(|card| card.value).collect();
            
            // Generate possible two-pair hands for no pairs on board
            let possible_hands = board_values.iter()
                .combinations(2)
                .filter(|pair| pair[0] != pair[1])
                .map(|pair| vec![
                    vec![Card { value: *pair[0], suit: Suit::A }, Card { value: *pair[1], suit: Suit::A }],
                ])
                .sorted_by(|a, b| Ord::cmp(&b[0][0].value, &a[0][0].value).then_with(|| Ord::cmp(&b[0][1].value, &a[0][1].value)))
                .collect::<Vec<SameRankHands>>();

            if !possible_hands.is_empty() {
                can_have_combis.push((comcards.clone(), possible_hands));
            }
        },

        true => {
            // println!("has pairs true {:?}", sorted);
            let grouped_cards = get_same_value_map(&replace_ace_as_high(&sorted));
            // println!("before grouped_cards {:?} {:?}", grouped_cards, sorted);
            let pair_groups: Vec<Vec<Card>> = grouped_cards.iter().filter(|group| group.len() == 2).cloned().collect();
            // println!("grouped_cards {:?} pair_groups {:?}", grouped_cards, pair_groups);
            let pair_values: Vec<u8> = pair_groups.iter().map(|group| group[0].value).collect();
            let single_groups : Vec<Vec<Card>> = grouped_cards.iter().filter(|group| group.len() == 1).cloned().collect();
            let all_values: Vec<u8> = (2..=14).rev().collect(); // All possible card values
            let single_values: Vec<u8> = single_groups.iter().map(|group| group[0].value).collect();
            // we can ignore 2nd pair group as it is the lower value
            // println!("all_values {:?}", all_values);
            for single in &single_values {
                let higher_singles: Vec<u8> = single_values.iter().filter(|v| v >= &single).cloned().collect();
                let kickers: Vec<Vec<Vec<Card>>> = all_values.iter().filter(|&&v| !higher_singles.contains(&v) && !pair_values.contains(&v)).map(|&v| vec![vec![Card {value: v, suit: Suit::A}, Card {value: *single, suit: Suit::A}]]).collect();
                can_have_combis.push((
                    vec![pair_groups[0][0], pair_groups[0][1], Card { value: *single, suit: Suit::A}],
                    kickers 
                ))
            }



  
        }
    }

    can_have_combis
}
