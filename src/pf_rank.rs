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
            let multiplier: f32 = match cards.len() {
                4 => 1.2,
                3 => 1.6,
                2 => 2 as f32,
                _ => 0 as f32
            };
            total += multiplier * adjust_card_value(card_v) as f32;
        }
    }  
  
    // process values
    for v in &value_obj {
        let value = v.0;
        let cards = v.1;
        let multiplier: f32 = match cards.len() {
            3 => 1.2,
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
        let card_value = adjust_card_value(card.value);

        if !used_values.contains(&card_value) {
            used_values.push(card_value);
            let diff = calculate_difference(value, card_value);
            total = update_total_for_straightness(total, value, diff);
        }
    }
    total
}

fn calculate_difference(v1: u8, v2: u8) -> u8 {
    let adjusted_v1 = adjust_card_value(v1);
    let adjusted_v2 = adjust_card_value(v2);
    let diff = (adjusted_v1 as i32 - adjusted_v2 as i32).abs() as u8;

    if [1, 14].contains(&v1) || [1, 14].contains(&v2) {
        diff.min((1i32 - adjusted_v2 as i32).abs() as u8)
    } else {
        diff
    }
}

fn update_total_for_straightness(total: f32, value: u8, diff: u8) -> f32 {
    let adjusted_value = adjust_card_value(value) as f32;
    let multiplier = match diff {
        1 => adjusted_value / 1.5,
        2 => adjusted_value / 2.5,
        3 => adjusted_value / 3.5,
        4 => adjusted_value / 5.0,
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
        println!("pf rank {:?}", rank);
        assert_eq!(rank, 61);
    }
}