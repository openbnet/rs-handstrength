use crate::card::{Card, Suit};
use crate::sorting::get_same_value_map;
use crate::can_libs::*;
use itertools::Itertools;
use std::collections::HashSet;
// new implementation
// first identify subsets of cards that can form a straight
// then identify which cards can be added to the subset to form a straight

pub fn can_have_str(comcards: &Vec<Card>) -> CanHaveCombis {
    let mut can_have_combis: CanHaveCombis = Vec::new();
    let grouped_cards = get_same_value_map(comcards);
    let mut unique_cards = grouped_cards.iter()
        .map(|group| group[0].clone())
        .collect::<Vec<Card>>();
    // we expected the cards to be sorted desc, so if there is an ace, it should be the last card
    let has_ace = &unique_cards.last().unwrap().value == &1;
    if has_ace {
        // find we only need to inject one ace, since we expect unique cards anyway
        let aces = grouped_cards.iter()
            .clone()
            .filter(|group| group[0].value == 1)
            .map(|group| group[0].clone())
            .map(|card| Card { value: 14, suit: card.suit })
            .collect::<Vec<Card>>();
        // add all aces to the front of unique_cards
        unique_cards.splice(0..0, aces);
        
        println!("unique_cards after extend{:?}", unique_cards);
        // find subset of cards that can form a straight

    } 
    // below is giving me repeat values
    // we need to store the CardHand that we've used in PEval so far 
    // and then not include these CardHand in the inner loop
    let mut used_hands: HashSet<CardHand> = HashSet::new();
 
    for start in [10, 1, 9, 8, 7, 6, 5, 4, 3, 2] {
        let straight_values = (start..start + 5).collect::<Vec<u8>>();
        // println!("straight_values {:?}", straight_values);
        let board_candidates: Vec<Card> = unique_cards.iter()
            .filter(|&card| straight_values.contains(&card.value))
            .cloned()
            .collect();
        let missing_values: Vec<u8> = straight_values.iter()
            .filter(|&val| !board_candidates.iter().map(|card| card.value).collect::<HashSet<u8>>().contains(val))
            .cloned()
            .collect();
        if missing_values.len() > 2 {
            continue;
        }
        // println!("board_candidates {:?}", board_candidates);
        // println!("missing_values {:?}", missing_values);
        match missing_values.len() {
            2 => {
                let hand: CardHand = vec![
                    Card { value: missing_values[0], suit: Suit::A },
                    Card { value: missing_values[1], suit: Suit::A }
                ];
                // if in hashset, continue
                
                // add to hashset
                used_hands.insert(hand.clone());
                can_have_combis.push((board_candidates, vec![vec![hand]]));
            }
            1 => {
                // there can only be one board combi with one missing value
                // we need a SameRankHands vec with each hand as the missing value, paired with each card in the board_candidates
                // can_have_combis.push((board_candidates.clone(), board_candidates.iter()
                //     .map(|&existing_card| vec![vec![
                //         Card { value: missing_values[0], suit: Suit::A },
                //         Card { value: existing_card.value, suit: Suit::A }
                //     ]])
                //     .collect::<Vec<Vec<Vec<Card>>>>()));
                // the above code does not group them all into the same Same, but are all different rank
                // we need to group them into the same rank
                let combis = board_candidates.iter()
                    .filter_map(|&card| {
                        let hand = vec![
                            Card { value: missing_values[0], suit: Suit::A },
                            Card { value: card.value, suit: Suit::A }
                        ]; 
                        // if in hashset, filter it out
                        if used_hands.contains(&hand) {
                            None
                        } else {
                            // add to hashset
                            used_hands.insert(hand.clone());
                            Some(hand)
                        }
                    }).collect::<SameRankHands>();
                // println!("combis {:?}", combis);
                can_have_combis.push((board_candidates, vec![combis]));
                // return vec![(board_candidates, vec![vec![vec![
                //     Card { value: missing_values[0], suit: Suit::A }
                // ]]])];
            }
            0 => {
                // all cards in the board_candidates form a straight
                // we need a SameRankHands vec with each hand as any of 2 of the board candidates of 5 cards
                let combis = board_candidates.iter().combinations(2).map(|combo| {
                    combo.into_iter().map(|card| {
                        let hand = Card { value: card.value, suit: Suit::A };
                        // add to hashset
                        used_hands.insert(vec![hand.clone()]);
                        hand
                    }).collect::<Vec<Card>>()
                }).collect::<SameRankHands>();
                // println!("combis {:?}", combis);
                can_have_combis.push((board_candidates, vec![combis]));
            }
            _ => panic!("match isnt smart enough to know we continued on len 2 above")
        }
    }
    can_have_combis
    
}
// this doesnt work because it only iterates over subsets of the board at a time
// we need to take into account the whole board at once
// pub fn can_have_str(comcards: &Vec<Card>) -> CanHaveCombis {
//     let mut can_have_combis: CanHaveCombis = Vec::new();
//     let grouped_cards = get_same_value_map(comcards);
//     let unique_cards: Vec<Card> = grouped_cards.iter()
//         .filter(|group| group.len() == 1)
//         .map(|group| group[0].clone())
//         .collect();
//     let card_set: HashSet<u8> = unique_cards.iter().map(|card| card.value).collect();

//     for start in [10, 1, 9, 8, 7, 6, 5, 4, 3, 2] {
//         let straight_values = (start..start + 5).collect::<Vec<u8>>();

//         let board_candidates: Vec<Card> = unique_cards.iter()
//             .filter(|&card| straight_values.contains(&card.value))
//             .cloned()
//             .collect();

//         let missing_values: Vec<u8> = straight_values.iter()
//             .filter(|&val| !card_set.contains(val))
//             .cloned()
//             .collect();

//         let possible_hands_combinations = match board_candidates.len() {
//             5 => {
//                 // All five cards form a straight
//                 vec![board_candidates.iter().combinations(2).map(|combo| {
//                     combo.into_iter().cloned().collect::<Vec<Card>>()
//                 }).collect::<Vec<Vec<Card>>>()] // This is a Vec<SameRankHands>
//             },
//             4 => {
//                 // Pair the missing value with each card in the straight
//                 let missing_card = Card { value: missing_values[0], suit: Suit::A };
//                 vec![board_candidates.iter()
//                     .filter(|card| card.value != missing_values[0])
//                     .map(|&existing_card| vec![missing_card.clone(), existing_card])
//                     .collect::<Vec<Vec<Card>>>()] // This is a Vec<SameRankHands>
//             },
//             3 => {
//                 // Create combinations with the two missing values
//                 vec![missing_values.iter().combinations(2).map(|pair| {
//                     pair.into_iter().map(|&val| Card { value: val, suit: Suit::A }).collect::<Vec<Card>>()
//                 }).collect::<Vec<Vec<Card>>>()] // This is a Vec<SameRankHands>
//             },
//             _ => Vec::new()
//         };
        
//         if !possible_hands_combinations.is_empty() {
//             can_have_combis.push((board_candidates, possible_hands_combinations));
//         }
//     }

//     can_have_combis
// }
