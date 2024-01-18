use crate::card::Card;
use std::collections::HashMap;

pub fn get_same_value_map(cards: &[Card]) -> Vec<Vec<Card>> {
    let mut map: HashMap<u8, Vec<Card>> = HashMap::new();

    // Group cards by value
    for &card in cards {
        map.entry(card.value).or_insert_with(Vec::new).push(card);
    }

    // Collect and sort the groups
    let mut res_cards: Vec<Vec<Card>> = map.into_iter().map(|(_, group)| group).collect();
    res_cards.sort_by(|a, b| b[0].value.cmp(&a[0].value));

    res_cards
}

pub fn sort_cards(cards: &Vec<Card>) -> Vec<Card> {
    let mut sorted_cards = cards.to_vec();
    sorted_cards.sort_by(|a, b| {
        b.value.cmp(&a.value).then_with(|| a.suit.cmp(&b.suit))
    });

    sorted_cards
}