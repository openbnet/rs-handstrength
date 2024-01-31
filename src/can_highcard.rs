use crate::card::{Card, Suit};
use itertools::Itertools;
use crate::can_libs::*;

pub fn can_have_highcard(comcards: &Vec<Card>) -> CanHaveCombis {
    let com_ace = replace_ace_as_high(comcards);
    let com_values = com_ace.iter().map(|c| c.value).collect::<Vec<u8>>();
    let all_values: Vec<u8> = (2..=14).rev().collect(); 
    let non_com_values: Vec<u8> = all_values.iter()
    .filter(|av| !com_values.contains(av))
    .cloned()
    .collect();
    let kickers = non_com_values.iter().combinations(2).collect::<Vec<_>>();
    let ranked_kickers = kickers.iter().map(
        |v| vec![vec![Card {
            value: *v[0], suit: Suit::A
        },
        Card {
            value: *v[1], suit: Suit::A
        }
        
        ]]
    ).collect::<Vec<Vec<Vec<Card>>>>();
    // println!("non com values com {:?}", kickers);
    vec![(
        comcards.clone(),
        ranked_kickers
    )]
}