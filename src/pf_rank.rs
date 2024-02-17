use itertools::{Itertools, GroupBy};
use std::collections::HashMap;
use crate::card::{Card, Suit};
pub fn pf_rank(cards: [Card; 4]) -> u16 {
    let suit_obj: HashMap<_, _> = cards.iter().group_by(|c| c.suit).into_iter().map(|(key, group)| (key, group.collect::<Vec<_>>())).collect();
    let value_obj: HashMap<_, _> = cards.iter().group_by(|c| c.value).into_iter().map(|(key, group)| (key, group.collect::<Vec<_>>())).collect();
    
    let mut total: f32 = 0 as f32;
    // process suits
    for s in suit_obj.values() {
        for card in s {
            let card_v = card.value;
            let multiplier: f32 = match s.len() {
                3 => 0.5,
                2 => 2 as f32,
                _ => 0 as f32
            };
            total += multiplier * adjust_card_value(card_v) as f32;
        }
    }  
  
    // process values
    for v in &value_obj {
        let value = v.0;
        let multiplier: f32 = match v.1.len() {
            3 => 0.5,
            2 => 2.5,
            _ => 0 as f32
        };
        total += multiplier * adjust_card_value(*value) as f32;
    }

    process_straightness(
        &cards, 
        value_obj, 
        total) as u16
}

fn adjust_card_value(value: u8) -> u8 {
    match value {
        1 | 14 => 18,
        13 => 14,
        _ => value
    }
}

fn process_straightness(cards: &[Card], value_obj: HashMap<u8, Vec<&Card>>, mut total: f32) -> f32 {
    for (value, _) in value_obj {
        let other_cards: Vec<&Card> = cards.iter().filter(|&card| card.value != value).collect();
        total = process_card_for_straightness(value, &other_cards, total);
    }
    total
}

fn process_card_for_straightness(value: u8, other_cards: &[&Card], mut total: f32) -> f32 {
    let mut used_values = Vec::new();

    for &card in other_cards {
        let card_value = card.value;

        if !used_values.contains(&card_value) {
            used_values.push(card_value);
            let diff = calculate_difference(value, card_value);
            total = update_total_for_straightness(total, value, diff);
        }
    }
    total
}

fn calculate_difference(v1: u8, v2: u8) -> u8 {
    if [1, 14].contains(&v1) && [1, 14].contains(&v2) {
        0
    } else 
    if [1, 14].contains(&v1) {
        ((1.0 - v2 as f32).abs()).min(((14.0 - v2 as f32).abs())) as u8
    }
    else if [1, 14].contains(&v2) {
        ((1.0 - v1 as f32).abs()).min(((14.0 - v1 as f32).abs())) as u8
    }
    else {
        (v1 as i32 - v2 as i32).abs() as u8
    }
}

fn update_total_for_straightness(total: f32, value: u8, diff: u8) -> f32 {
    let multiplier = match diff {
        1 => value as f32 / 10.0,
        2 => value as f32 / 20.0,
        3 => value as f32 / 50.0,
        4 => value as f32 / 100.0,
        _ => 0.0,
    };
    total + multiplier
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pf_rank() {
        let hand = [Card { value: 7, suit: Suit::S }, Card { value: 1, suit: Suit::D }, Card { value: 13, suit: Suit::S}, Card {value: 12, suit: Suit::S}];
        let rank = pf_rank(hand);
        // println!("pf rank {:?}", rank);
        assert_eq!(rank, 61);
    }
}