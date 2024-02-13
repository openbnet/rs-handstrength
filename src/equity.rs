use crate::get_nut_rank::get_nut_rank;
use std::collections::HashMap;
use crate::card::{Card, Suit};
use itertools::Itertools;
use rayon::prelude::*;
use std::sync::Mutex;
use std::sync::Arc;
use lazy_static::lazy_static;

lazy_static! {
    static ref CACHE: Mutex<HashMap<(Vec<[Card; 4]>, [Card; 3]), Vec<u8>>> = Mutex::new(HashMap::new());
}
pub fn get_remaining_cards(player_hands: &Vec<[Card; 4]>, board: &[Card; 3]) -> Vec<Card> {
    let mut allp_cards = player_hands.iter().flatten().cloned().collect::<Vec<Card>>();
    allp_cards.extend(board);
    let mut remaining = create_full_deck();
    remaining.retain(|c| !allp_cards.contains(c));
    remaining
}

pub fn create_full_deck() -> Vec<Card> {
    let suits = vec![Suit::S, Suit::H, Suit::C, Suit::D];
    let mut deck = Vec::new();

    for suit in suits {
        for value in 1..=13 {
            deck.push(Card { value, suit });
        }
    }

    deck
}
// Function to calculate the equity for each set of player cards
fn calculate_equity(player_hands: &Vec<[Card; 4]>, flop: &[Card; 3], deck: Vec<Card>) -> Vec<u8> {
    let mut equities = vec![0; player_hands.len()];
    let win_counts = Arc::new(Mutex::new(HashMap::new()));
    let tie_counts = Arc::new(Mutex::new(HashMap::new()));

    let total_outcomes = deck.iter().combinations(2).count();

    deck.iter()
        .combinations(2)
        .par_bridge()  // This enables parallel iteration
        .for_each(|combo| {
            let turn_card = *combo[0];
            let river_card = *combo[1];

            let mut board = flop.clone().to_vec();
            board.push(turn_card);
            board.push(river_card);

            let hand_strengths: Vec<u16> = player_hands.iter()
                .map(|hand| get_nut_rank(&hand.to_vec(), &board, false).0,)
                .collect();

            let min_value = hand_strengths.iter().min().copied().unwrap_or(99);
            let winner_indexes: Vec<usize> = hand_strengths.iter()
                .enumerate()
                .filter(|&(_, &value)| value == min_value)
                .map(|(index, _)| index)
                .collect();

            let mut local_win_counts = HashMap::new();
            let mut local_tie_counts = HashMap::new();

            if winner_indexes.len() > 1 {
                for &win in &winner_indexes {
                    *local_tie_counts.entry(win).or_insert(0) += 1;
                }
            } else {
                let winner_index = winner_indexes[0];
                *local_win_counts.entry(winner_index).or_insert(0) += 1;
            }

            // Use a mutex or another form of synchronization to safely update the shared win_counts and tie_counts
            // This is a simplified example; in practice, you may need more sophisticated synchronization
            let wc = win_counts.clone();
            let tc = tie_counts.clone();

            if winner_indexes.len() > 1 {
                let mut tie_counts = tc.lock().unwrap();
                for &win in &winner_indexes {
                    *tie_counts.entry(win).or_insert(0) += 1;
                }
            } else {
                let mut win_counts = wc.lock().unwrap();
                let winner_index = winner_indexes[0];
                *win_counts.entry(winner_index).or_insert(0) += 1;
            }
        });

    let win_counts = win_counts.lock().unwrap();  // Lock the mutex to access the HashMap
    let tie_counts = tie_counts.lock().unwrap();  // Lock the mutex to access the HashMap
    let mut curr_total: u8 = 0;
    for index in 0..player_hands.len() {
        let wins = *win_counts.get(&index).unwrap_or(&0);
        let ties = *tie_counts.get(&index).unwrap_or(&0);
        // we work with u8 so 1 = 1% and 100 = 100%
        let score = (((wins + ties) as f32 / total_outcomes as f32) * 100 as f32).floor() as u8;
        if curr_total + score > 100 {
            equities[index] = 100 - curr_total;
        } else {
            equities[index] = score;
            curr_total += score;
        }
    }
    
    equities
        
}
pub fn equity(hands: &Vec<[Card; 4]>, comm: &[Card; 3]) -> Vec<u8> {
    calculate_equity(hands, comm, get_remaining_cards(&hands, &comm))
    // let mut cache = CACHE.lock().unwrap();

    // // Convert the inputs to a form that can be used as a key in the HashMap
    // let key = (hands.clone(), *comm);

    // // If the result is in the cache, return it
    // if let Some(result) = cache.get(&key) {
    //     // println!("equity cache hit {:?} result {:?}", key, result);
    //     return result.clone();
    // }

    // // Otherwise, calculate the result and store it in the cache
    // // let cal_eq_start = std::time::Instant::now();
    // let result = calculate_equity(hands, comm, get_remaining_cards(&hands, &comm));
    // // let cal_eq_end = std::time::Instant::now();
    // // println!("equity cache miss {:?} result {:?} time {:?}", key, result, cal_eq_end.duration_since(cal_eq_start));
    // cache.insert(key, result.clone());
    // result
}

// normalize equity function to make sure the sum of all equities is 100
// input would be like [81, 19, 9] 

pub fn normalize_equity(equities: &[u8]) -> Vec<u8> {
    let total: u32 = equities.iter().map(|&equity| equity as u32).sum();
    // println!("normalize equity total {} equities {:?}", total, equities);

    if total == 0 {
        return vec![0; equities.len()];
    }

    equities.iter()
            .map(|&equity| ((equity as f32 * 100.0) / total as f32).floor() as u8)
            .collect()
}