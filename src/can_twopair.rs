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
    let sorted = replace_ace_as_high(&sort_cards(comcards));
    let grouped_cards = get_same_value_map(&sorted);
    let has_trips_or_quads = grouped_cards.iter().any(|group| group.len() >= 3);
    if has_trips_or_quads {
        return can_have_combis;
    }
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
            let grouped_cards = get_same_value_map(&sorted);
            let has_2pairs = grouped_cards.iter().filter(|group| group.len() == 2).count() == 2;
            let sorted_values = sorted.iter().map(|card| card.value).collect::<Vec<u8>>();
            let highest_pair = grouped_cards.iter().find(|group| group.len() == 2).unwrap()[0].value;
            let all_values = [14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2];
            let not_used_values = all_values.iter().filter(|&v| !sorted_values.contains(v)).collect::<Vec<&u8>>();
            
            if has_2pairs {
                // grouped_cards are sorted, highest pair is the first one

                // when theres 2 pairs, there can only be one single value in a 5 card board
                let single_value = grouped_cards.iter().filter(|group| group.len() == 1).next();
                // println!("kicker {:?}", kicker);
                // println!("highest_pair {:?}", highest_pair);
                
                let mut p_eval: PEval = Vec::new();
                
                for i in all_values {
                    if !sorted_values.contains(&i) {
                        // push pair in hand to PEval
                        p_eval.push(vec![vec![Card { value: i, suit: Suit::A },Card { value: i, suit: Suit::A }]]);
                        

                    } else {
                       // hit the single_value if it exists with unused values
                        if let Some(single_value) = single_value {
                            if single_value[0].value == i {
                                let kickers = not_used_values.iter()
                                    .map(|&v| {
                                        vec![vec![Card { value: *v, suit: Suit::A },Card { value: i, suit: Suit::A }]]
                                    }).collect::<PEval>();
                                p_eval.extend(kickers);
                            } 
                        }
                    }
                } 
                return vec![(sorted.clone(), p_eval)];

            } else {
                // has 1 pair,
                let mut p_eval: PEval = Vec::new();
                let single_values = grouped_cards.iter().filter(|group| group.len() == 1).map(|group| group[0].value).collect::<Vec<u8>>();
                for i in all_values {
                    if !sorted_values.contains(&i) {
                        // push pair in hand to PEval
                        p_eval.push(vec![vec![Card { value: i, suit: Suit::A },Card { value: i, suit: Suit::A }]]);

                    } else {
                        if single_values.contains(&i) {
                            let kickers = not_used_values.iter()
                                .map(|&v| {
                                    vec![vec![Card { value: *v, suit: Suit::A },Card { value: i, suit: Suit::A }]]
                                }).collect::<PEval>();
                            p_eval.extend(kickers);
                        } else {
                            // println!("ignoring i {:?} {:?}", i, single_values)
                        }
                    }
                }
                can_have_combis.push((sorted.clone(), p_eval));

            }
        }
    }

    can_have_combis
}
