use crate::get_nut_rank::get_nut_rank;
use std::collections::HashMap;
use crate::card::{Card, Suit};
use std::collections::HashSet;

pub fn get_remaining_cards(player_hands: Vec<Vec<Card>>, board: Vec<Card>) -> Vec<Card> {
    let mut deck = create_full_deck();
    let mut allp_cards = player_hands.iter().flatten().cloned().collect::<Vec<Card>>();
    allp_cards.extend(board);
    let mut remaining = create_full_deck();
    remaining.retain(|c| !allp_cards.contains(c))
    // let used_cards: HashSet<Card> = player_hands.into_iter().flatten()
    //                                              .chain(flop.into_iter())
    //                                              .collect();

    // deck.retain(|card| !used_cards.contains(card));
    // deck
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
fn calculate_equity(player_hands: Vec<Vec<Card>>, flop: Vec<Card>, deck: Vec<Card>) -> HashMap<usize, f64> {
    let mut equities: HashMap<usize, f64> = HashMap::new();
    let mut total_outcomes = 0;
    let mut win_counts: HashMap<usize, u32> = HashMap::new();
    let mut tie_counts: HashMap<usize, u32> = HashMap::new();

    // Iterate over all possible turn and river cards
    for &turn_card in &deck {
        for &river_card in &deck {
            if turn_card == river_card { continue; }  // Skip the same card for turn and river

            // Build the full board with turn and river cards
            let mut board = flop.clone();
            board.push(turn_card);
            board.push(river_card);

            // Calculate the hand strength for each player
            let mut hand_strengths: Vec<(usize, (u16, u8))> = player_hands.iter().enumerate().map(|(index, hand)| {
                let mut full_hand = hand.clone();
                full_hand.extend(&board);
                (index, get_nut_rank(&full_hand, &board))
            }).collect();

            // Determine winners and ties
            hand_strengths.sort_by(|a, b| a.1.cmp(&b.1).reverse());
            let best_hand = hand_strengths[0].1;
            let winners: Vec<_> = hand_strengths.iter().filter(|&hand| hand.1 == best_hand).collect();

            if winners.len() > 1 {
                // It's a tie
                for &(index, _) in &winners {
                    *tie_counts.entry(index).or_insert(0) += 1;
                }
            } else {
                // Single winner
                let winner_index = winners[0].0;
                *win_counts.entry(winner_index).or_insert(0) += 1;
            }

            total_outcomes += 1;
        }
    }

    // Calculate equity for each player
    for (index, _) in player_hands.iter().enumerate() {
        let wins = *win_counts.get(&index).unwrap_or(&0) as f64;
        let ties = *tie_counts.get(&index).unwrap_or(&0) as f64;
        let equity = (wins + ties / winners.len() as f64) / total_outcomes as f64;
        equities.insert(index, equity);
    }

    equities
}

pub fn equity(hands: Vec<Vec<Card>>, comm: Vec<Card>) -> HashMap<usize, f64> {
    calculate_equity(hands,comm, get_remaining_cards(hands, comm)) 
} 