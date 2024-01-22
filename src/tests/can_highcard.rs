#[cfg(test)]
mod tests {
    use crate::card::{Card, Suit};
    use crate::can_highcard::can_have_highcard;
    use insta::assert_debug_snapshot;
    #[test]
    fn nopair_board() {
        let cards = vec![
            Card { value: 1, suit: Suit::H },
            Card { value: 8, suit: Suit::D },
            Card { value: 10, suit: Suit::C },
            Card { value: 2, suit: Suit::S },
            Card { value: 3, suit: Suit::S },
        ];
        let combis = can_have_highcard(&cards);
        println!("combis {:?}", combis);
        assert_debug_snapshot!(combis);
    }
}