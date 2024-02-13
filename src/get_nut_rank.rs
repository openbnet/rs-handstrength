
use std::collections::HashMap;
use std::sync::Mutex;

use crate::can_strflush::*;
use crate::can_quads::*;
use crate::can_fullhouse::*;
use crate::can_flush::*;
use crate::can_str::*;
use crate::can_trips::*;
use crate::can_twopair::*;
use crate::can_pair::*;
use crate::can_highcard::*;
use crate::can_libs::*;
use crate::card::{Card, Suit};
use lazy_static::lazy_static;

lazy_static! {
    static ref CACHE: Mutex<HashMap<(Vec<Card>, Vec<Card>), (u16, NutRankType)>> = Mutex::new(HashMap::new());
}
#[derive(Debug,Clone, PartialEq, Eq)]
pub enum NutRankType {
    StraightFlush,
    Quads,
    FullHouse,
    Flush,
    Straight,
    Trips,
    TwoPair,
    Pair,
    HighCard
}
pub fn get_nut_rank(hand: &Vec<Card>, comcards: &Vec<Card>, relative: bool) -> (u16, NutRankType) {
    // get_nut_rank_uncached(hand, comcards, relative)
    let mut cache = CACHE.lock().unwrap();
    let key = (hand.clone(), comcards.clone());
    if let Some((rank, nut_rank_type)) = cache.get(&key) {
        return (*rank, nut_rank_type.clone());
    }
    let (rank, nut_rank_type) = get_nut_rank_uncached(hand, comcards, relative);
    cache.insert((hand.clone(), comcards.clone()), (rank, nut_rank_type.clone()));
    (rank, nut_rank_type)
}
pub fn get_nut_rank_uncached(hand: &Vec<Card>, comcards: &Vec<Card>, relative: bool) -> (u16, NutRankType) {
    let can_str_flush = can_have_straight_flush(comcards);
    // println!("can_str_flush {:?}", can_str_flush);
    let (hit, rank) = is_subset(hand, can_str_flush, relative); 
    // println!("str flush hit {:} rank {:}", hit, rank);
    if hit {
        return (rank, NutRankType::StraightFlush);
    }
    let mut score = rank.clone();
    let can_quads = can_have_quads(comcards);
    let (hitq, rankq) = is_subset(hand, can_quads, relative); 
    // println!("quads  {:} rank {:}", hitq, rankq);
    score += rankq; 
    if hitq {
        return (score, NutRankType::Quads);
    }
    

    let can_fh = can_have_fullhouse(comcards);
    let (hitfh, rankfh) = is_subset(hand, can_fh, relative); 
    // println!("can_fh  {:} rank {:}", hitfh, rankfh);
    score += rankfh; 
    
    if hitfh {
        return (score, NutRankType::FullHouse);
    }

    let can_fl = can_have_flush(comcards);
    let (hitfl, rankfl) = is_subset(hand, can_fl, relative); 
    // println!("can_fl  {:} rank {:}", hitfl, rankfl);
    score += rankfl; 
    if hitfl {
        return (score, NutRankType::Flush);
    }


    let can_str = can_have_str(comcards);
    let (hitstr, rankstr) = is_subset(hand, can_str, relative); 
    // println!("can_str  {:} rank {:}", hitstr, rankstr);
    score += rankstr; 
    if hitstr {
        return (score, NutRankType::Straight);
    }

    let can_trips = can_have_trips(comcards);
    let (hittrips, ranktrips) = is_subset(hand, can_trips, relative); 
    // println!("can_trips  {:} rank {:}", hittrips, ranktrips);
    score += ranktrips; 
    if hittrips {
        return (score, NutRankType::Trips);
    }

    let can_2pair = can_have_twopairs(comcards);
    let (hit2p, rank2p) = is_subset(hand, can_2pair, relative); 
    // println!("can_2pair  {:} rank {:}", hit2p, rank2p);
    score += rank2p; 
    if hit2p {
        return (score, NutRankType::TwoPair);
    }


    let can_pair = can_have_pair(comcards);
    let (hitp, rankp) = is_subset(hand, can_pair, relative); 
    // println!("can_pair  {:} rank {:}", hitp, rankp);
    score += rankp; 
    if hitp {
        return (score, NutRankType::Pair);
    }

    let can_hc = can_have_highcard(comcards);
    let (hithc, rankhc) = is_subset(hand, can_hc, relative); 
    // println!("can_hc  {:} rank {:}", hithc, rankhc);
    score += rankhc; 

    return (score, NutRankType::HighCard);
    
    
}
pub fn is_subset(hand: &Vec<Card>, combis: CanHaveCombis, relative: bool) -> (bool, u16) {
    let mut rank = 0;

    for (board, same_rank_hands) in combis {
        if relative {
            // println!("is subset relative board {:?} {:?}", board, hand);
            // [Card { value: 13, suit: C }, Card { value: 6, suit: C }, Card { value: 5, suit: C }] 
            // [Card { value: 13, suit: H }, Card { value: 8, suit: C }, Card { value: 7, suit: C }, Card { value: 10, suit: C }]
            // we need to see if any of the board cards are in hand
            // if there is any single match, we need to ignore this whole board combi if relative is on
            // else proceed as normal
            // however the tricky thing is to handle the suit::A case for the board
            // if its the board's suit::A, it should be ignored if the num of card value in hand + num of card value on board > 4

            
            let mut should_pass = true;
            for bc in &board {
                if bc.suit == Suit::A {
                  let num_in_board = board.iter().filter(|&card| card.value == bc.value).count();
                  let num_in_hand = hand.iter().filter(|&card| card.value == bc.value).count();
                    if num_in_board + num_in_hand > 4 {
                        should_pass = false;
                        break;
                    }  
                } else {
                    if hand.contains(&bc) {
                        should_pass = false;
                        break;
                    }
                }

            }

            if !should_pass {
                continue;
            }


        }
        for same_rank in same_rank_hands {
            if relative {
                let filtered_same_rank: Vec<Vec<Card>> = same_rank.iter()
                    .filter(|&handmatcher| {
                        for hc in handmatcher {
                            if hc.suit == Suit::A {
                                let num_in_board = board.iter().filter(|&card| card.value == hc.value).count();
                                let num_in_hand = hand.iter().filter(|&card| card.value == hc.value).count();
                                if num_in_board + num_in_hand > 4 {
                                    return false;
                                }  
                            } else {
                                if hand.contains(&hc) {
                                    return false;
                                }
                            }
                        }
                        return true;
                    })
                    .cloned()
                    .collect();
                let (hit, score) = matches_hand(hand, &filtered_same_rank);
                // println!("matches hand hit {:?} score {:?} rank {:?}", hit, score, rank);
                rank += score;
                // println!("rank {:?}", rank);
                if hit {
                    return (true, rank);
                }
            } else {
                let (hit, score) = matches_hand(hand, &same_rank);
                // println!("matches hand hit {:?} score {:?} rank {:?}", hit, score, rank);
                rank += score;
                // println!("rank {:?}", rank);
                if hit {
                    return (true, rank);
                }
            }
        }
    }
    (false, rank)
}

fn matches_hand(hand: &Vec<Card>, same_rank: &Vec<CardHand>) -> (bool, u16) {
    let mut score: u16 = 0;
    for hand_matcher in same_rank {
        if is_hand_match(hand, hand_matcher) {
            return (true, score);
        }
        score += 1;
    }
    (false, score)
}
// Helper function to determine if a hand matches a specific hand matcher
// matches any and one
fn is_hand_match(hand: &Vec<Card>, hand_matcher: &CardHand) -> bool {
    let mut matched_cards = 0;
    let mut r_hand = hand.clone();
    for matcher_card in hand_matcher {
        if let Some(index) = r_hand.iter().position(|card| matches_card(card, matcher_card)) {
            matched_cards += 1;
            // Remove the matched card from the remaining hand
            r_hand.remove(index);
        } else {
            return false;
        }
    }

    matched_cards == hand_matcher.len()
}

// Helper function to check if a single card matches a matcher card
fn matches_card(card: &Card, matcher_card: &Card) -> bool {
    (matcher_card.suit == Suit::A || card.suit == matcher_card.suit) && card.value == matcher_card.value
}

// fn get_cards_left_matchers(index: usize, hand: [Card;4], pEval)