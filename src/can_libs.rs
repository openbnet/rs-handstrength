use crate::card::{Card, Suit};

pub type CardHand = Vec<Card>;
pub type SameRankHands = Vec<CardHand>;
pub type PEval = Vec<SameRankHands>;
pub type CanHaveCombis = Vec<(Vec<Card>, PEval)>;
pub fn can_flush(comcards: &Vec<Card>) -> Option<Suit> {
    let mut suit_counts = std::collections::HashMap::new();

    for card in comcards {
        *suit_counts.entry(card.suit).or_insert(0) += 1;
    }

    suit_counts.into_iter()
        .find(|&(_, count)| count >= 3)
        .map(|(suit, _)| suit)
}
pub fn can_flush_hand(hand: &Vec<Card>) -> Option<Suit> {
    let mut suit_counts = std::collections::HashMap::new();

    for card in hand {
        *suit_counts.entry(card.suit).or_insert(0) += 1;
    }

    suit_counts.into_iter()
        .find(|&(_, count)| count >= 2)
        .map(|(suit, _)| suit)
}

// cal gaps EXPECTS desc sorted combi of 3 only!
pub fn calculate_gaps(sorted_combination: &[Card]) -> (u8, Vec<Vec<u8>>) {
    let mut gaps: Vec<Vec<u8>> = Vec::new();
    let mut num_gaps = 0;

    for i in 0..sorted_combination.len() - 1 {
        let diff = sorted_combination[i].value as i32 - sorted_combination[i + 1].value as i32 - 1;
        if diff < 1 {
            continue;
        }

        let mut current_gap: Vec<u8> = Vec::new();
        for y in 1..=diff {
            current_gap.push((sorted_combination[i].value as i32 - y) as u8);
            num_gaps += 1;
        }

        gaps.push(current_gap);
    }

    (num_gaps as u8, gaps)
}
pub fn add_ace_as_high(cards: &Vec<Card>) -> Vec<Card> {
    let mut modified_cards = cards.clone();

    // For each Ace, add an additional Ace with value 14 and the same suit
    for card in cards.iter().filter(|&card| card.value == 1) {
        modified_cards.insert(0, Card { value: 14, suit: card.suit });
    }

    modified_cards
}
pub fn replace_ace_as_high(cards: &Vec<Card>) -> Vec<Card> {
    let mut modified_cards = Vec::new();

    // For each Ace, add an additional Ace with value 14 and the same suit
    for card in cards.iter() {
        if card.value != 1 {
            modified_cards.push(card.clone());
        } else {
            modified_cards.insert(0, Card { value: 14, suit: card.suit });
        }

    }
    modified_cards
}