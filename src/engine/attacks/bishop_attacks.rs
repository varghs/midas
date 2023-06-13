use super::super::bitboard::Bitboard;
use super::super::square::Square;
use crate::engine::bitboard::{EMPTY, NOTABFILE, NOTAFILE, NOTHFILE, NOTHGFILE, ONE};
use crate::set_bit;

pub struct BishopAttacks {
    bishop_attacks: [Bitboard; 64], // [color][square]
}

impl BishopAttacks {
    pub fn new() -> Self {
        todo!()
    }

    pub fn mask_bishop_attacks(square: Square) -> Bitboard {
        let mut attacks: Bitboard = EMPTY;
        let (target_rank, target_file) = ((square as i32) / 8, (square as i32) % 8);
        let mut rank = target_rank + 1;
        let mut file = target_file + 1;

        // while loop instead of for loop cuz... C is too complicated LOL
        while rank <= 6 && file <= 6 {
            attacks |= ONE << (rank * 8 + file);
            rank += 1;
            file += 1;
        }

        attacks
    }

    pub fn populate() {
        todo!()
    }
}
