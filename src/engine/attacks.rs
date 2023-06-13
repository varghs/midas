use self::bishop_attacks::BishopAttacks;
use self::king_attacks::KingAttacks;
use self::knight_attacks::KnightAttacks;
use self::pawn_attacks::PawnAttacks;

use super::bitboard::{Bitboard, EMPTY};
use super::board::Piece;
use super::square::Square;

pub mod bishop_attacks;
pub mod king_attacks;
pub mod knight_attacks;
pub mod pawn_attacks;

struct AttackTables {
    pawns: PawnAttacks,
    knights: KnightAttacks,
    kings: KingAttacks,
    bishops: BishopAttacks,
}

impl AttackTables {
    // for every attack_table calls populate method
    fn populate(&mut self) {
        todo!();
    }

    fn populate_leapers_attacks(&mut self) {
        self.pawns.populate();
        self.knights.populate();
        self.kings.populate();
    }
}

fn set_occupancy(index: usize, bits_in_mask: usize, attack_mask: Bitboard) -> Bitboard {}

fn find_magic_number(square: Square, relevant_bits: u32, piece_type: Piece) -> Bitboard {
    //
    let occupancies: [Bitboard; 4096];
    let attacks: [Bitboard; 4096];
    let used_attacks: [Bitboard; 4096];
    let attack_mask: Bitboard = match piece_type {
        Piece::Bishop => BishopAttacks::mask_bishop_attacks(square),
        // Piece::Rook => {
        //     RookAttacks::mask_rook_attacks(square)
        // }
        _ => panic!("bro its gotta be rook or bishop LOLS"),
    };
    let occupancy_indicies = 1 << relevant_bits;

    for i in 0..occupancy_indicies {
        // todo!
        // call @set_occupancy
    }
    todo!();
}
