mod card;
pub use card::{Card, Suit};
mod sorting;
pub use sorting::{get_same_value_map, sort_cards};
mod can_quads;
pub use can_quads::can_have_quads;
mod can_libs;
pub use can_libs::*;
mod can_strflush;
pub use can_strflush::*;

mod tests;