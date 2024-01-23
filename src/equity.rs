use crate::get_nut_rank::get_nut_rank;
use std::collections::HashMap;
use crate::card::{Card, Suit};
use std::collections::HashSet;
use itertools::Itertools;
use rayon::prelude::*;
use std::sync::Mutex;
use std::sync::Arc;
pub fn get_remaining_cards(player_hands: &Vec<Vec<Card>>, board: &Vec<Card>) -> Vec<Card> {
    let mut allp_cards = player_hands.iter().flatten().cloned().collect::<Vec<Card>>();
    allp_cards.extend(board);
    let mut remaining = create_full_deck();
    remaining.retain(|c| !allp_cards.contains(c));
    remaining
}

fn create_full_deck() -> Vec<Card> {
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
fn calculate_equity(player_hands: &Vec<Vec<Card>>, flop: &Vec<Card>, deck: Vec<Card>) -> Vec<f64> {
    let mut equities = vec![0.0; player_hands.len()];
    let win_counts = Arc::new(Mutex::new(HashMap::new()));
    let tie_counts = Arc::new(Mutex::new(HashMap::new()));

    let total_outcomes = deck.iter().combinations(2).count();

    deck.iter()
        .combinations(2)
        .par_bridge()  // This enables parallel iteration
        .for_each(|combo| {
            let turn_card = *combo[0];
            let river_card = *combo[1];

            let mut board = flop.clone();
            board.push(turn_card);
            board.push(river_card);

            let hand_strengths: Vec<u16> = player_hands.iter()
                .map(|hand| get_nut_rank(&hand, &board).0)
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
    
    for index in 0..player_hands.len() {
        let wins = *win_counts.get(&index).unwrap_or(&0) as f64;
        let ties = *tie_counts.get(&index).unwrap_or(&0) as f64;
        equities[index] = (wins + ties) / total_outcomes as f64;
    }
    
    equities
        
}
pub fn equity(hands: Vec<Vec<Card>>, comm: Vec<Card>) -> Vec<f64> {
    calculate_equity(&hands,&comm, get_remaining_cards(&hands, &comm)) 
} 