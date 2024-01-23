
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

pub fn get_nut_rank(hand: &Vec<Card>, comcards: &Vec<Card>) -> (u16, u8) {
    let can_str_flush = can_have_straight_flush(comcards);
    let (hit, rank) = is_subset(hand, can_str_flush); 
    // println!("str flush hit {:} rank {:}", hit, rank);

    if hit {
        return (rank, 0);
    }
    let mut score = rank.clone();
    let can_quads = can_have_quads(comcards);
    let (hitq, rankq) = is_subset(hand, can_quads); 
    // println!("quads  {:} rank {:}", hitq, rankq);
    score += rankq; 
    if hitq {
        return (score, 1);
    }
    

    let can_fh = can_have_fullhouse(comcards);
    let (hitfh, rankfh) = is_subset(hand, can_fh); 
    // println!("can_fh  {:} rank {:}", hitfh, rankfh);
    score += rankfh; 
    if hitfh {
        return (score, 2);
    }

    let can_fl = can_have_flush(comcards);
    let (hitfl, rankfl) = is_subset(hand, can_fl); 
    // println!("can_fl  {:} rank {:}", hitfl, rankfl);
    score += rankfl; 
    if hitfl {
        return (score, 3);
    }


    let can_str = can_have_str(comcards);
    let (hitstr, rankstr) = is_subset(hand, can_str); 
    // println!("can_str  {:} rank {:}", hitstr, rankstr);
    score += rankstr; 
    if hitstr {
        return (score, 4);
    }

    let can_trips = can_have_trips(comcards);
    let (hittrips, ranktrips) = is_subset(hand, can_trips); 
    // println!("can_trips  {:} rank {:}", hittrips, ranktrips);
    score += ranktrips; 
    if hittrips {
        return (score, 5);
    }

    let can_2pair = can_have_twopairs(comcards);
    let (hit2p, rank2p) = is_subset(hand, can_2pair); 
    // println!("can_2pair  {:} rank {:}", hit2p, rank2p);
    score += rank2p; 
    if hit2p {
        return (score, 6);
    }


    let can_pair = can_have_pair(comcards);
    let (hitp, rankp) = is_subset(hand, can_pair); 
    // println!("can_pair  {:} rank {:}", hitp, rankp);
    score += rankp; 
    if hitp {
        return (score, 7);
    }

    let can_hc = can_have_highcard(comcards);
    let (hithc, rankhc) = is_subset(hand, can_hc); 
    // println!("can_hc  {:} rank {:}", hithc, rankhc);
    score += rankhc; 

    return (score, 8);
    
    
}
pub fn is_subset(hand: &Vec<Card>, combis: CanHaveCombis) -> (bool, u16) {
    let mut rank = 0;

    for (_board, same_rank_hands) in combis {
        for same_rank in same_rank_hands {
            let (hit, score) = matches_hand(hand, &same_rank);
            // println!("matches hand hit {:?} score {:?} rank {:?}", hit, score, rank);
            rank += score;
            // println!("rank {:?}", rank);
            if hit {
                return (true, rank);
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