use crate::get_nut_rank::get_nut_rank;
use std::collections::HashMap;
use crate::card::{Card, Suit};
use std::collections::HashSet;
use itertools::Itertools;
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
    let mut win_counts: HashMap<usize, u32> = HashMap::new();
    let mut tie_counts: HashMap<usize, u32> = HashMap::new();
    let mut total_outcomes = 0;
    for combo in deck.iter().combinations(2) {
        let turn_card = *combo[0];
        let river_card = *combo[1];

        let mut board = flop.clone();
        board.push(turn_card);
        board.push(river_card);

        let mut hand_strengths: Vec<u16> = player_hands.iter()
        .map(|hand| get_nut_rank(&hand, &board).0)
        .collect();


      
        // Inside calculate_equity, after determining winners
        println!("Board: {:?}, Hand Strengths: {:?}", board, hand_strengths);
        let min_value = match hand_strengths.iter().min() {
            Some(&min) => min,
            None => 99
        };
        let winner_indexes: Vec<usize> = hand_strengths.iter()
        .enumerate()
        .filter(|&(_, &value)| value == min_value)
        .map(|(index, _)| index)
        .collect();

        println!("min {:?}", winner_indexes);
        if winner_indexes.len() > 1 {
            for &win in &winner_indexes {
                *tie_counts.entry(usize::from(win)).or_insert(0) += 1;
            }
        } else {
            let winner_index = winner_indexes[0];
            *win_counts.entry(usize::from(winner_index)).or_insert(0) += 1;
        }

        total_outcomes += 1;
    }
    println!("player_hands {:?}", player_hands.len());
    for index in 0..player_hands.len() {
        let wins = *win_counts.get(&index).unwrap_or(&0) as f64;
        let ties = *tie_counts.get(&index).unwrap_or(&0) as f64;
        println!("index {:?} wins {:?} ties {:?}", index, wins, ties);
        equities[index] = (wins + ties) / total_outcomes as f64;
    }
    equities
}

pub fn equity(hands: Vec<Vec<Card>>, comm: Vec<Card>) -> Vec<f64> {
    calculate_equity(&hands,&comm, get_remaining_cards(&hands, &comm)) 
} 