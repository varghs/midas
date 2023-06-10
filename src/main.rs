mod test;

use midas::{
    engine::attacks::pawn::PawnAttacks,
    engine::{
        bitboard::{print_bitboard, Bitboard},
        board::{Board, Color, Square},
    },
    set_bit,
};

fn main() {
    /*
    let pawns: Bitboard = 0x00FF00000000FF00;
    let rooks: Bitboard = 0x8100000000000081;
    let knights: Bitboard = 0x4200000000000042;
    let bishops: Bitboard = 0x2400000000000024;
    let queens: Bitboard = 0x0800000000000008;
    let kings: Bitboard = 0x1000000000000010;
    let black: Bitboard = 0xFFFF000000000000;
    let white: Bitboard = 0x000000000000FFFF;

    let boards: [Bitboard; 8] = [black, white, pawns, rooks, knights, bishops, queens, kings];
    let board: Board = Board {
        boards,
        double_pawn_push: false,
        king_side_castle: false,
        queen_side_castle: false,
        en_pessant: false,
    };

    print!("{}", board);
    */

    print_bitboard(PawnAttacks::mask_pawn_attacks(Square::a7, Color::Black));
}
