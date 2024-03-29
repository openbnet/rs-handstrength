// #![feature(test)]
// extern crate test;
mod card;
pub use card::{Card, Suit};
mod sorting;
pub use sorting::*;
mod can_quads;
pub use can_quads::can_have_quads;
mod can_libs;
pub use can_libs::*;
mod can_strflush;
pub use can_strflush::*;
mod can_fullhouse;
pub use can_fullhouse::*;
mod can_flush;
pub use can_flush::*;
mod can_str;
pub use can_str::*;
mod can_trips;
pub use can_trips::*;
mod can_twopair;
pub use can_twopair::*;
mod can_pair;
pub use can_pair::*;
mod can_highcard;
pub use can_highcard::*;
mod get_nut_rank;
pub use get_nut_rank::*;
mod equity;
pub use equity::*;
mod get_winner;
pub use get_winner::*;
mod pf_rank;
pub use pf_rank::*; 
mod tests;