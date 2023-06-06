#[cfg(test)]
mod tests {
    use midas::engine::bitboard::*;

    #[test]
    fn test_least_significant_one_bit() {
        let bb_1: Bitboard = 1;
        let bb_4: Bitboard = 4;
        let bb_128: Bitboard = 128;
        let bb_0: Bitboard = 0;
        // Test case with bit at index 0
        assert_eq(bb_1.pop_lsb(), Some(Square::a1));
        assert_eq(bb_4.pop_lsb(), Some(Square::a3));
        assert_eq(bb_128.pop_lsb(), Some(Square::a8));
        assert_eq(bb_0.pop_lsb(), None);
    }
}
