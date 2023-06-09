use crate::{engine::{bitboard::Bitboard, board::{Square, Color}}, set_bit};

const NOTAFILE: Bitboard = 0xfefefefefefefefe;
const NOTHFILE: Bitboard = 0x7f7f7f7f7f7f7f7f;

pub struct PawnAttacks {
    pawn_attacks: [[Bitboard; 64]; 2] // [color][square]
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
                if bitboard & NOTHFILE == 0 {
                    attacks |= bitboard << 9;
                }
            },
            Color::Black => {
            }
        }

        attacks
    }
}
