use super::super::bitboard::Bitboard;
use super::super::square::Square;
use crate::engine::bitboard::{NOTABFILE, NOTAFILE, NOTHFILE, NOTHGFILE, EMPTY, ONE, print_bitboard};
use crate::set_bit;

pub struct RookAttacks {
    rook_attacks: [Bitboard; 64], // [color][square]
}

impl RookAttacks {
    pub fn new() -> Self {
        let rook_attacks = [0; 64];
        Self { rook_attacks }
    }

    pub fn mask_rook_attacks(square: Square) -> Bitboard {
        let mut attacks: Bitboard = EMPTY;
        let (target_rank, target_file) = ((square as i32) / 8, (square as i32) % 8);
        let (mut rank, mut file): (i32, i32) = (target_rank + 1, target_file + 1);

        while rank <= 6 {
            set_bit!(attacks, rank * 8 + target_file);
            rank += 1;
        }

        while file <= 6 {
            set_bit!(attacks, target_rank * 8 + file);
            file += 1;
        }

        (rank, file) = (target_rank - 1, target_file - 1);

        while rank >= 1 {
            set_bit!(attacks, rank * 8 + target_file);
            rank -= 1;
        }

        while file >= 1 {
            set_bit!(attacks, target_rank * 8 + file);
            file -= 1;
        }

        attacks
    }

    pub fn populate() {
        todo!()
    }
}
