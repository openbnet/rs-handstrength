use crate::card::{Card, Suit};
use crate::sorting::sort_cards;
use crate::can_libs::*;
use std::collections::HashMap;
// does not care about straight flushes and will return a str flush match as a flush
pub fn can_have_flush(comcards: &Vec<Card>) -> CanHaveCombis {
    if let Some(flush_suit) = can_flush(comcards) {
        let flush_suit_cards: Vec<Card> = comcards.iter()
            .filter(|card| card.suit == flush_suit)
            .cloned()
            .collect();

        let sorted_flush_suit_cards = sort_cards(&flush_suit_cards);

        let unused_flush_cards = get_unused_flush_cards(&sorted_flush_suit_cards, flush_suit);
        println!("unused_flush_cards {:?} board {:?}", unused_flush_cards, sorted_flush_suit_cards[..3].to_vec());
        vec![(sorted_flush_suit_cards[..3].to_vec(), generate_two_card_combos(&unused_flush_cards))]
    } else {
        Vec::new() // No flush suit found
    }
}

fn get_unused_flush_cards(comcards: &Vec<Card>, suit: Suit) -> Vec<Card> {
    let used_values: Vec<u8> = comcards.iter().map(|c| c.value).collect();
    let all_values: [u8; 13] = [1, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2];
    all_values.iter().filter(
        |v| used_values.contains(v) == false
    ).map(|v| Card {
        value: *v,
        suit
    }).collect::<Vec<Card>>()
} 
pub fn generate_two_card_combos(cards: &[Card]) -> Vec<SameRankHands> {
    let mut combinations: Vec<SameRankHands> = Vec::new();
    for i in 0..cards.len() {
        for j in i + 1..cards.len() {
            combinations.push(vec![vec![cards[i], cards[j]]]);
        }
    }
    combinations
}