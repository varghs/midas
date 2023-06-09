use super::super::bitboard::Bitboard;
use super::super::square::Square;
use crate::engine::bitboard::{EMPTY, NOTABFILE, NOTAFILE, NOTHFILE, NOTHGFILE};
use crate::set_bit;

#[derive(Clone)]
pub struct KingAttacks {
    pub king_attacks: [Bitboard; 64], // [color][square]
}

impl KingAttacks {
    pub fn new() -> Self {
        let king_attacks = [0; 64];
        Self { king_attacks }
    }

    pub fn mask_king_attacks(square: Square) -> Bitboard {
        let mut attacks: Bitboard = EMPTY;
        let mut bitboard: Bitboard = EMPTY;
        set_bit!(bitboard, square);

        if bitboard & NOTHFILE > 0 {
            attacks |= bitboard << 1;
            attacks |= bitboard << 9;
            attacks |= bitboard >> 7;
        }

        if bitboard & NOTAFILE > 0 {
            attacks |= bitboard << 7;
            attacks |= bitboard >> 1;
            attacks |= bitboard >> 9;
        }

        attacks |= bitboard << 8;
        attacks |= bitboard >> 8;

        attacks
    }

    pub fn populate(&mut self) {
        for square in 0..64 {
            self.king_attacks[square] = Self::mask_king_attacks(square.try_into().unwrap());
        }
    }
}
