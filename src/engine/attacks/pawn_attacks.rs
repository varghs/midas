use crate::{
    engine::{
        bitboard::{Bitboard, NOTAFILE, NOTHFILE},
        board::Color,
        square::Square,
    },
    set_bit,
};

pub struct PawnAttacks {
    pub pawn_attacks: [[Bitboard; 64]; 2], // [color][square]
}

impl PawnAttacks {
    pub fn new() -> Self {
        let pawn_attacks = [[0; 64]; 2];
        Self { pawn_attacks }
    }

    pub fn mask_pawn_attacks(square: Square, color: Color) -> Bitboard {
        let mut attacks: Bitboard = 0;
        let mut bitboard: Bitboard = 0;

        set_bit!(bitboard, square);

        match color {
            Color::White => {
                if bitboard & NOTHFILE != 0 {
                    attacks |= bitboard << 9;
                }
                if bitboard & NOTAFILE != 0 {
                    attacks |= bitboard << 7;
                }
            }
            Color::Black => {
                if bitboard & NOTHFILE != 0 {
                    attacks |= bitboard >> 7;
                }
                if bitboard & NOTAFILE != 0 {
                    attacks |= bitboard >> 9;
                }
            }
        }

        attacks
    }

    pub fn populate(&mut self) {
        for square in 0..64 {
            self.pawn_attacks[Color::White as usize][square] =
                Self::mask_pawn_attacks(square.try_into().unwrap(), Color::White);
            self.pawn_attacks[Color::Black as usize][square] =
                Self::mask_pawn_attacks(square.try_into().unwrap(), Color::Black);
        }
    }
}
