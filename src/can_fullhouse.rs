use crate::card::{Card, Suit};
use crate::sorting::get_same_value_map;
use crate::can_libs::*;
pub fn can_have_fullhouse(cards:  &Vec<Card>) -> CanHaveCombis {
    let grouped_cards = get_same_value_map(&replace_ace_as_high(cards));
    let mut fh_combis: CanHaveCombis = Vec::new();
    let has_pair = grouped_cards.iter().any(|group| group.len() == 2);
    let has_trips = grouped_cards.iter().any(|group| group.len() == 3);
    let has_two_pair = grouped_cards.iter().filter(|group| group.len() == 2).count() > 1;
    println!("grouped_cards {:?}", grouped_cards);
    if !has_pair && !has_trips && !has_two_pair {
        return fh_combis;
    } else {
        // do nothing
    }
    // if has trips, pairs are ignored anyway
    if has_trips {
        let trips: Vec<Vec<Card>> = grouped_cards.iter().filter(|group| group.len() == 3).cloned().collect();
        println!("group 1 {:?}", trips);
        let trips_value = trips[0][0].value;

        // Handle overhouses
        let higher_groups: Vec<Vec<Card>> = grouped_cards.iter()
            .filter(|group1| group1[0].value > trips_value)
            .cloned()
            .collect();

        for higher_group in &higher_groups {
            if higher_group.len() == 1 {
                let higher_card = higher_group[0].clone();
                fh_combis.push((
                    vec![higher_card.clone(), trips[0][0], trips[0][1]],
                    vec![vec![Card {value: higher_card.value, suit: Suit::A }, Card {value: trips_value, suit: Suit::A }]]
                ));
            }
        }

        // Handle trips
        let higher_group_values: Vec<u8> = higher_groups.iter()
            .map(|group| group[0].value)
            .collect();
        let lower_pair_groups: Vec<Vec<Card>> = grouped_cards.iter()
            .filter(|group1| group1[0].value < trips_value && group1.len() == 2)
            .cloned()
            .collect();
        let lower_pair_values: Vec<u8> = lower_pair_groups.iter()
        .map(|group| group[0].value)
        .collect();  
        let all_values = (2..=14).rev();
        let mut g_combis: Vec<Vec<Card>> = Vec::new();
        for value in all_values.filter(|value| !higher_group_values.contains(value) && *value != trips_value && !lower_pair_values.contains(value)) {
            // println!("doing value {:?} {:?} {:?}", value, higher_group_values, grouped_cards);
            g_combis.push(
                vec![
                    Card { value, suit: Suit::A },
                    Card { value, suit: Suit::A }
            ]);
        }
        // println!("gona push {:?} {:?}", trips[0], g_combis);
        fh_combis.push((
            trips[0].to_vec(),
            g_combis 
        ))
    }

    else if has_pair {

    let pairs: Vec<Vec<Card>> = grouped_cards.iter().filter(|group| group.len() == 2).cloned().collect();
    for (i, pair) in pairs.iter().enumerate() {
        let pair_value = pair[0].value;

        // Find hands higher than the pair
        let not_my_hands_higher = grouped_cards.iter()
            .filter(|group| group[0].value > pair_value && group.len() == 1)
            .collect::<Vec<_>>();

        // Find hands lower than the pair
        let not_my_hands_lower = grouped_cards.iter()
            .filter(|group| group[0].value < pair_value)
            .collect::<Vec<_>>();

        // first pair
        if i == 0 {
            for higher_hand in &not_my_hands_higher {
                let higher_card = higher_hand[0];
                fh_combis.push((
                    vec![higher_card, pair[0], pair[1]],
                    vec![vec![Card { value: higher_card.value, suit: Suit::A }, Card { value: higher_card.value, suit: Suit::A }]]
                ));
            }
        }

        // 2nd pair
        if i == 1  {
            let middle_cards = grouped_cards.iter()
            .filter(|group| group[0].value > pair_value && group[0].value < pair[0].value)
            .collect::<Vec<_>>();
            for not_my_hand in &middle_cards {
                let higher_card = not_my_hand[0];
                fh_combis.push((
                    vec![higher_card, pair[0], pair[1]],
                    vec![vec![Card { value: higher_card.value, suit: Suit::A }, Card { value: higher_card.value, suit: Suit::A }]]
                ));
            }
        }
        // using pair as trips

        for not_my_hand in &not_my_hands_higher {
            let higher_card = not_my_hand[0];
            fh_combis.push((
                vec![higher_card, pair[0], pair[1]],
                vec![vec![Card {value: higher_card.value, suit: Suit::A}, Card {value: pair_value, suit: Suit::A}]]
            ))
            
        }
        for not_my_hand in &not_my_hands_lower {
                let lower_card = not_my_hand[0];
                fh_combis.push((
                    vec![pair[0], pair[1], lower_card],
                    vec![vec![Card {value: pair_value, suit: Suit::A}, Card { value: lower_card.value, suit: Suit::A }]]
                ))
                
        }
        // handle lower trips then pair
        if i == 1 || pairs.len() == 1 {
            for lower_hand in &not_my_hands_lower {
                let lower_card = lower_hand[0];
                fh_combis.push((
                    vec![pair[0], pair[1], lower_card],
                    vec![vec![Card { value: lower_card.value, suit: Suit::A }, Card { value: lower_card.value, suit: Suit::A }]]
                ));
            }
        }
    }

    
    }
    // println!("fh combis end");
    fh_combis
}