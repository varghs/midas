use super::super::bitboard::Bitboard;
use super::super::square::Square;
use crate::engine::bitboard::{NOTABFILE, NOTAFILE, NOTHFILE, NOTHGFILE, EMPTY};
use crate::set_bit;
pub struct KnightAttacks {
    knight_attacks: [Bitboard; 64], // [color][square]
}

impl KnightAttacks {
    pub fn new() -> Self {
        let knight_attacks = [0; 64];
        Self { knight_attacks }
    }

    pub fn mask_knight_attacks(square: Square) -> Bitboard {
        let mut attacks: Bitboard = EMPTY;
        let mut bitboard: Bitboard = EMPTY;

        set_bit!(bitboard, square);

        // shl 15 for top left, shl 17 for top right, shl 6 for left top, shl 10 for right top
        if (bitboard << 15) & NOTHFILE > 0 {
            attacks |= bitboard << 15;
        }

        if (bitboard << 17) & NOTAFILE > 0 {
            attacks |= bitboard << 17;
        }

        if (bitboard << 6) & NOTHGFILE > 0 {
            attacks |= bitboard << 6;
        }

        if (bitboard << 10) & NOTABFILE > 0 {
            attacks |= bitboard << 10;
        }

        if (bitboard >> 15) & NOTAFILE > 0 {
            attacks |= bitboard >> 15;
        }

        if (bitboard >> 17) & NOTHFILE > 0 {
            attacks |= bitboard >> 17;
        }

        if (bitboard >> 6) & NOTABFILE > 0 {
            attacks |= bitboard >> 6;
        }

        if (bitboard >> 10) & NOTHGFILE > 0 {
            attacks |= bitboard >> 10;
        }
        attacks
    }

    pub fn populate(&mut self) {
        for square in 0..64 {
            self.knight_attacks[square] = Self::mask_knight_attacks(square.try_into().unwrap());
        }
    }
}
