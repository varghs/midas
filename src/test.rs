#[cfg(test)]
mod tests {
    use std::time::Instant;

    use midas::{engine::{
        bitboard::*,
        board::{Board, Square, Color},
        attacks::pawn::PawnAttacks,
    }, set_bit};

    #[test]
    fn lsb1_1() {
        let bb_1: Bitboard = 1;
        println!("bb_4: {:?}\nbinary: {:#b}", bb_1, bb_1);
        // Test case with bit at index 0
        assert_eq!(bb_1.index_of_lsb(), Some(Square::a1));
    }
    #[test]
    fn lsb1_4() {
        let bb_4: Bitboard = 4;
        println!("bb_4: {:?}\nbinary: {:#b}", bb_4, bb_4);
        assert_eq!(bb_4.index_of_lsb(), Some(Square::c1));
    }
    #[test]
    fn lsb1_128() {
        let bb_128: Bitboard = 128;
        println!("bb_128: {:?}\nbinary: {:#b}", bb_128, bb_128);
        // Test case with bit at index 0
        assert_eq!(bb_128.index_of_lsb(), Some(Square::h1));
    }
    #[test]
    fn lsb1_0() {
        let bb_0: Bitboard = 0;
        println!("bb_0: {:?}\nbinary: {:#b}", bb_0, bb_0);
        assert_eq!(bb_0.index_of_lsb(), None);
    }
    #[test]
    fn stress_test() {
        let pawns: Bitboard = 0x00FF00000000FF00;
        let rooks: Bitboard = 0x8100000000000081;
        let knights: Bitboard = 0x4200000000000042;
        let bishops: Bitboard = 0x2400000000000024;
        let queens: Bitboard = 0x0800000000000008;
        let kings: Bitboard = 0x1000000000000010;
        let black: Bitboard = 0xFFFF000000000000;
        let white: Bitboard = 0x000000000000FFFF;

        let boards: [Bitboard; 8] = [black, white, pawns, rooks, knights, bishops, queens, kings];

        let start_time = Instant::now();

        use rand::Rng;
        let mut rng = rand::thread_rng();
        for mut board in boards {
            let mut popped = board.pop_lsb();
            while popped.is_some() {
                popped = board.pop_lsb();
            }
            println!("this is board {:#066b}", board);
        }
        let end_time = Instant::now();
        println!("Elapsed time {:?}", end_time - start_time);
    }

    #[test]
    fn pawn_attacks_reg_white() {
        let mut bitboard: Bitboard = EMPTY;
        set_bit!(bitboard, Square::e4);
        set_bit!(bitboard, Square::c4);
        assert_eq!(PawnAttacks::mask_pawn_attacks(Square::d3, Color::White), bitboard);
    }

    #[test]
    fn pawn_attacks_reg_black() {
        let mut bitboard: Bitboard = EMPTY;
        set_bit!(bitboard, Square::e5);
        set_bit!(bitboard, Square::c5);
        assert_eq!(PawnAttacks::mask_pawn_attacks(Square::d6, Color::Black), bitboard);
    }

    #[test]
    fn pawn_attacks_h_white() {
        let mut bitboard: Bitboard = EMPTY;
        set_bit!(bitboard, Square::g7);
        assert_eq!(PawnAttacks::mask_pawn_attacks(Square::h6, Color::White), bitboard);
    }

    #[test]
    fn pawn_attacks_h_black() {
        let mut bitboard: Bitboard = EMPTY;
        set_bit!(bitboard, Square::g4);
        assert_eq!(PawnAttacks::mask_pawn_attacks(Square::h5, Color::Black), bitboard);
    }

    #[test]
    fn pawn_attacks_a_waite() {
        let mut bitboard: Bitboard = EMPTY;
        set_bit!(bitboard, Square::b4);
        assert_eq!(PawnAttacks::mask_pawn_attacks(Square::a3, Color::White), bitboard);
    }

    #[test]
    fn pawn_attacks_a_black() {
        let mut bitboard: Bitboard = EMPTY;
        set_bit!(bitboard, Square::b6);
        assert_eq!(PawnAttacks::mask_pawn_attacks(Square::a7, Color::Black), bitboard);
    }

    #[test]
    fn pawn_attacks_fail() {
        let mut bitboard: Bitboard = EMPTY;
        set_bit!(bitboard, Square::e4);
        set_bit!(bitboard, Square::c4);
        assert_ne!(PawnAttacks::mask_pawn_attacks(Square::d6, Color::Black), bitboard);
    }
}
