use crate::card::Card;
use crate::get_nut_rank::get_nut_rank;

pub fn get_winner(phands: &Vec<Vec<Card>>, comcards: &Vec<Card>) -> Vec<usize> {
    // Calculate scores for each hand in parallel
    let scores = phands.iter()
        .map(|hand| get_nut_rank(hand, comcards, false).0)
        .collect::<Vec<u16>>();

    // Find the lowest score (winner score)
    let winner_score = match scores.iter().min() {
        Some(&score) => score,
        None => return Vec::new(), // Return empty if no hands are given
    };

    // Find all indexes with the winner score in parallel
    scores.iter().enumerate()
        .filter(|&(_index, &score)| score == winner_score)
        .map(|(index, _)| index)
        .collect()
}