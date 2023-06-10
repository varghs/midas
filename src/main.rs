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
    print_bitboard(PawnAttacks::mask_pawn_attacks(Square::a7, Color::Black));
}
