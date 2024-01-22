#[cfg(test)]
mod tests {
    use crate::card::{Card, Suit};
    use crate::can_strflush::*;
    use crate::can_libs::*;
    use crate::get_nut_rank::*;
    use insta::assert_debug_snapshot;
    #[test]
    fn no_strflush() {
        let comcards = vec![
            Card { value: 2, suit: Suit::S },
            Card { value: 3, suit: Suit::H },
            Card { value: 4, suit: Suit::D },
            Card { value: 5, suit: Suit::C }
        ];
        assert!(can_have_straight_flush(&comcards).is_empty());
    }
    #[test]
    fn one_strflush() {
        let comcards = vec![
            Card { value: 3, suit: Suit::S },
            Card { value: 2, suit: Suit::S },
            Card { value: 4, suit: Suit::S },
            Card {value: 1, suit: Suit::S }
        ]; 
        let strflush = can_have_straight_flush(&comcards);
        println!("strflush {:?}", strflush);
        assert_eq!(strflush.len(), 1);
        let can_have = can_have_straight_flush(&comcards);
        println!("can_have {:?}", can_have);
        assert_debug_snapshot!(can_have); 
        let hand = vec![
            Card { value: 6, suit: Suit::S },
            Card { value: 5, suit: Suit::S },
            Card { value: 7, suit: Suit::S },
            Card {value: 8, suit: Suit::S } 
        ];
        let nr = get_nut_rank(&hand, &comcards);
        println!("nr {:?}", nr);
        assert_debug_snapshot!(nr);
    }
    #[test]
    fn cal_gaps1() {
        let comcards = [
            Card { value: 4, suit: Suit::S },
            Card { value: 2, suit: Suit::S },
            Card { value: 1, suit: Suit::S },

        ]; 
        let v = vec![
            Card { value: 4, suit: Suit::S },
            Card { value: 2, suit: Suit::S },
            Card { value: 1, suit: Suit::S }, 
        ];

        let gaps = calculate_gaps(&comcards);
        // println!("gaps, {:?}", gaps1);
        assert_eq!(gaps.0, 1);
        assert_eq!(gaps.1, vec![vec![3]]);
        let blocked = is_blocked(&comcards, &v, gaps);
        println!("blocked 1 {:?}", blocked);
        assert_debug_snapshot!(blocked);
        let can_have = can_have_straight_flush(&v);
        println!("can_have {:?}", can_have);
        assert_debug_snapshot!(can_have);
        let hand = vec![
            Card { value: 9, suit: Suit::S },
            Card { value: 5, suit: Suit::S },
            Card { value: 3, suit: Suit::S },
            Card {value: 10, suit: Suit::S } 
        ];
        let nr = get_nut_rank(&hand, &v);
        println!("nr {:?}", nr);
        assert_debug_snapshot!(nr); 
    }
    // #[test]
    fn strflush1() {
        let comcards = vec![
            Card { value: 4, suit: Suit::S },
            Card { value: 2, suit: Suit::S },
            Card { value: 1, suit: Suit::S },
        ]; 
        let can_have = can_have_straight_flush(&comcards);
        println!("can_have {:?}", can_have);
        assert_debug_snapshot!(can_have);
        
    }
    #[test]
    fn strflush2() {
        let comcards = vec![
            Card { value: 4, suit: Suit::S },
            Card { value: 2, suit: Suit::S },
            Card { value: 3, suit: Suit::S },
        ]; 
        let can_have = can_have_straight_flush(&comcards);
        println!("can_have {:?}", can_have);
        assert_debug_snapshot!(can_have);
        let hand = vec![
            Card { value: 6, suit: Suit::S },
            Card { value: 5, suit: Suit::S },
            Card { value: 9, suit: Suit::C },
            Card {value: 10, suit: Suit::C } 
        ];
        let nr = get_nut_rank(&hand, &comcards);
        println!("nr {:?}", nr);
        assert_eq!(nr.0, 1);
        let hand = vec![
            Card { value: 1, suit: Suit::S },
            Card { value: 5, suit: Suit::S },
            Card { value: 9, suit: Suit::C },
            Card {value: 10, suit: Suit::C } 
        ];
        let nr2 = get_nut_rank(&hand, &comcards);
        println!("nr2 {:?}", nr2);
        assert_eq!(nr2.0, 0);
    }
    #[test]
    fn cal_gaps21() {
        let comcards = [
            Card { value: 5, suit: Suit::S },
            Card { value: 2, suit: Suit::S },
            Card { value: 1, suit: Suit::S },

        ]; 
        let v = vec![
            Card { value: 5, suit: Suit::S },
            Card { value: 2, suit: Suit::S },
            Card { value: 1, suit: Suit::S }, 
        ];
        let gaps = calculate_gaps(&comcards);
        println!("gaps, {:?}", gaps);
        assert_eq!(gaps.0, 2);
        assert_eq!(gaps.1, vec![vec![4, 3]]);
        let blocked = is_blocked(&comcards, &v, gaps);
        println!("blocked 21 {:?}", blocked);
        assert_debug_snapshot!(blocked);
        let can_have = can_have_straight_flush(&v);
        println!("can_have {:?}", can_have);
        assert_debug_snapshot!(can_have);
    }
    #[test]
    fn cal_gaps12() {
        let comcards = [
            Card { value: 5, suit: Suit::S },
            Card { value: 3, suit: Suit::S },
            Card { value: 1, suit: Suit::S },

        ]; 
        let v = vec![
            Card { value: 5, suit: Suit::S },
            Card { value: 3, suit: Suit::S },
            Card { value: 1, suit: Suit::S },

        ]; 
        let gaps = calculate_gaps(&comcards);
        println!("gaps, {:?}", gaps);
        assert_eq!(gaps.0, 2);
        assert_eq!(gaps.1, vec![vec![4], vec![2]]);
        let blocked = is_blocked(&comcards, &v, gaps);
        println!("blocked {:?}", blocked);
        assert_debug_snapshot!(blocked);
        let can_have = can_have_straight_flush(&v);
        println!("can_have {:?}", can_have);
        assert_debug_snapshot!(can_have); 
    }
    #[test]
    fn cal_gaps31() {
        let comcards = [
            Card { value: 9, suit: Suit::S },
            Card { value: 5, suit: Suit::S },
            Card { value: 4, suit: Suit::S },
        ]; 
        let v = vec![
            Card { value: 9, suit: Suit::S },
            Card { value: 5, suit: Suit::S },
            Card { value: 4, suit: Suit::S },
        ];
        let gaps = calculate_gaps(&comcards);
        println!("gaps, {:?}", gaps);
        assert_eq!(gaps.0, 3);
        assert_eq!(gaps.1, vec![vec![8, 7, 6]]);
        let can_have = can_have_straight_flush(&v);
        println!("can_have {:?}", can_have);
        assert_eq!(can_have.len(), 0);
    }
}