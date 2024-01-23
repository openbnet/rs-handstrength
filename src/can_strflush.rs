use crate::card::Card;
use crate::sorting::sort_cards;
use crate::can_libs::*;

pub fn can_have_straight_flush(comcards: &Vec<Card>) -> CanHaveCombis {
    let sorted = sort_cards(comcards);
    let mut straight_flush_combinations: CanHaveCombis = Vec::new();
    // println!("cah have str flush called {:?}", comcards);
    let flushsuit = can_flush( &sorted);
    // println!("flush suit {:?}", flushsuit);
    if let Some(flush_suit) = can_flush(&sorted) {
        let flush_suit_cards = add_ace_as_high(
            &sorted.iter()
                  .filter(|card| card.suit == flush_suit)
                  .cloned()
                  .collect()
        );
        let length = flush_suit_cards.len();
        for i in 0..length {
      
            for j in i + 1..length {
                for k in j + 1..length {
                    let combination = vec![flush_suit_cards[i], flush_suit_cards[j], flush_suit_cards[k]];
                    let gaps = calculate_gaps(&combination);
                    // println!("Combination: {:?}, Gaps: {:?}", combination, gaps);

                    if gaps.0 <= 2 {
                        match is_blocked(&combination, &flush_suit_cards, gaps) {
                            Ok(possible_combinations) => {
                                // println!("Not Blocked: {:?}", combination);
                                straight_flush_combinations.push((combination, possible_combinations));
                            },
                            Err(_) => {
                                // println!("Blocked: {:?}", combination);
                            }
                        }
                    } else {
                        // println!("Too many gaps: {:?}", combination);
                    }
                }
            }
        }
    }
    // println!("Straight Flush Combinations: {:?}", straight_flush_combinations);
    handle_ace_low(straight_flush_combinations)
}

pub fn handle_ace_low(combinations: CanHaveCombis) -> CanHaveCombis {
    let mut modified_combinations: CanHaveCombis = Vec::new();

    for (straight_flush_combination, blocking_combinations) in combinations {
        let mut modified_blocking_combinations = Vec::new();
        let contains_ace_high = straight_flush_combination.iter().any(|card| card.value == 14);
        let contains_ace_low = straight_flush_combination.iter().any(|card| card.value == 1);

        for combo in blocking_combinations {
            // Handle Ace as high (14) or low (1), but not both
            if contains_ace_high {
                // Convert Ace to high if it's part of the combination
                let ace_high_combo = convert_ace_to_high(&combo);
                modified_blocking_combinations.push(ace_high_combo);
            } else if contains_ace_low {
                // Convert Ace to low if it's part of the combination
                let ace_low_combo = convert_ace_to_low(&combo);
                modified_blocking_combinations.push(ace_low_combo);
            } else {
                // If no Ace is involved, keep the combination as is
                modified_blocking_combinations.push(combo);
            }
        }

        // Deduplicate the combinations
        modified_blocking_combinations.sort();
        modified_blocking_combinations.dedup();

        modified_combinations.push((straight_flush_combination, modified_blocking_combinations));
    }

    modified_combinations
}

// Helper functions to convert Ace to high or low
fn convert_ace_to_high(combo: &Vec<CardHand>) -> Vec<CardHand> {
    combo.iter().map(|hand| {
        hand.iter().map(|card| {
            if card.value == 1 { Card { value: 14, suit: card.suit } } else { *card }
        }).collect()
    }).collect()
}

fn convert_ace_to_low(combo: &Vec<CardHand>) -> Vec<CardHand> {
    combo.iter().map(|hand| {
        hand.iter().map(|card| {
            if card.value == 14 { Card { value: 1, suit: card.suit } } else { *card }
        }).collect()
    }).collect()
}

pub fn is_blocked(straight_flush_cards: &[Card],all_flush_cards: &Vec<Card>, gaps: (u8, Vec<Vec<u8>>)) -> Result<Vec<SameRankHands>, bool> {
    if gaps.0 > 2 {
        return Err(true); // Too many gaps, blocked by default
    }

    let values: Vec<u8> = straight_flush_cards.iter().map(|card| card.value).collect();
    let highest = *values.iter().max().unwrap();
    let lowest = *values.iter().min().unwrap();
    let suit = straight_flush_cards[0].suit; // Assuming all straight flush cards have the same suit
    let not_my_cards: Vec<Card> = all_flush_cards.iter()
        .filter(|&card| !straight_flush_cards.contains(card))
        .cloned()
        .collect();
    let not_my_values: Vec<u8> = not_my_cards.iter().map(|card| card.value).collect();
    // println!("is blocked start {:?} {:?}", values, not_my_cards);
    match gaps.0 {
        0 => {
            // Check for cards above and below the sequence
            let mut possible = Vec::new();
            // println!("0 gapper {:?} {:?}", values, not_my_values);
            if !not_my_values.contains(&(highest + 1)) && !not_my_values.contains(&(highest + 2)) {
                possible.push(vec![vec![
                    Card { value: highest + 1, suit },
                    Card { value: highest + 2, suit },
                ]]);
            }
            if lowest > 1 && highest < 14 && !not_my_values.contains(&(lowest - 1)) && !not_my_values.contains(&(highest +1)) {
                possible.push(vec![vec![
                    Card { value: highest + 1, suit },
                    Card { value: lowest - 1, suit },
                ]]); 
            }
            if lowest > 2 && !not_my_values.contains(&(lowest - 1)) && !not_my_values.contains(&(lowest - 2)) {
                possible.push(vec![vec![
                    Card { value: lowest - 1, suit },
                    Card { value: lowest - 2, suit },
                ]]);
            }

            if possible.is_empty() { Err(true) } else { Ok(possible) }
        },
        1 => {
            // Handle the single gap
            let gap_value = gaps.1[0][0];
            if not_my_values.contains(&gap_value) {
                Err(true)
            } else {
                let mut possible = Vec::new();
                if highest < 14 && !not_my_values.contains(&(highest + 1)) {
                    possible.push(vec![vec![
                        Card { value: highest + 1, suit },
                        Card { value: gap_value, suit },
                    ]]);
                }
                if lowest > 1 && !not_my_values.contains(&(lowest - 1)) {
                    possible.push(vec![vec![
                        Card { value: gap_value, suit },
                        Card { value: lowest - 1, suit },
                    ]]);
                }
                if possible.is_empty() { Err(true) } else { Ok(possible) }
            }
        },
        2 => {
            // Handle two 1-gappers or one 2-gapper
            let first_gap = &gaps.1[0];

            let mut possible = Vec::new();
            // println!("is blocked 2 {:?} {:?} {:?} first gap {:?}", gaps, gaps.1, gaps.1.len(), first_gap[0]);
            // Check if it's two 1-gappers or one 2-gapper
            if first_gap.len() == 2 {
                // It's a single 2-gapper
                possible.push(Card { value: first_gap[0], suit });
                possible.push(Card { value: first_gap[1], suit });
            } else {
                // It's two separate 1-gappers
                let second_gap = &gaps.1[1];
                // println!("second_gap {:?}", second_gap[0]);
                possible.push(Card { value: first_gap[0], suit });
                possible.push(Card { value: second_gap[0], suit });
            }

            if possible.is_empty() { Err(true) } else { Ok(
                vec![vec![possible]]
            ) }
        },
        _ => Err(true) // Any other case is considered blocked
    }
}