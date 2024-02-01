use crate::card::{Card, Suit};
use crate::sorting::*;
use itertools::Itertools;
use crate::can_libs::*;

pub fn can_have_pair(comcards: &Vec<Card>) -> CanHaveCombis {
    let mut can_have_combis: CanHaveCombis = Vec::new();
    let sorted = sort_cards(&replace_ace_as_high(&comcards));
    let sorted_values: Vec<u8> = sorted.iter().map(|s| s.value).collect();
    let grouped_cards = get_same_value_map(&sorted);
    let pair_groups: Vec<Vec<Card>> = grouped_cards.iter().filter(|g| g.len() == 2).cloned().collect();
    // println!("pair_groups {:?}", pair_groups);
    let all_values: Vec<u8> = (2..=14).rev().collect(); 
    if pair_groups.len() == 0 {
        for v in &all_values {
            if sorted_values.contains(&v) == false {
                // push user pair in hand here
                can_have_combis.push((
                    sorted.clone(),
                    vec![vec![vec![Card {value: *v, suit: Suit::A}, Card { value: *v, suit: Suit::A}]]]
                ))
            } 
            if sorted_values.contains(&v) == true {
                // User hits the board for a pair
                let kickers = all_values.iter()
                    .filter(|&kv| !sorted_values.contains(kv) && kv != *&v)
                    .cloned()
                    .collect::<Vec<u8>>();
    
                let hand_combinations = kickers.into_iter()
                    .map(|kicker| vec![Card { value: *v, suit: Suit::A }, Card {value: kicker, suit: Suit::A}])
                    .collect::<Vec<Vec<Card>>>();
    
                can_have_combis.push((sorted.clone(), vec![hand_combinations]));
            } 
    
        } 
    }
    else {
        // pair on board
        // user cant have pair in hand or else its two pair
        // user cant hit anything on board or its two pair
        let first_pair_value = pair_groups[0][0].value;
        let kickers = all_values.iter()
            .filter(|&kv| !sorted_values.contains(kv) )
            .cloned()
            .collect::<Vec<u8>>();
        // println!("kickers {:?}", &kickers);
        let hand_combinations = kickers.into_iter()
            .combinations(2)
            .map(|kicker| vec![vec![Card { value: kicker[0], suit: Suit::A }, Card {value: kicker[1], suit: Suit::A}]])
            .collect::<Vec<Vec<Vec<Card>>>>();

        can_have_combis.push((sorted.clone(), hand_combinations));
    }



    can_have_combis
}