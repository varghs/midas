#[cfg(test)]
mod tests {
    use midas::engine::{bitboard::*, board::Square};

    #[test]
    fn lsb1_1() {
        let bb_1: Bitboard = 1;
        println!("bb_4: {:?}\nbinary: {:#b}", bb_1, bb_1);
        // Test case with bit at index 0
        assert_eq!(bb_1.pop_lsb(), Some(Square::a1));
    }
    #[test]
    fn lsb1_4() {
        let bb_4: Bitboard = 4;
        println!("bb_4: {:?}\nbinary: {:#b}", bb_4, bb_4);
        assert_eq!(bb_4.pop_lsb(), Some(Square::c1));
    }
    #[test]
    fn lsb1_128() {
        let bb_128: Bitboard = 128;
        println!("bb_128: {:?}\nbinary: {:#b}", bb_128, bb_128);
        // Test case with bit at index 0
        assert_eq!(bb_128.pop_lsb(), Some(Square::h1));
    }
    #[test]
    fn lsb1_0() {
        let bb_0: Bitboard = 0;
        println!("bb_0: {:?}\nbinary: {:#b}", bb_0, bb_0);
        assert_eq!(bb_0.pop_lsb(), None);
    }
}
