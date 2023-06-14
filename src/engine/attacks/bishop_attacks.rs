use super::super::bitboard::Bitboard;
use super::super::square::Square;
use crate::engine::bitboard::{EMPTY, NOTABFILE, NOTAFILE, NOTHFILE, NOTHGFILE, ONE};
use crate::set_bit;

pub struct BishopAttacks {
    bishop_attacks: [Bitboard; 64], // [color][square]
}

impl BishopAttacks {
    pub fn new() -> Self {
        let bishop_attacks = [0; 64];
        Self { bishop_attacks }
    }

    pub fn get_bishop_attack(square: Square, blockers: Bitboard) -> Bitboard {
        let mut result = EMPTY;
        let target_rank = (square as i32) / 8;
        let target_file = (square as i32) % 8;

        let mut rank = target_rank + 1;
        let mut file = target_file + 1;
        while rank <= 7 && file <= 7 {
            result |= ONE << (rank * 8 + file);
            // reached something that blocks us
            if (blockers & (ONE << (rank * 8 + file))) != 0 {
                break;
            }
            rank += 1;
            file += 1;
        }
        // ========
        rank = target_rank + 1;
        file = target_file - 1;
        while rank <= 7 && file >= 0 {
            result |= ONE << (rank * 8 + file);
            if (blockers & (ONE << (rank * 8 + file))) != 0 {
                break;
            }
            rank += 1;
            file -= 1;
        }
        // ========
        rank = target_rank - 1;
        file = target_file + 1;
        while rank >= 0 && file <= 7 {
            result |= ONE << (rank * 8 + file);
            if (blockers & (ONE << (rank * 8 + file))) != 0 {
                break;
            }
            rank -= 1;
            file += 1;
        }
        // ========
        rank = target_rank - 1;
        file = target_file - 1;
        while rank >= 0 && file >= 0 {
            result |= ONE << (rank * 8 + file);
            if (blockers & (ONE << (rank * 8 + file))) != 0 {
                break;
            }
            rank -= 1;
            file -= 1;
        }

        result
    }

    pub fn populate() {
        todo!()
    }
}
