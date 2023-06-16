use crate::engine::bitboard::Bitboard;
use crate::engine::square::Square;
use crate::engine::bitboard::{EMPTY, NOTABFILE, NOTAFILE, NOTHFILE, NOTHGFILE, ONE};
use crate::set_bit;

use super::BISHOP_MAGICS;

// bishop relevant occupancy bit count for every square on board
#[rustfmt::skip]
pub const bishop_relevant_bits: [usize; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6,
];

pub struct BishopAttacks {
    pub bishop_masks: [Bitboard; 64], // [square]
    pub bishop_attacks: [[Bitboard; 512]; 64], // [square][occupancy]
}

impl BishopAttacks {
    pub fn new() -> Self {
        let bishop_masks = [0; 64];
        let bishop_attacks = [[0; 512]; 64];
        Self { bishop_masks, bishop_attacks }
    }

    pub fn mask_bishop_attack(square: Square) -> Bitboard {
        let mut result = EMPTY;
        let target_rank = (square as i32) / 8;
        let target_file = (square as i32) % 8;

        let mut rank = target_rank + 1;
        let mut file = target_file + 1;
        while rank <= 6 && file <= 6 {
            set_bit!(result, rank * 8 + file);
            rank += 1;
            file += 1;
        }

        rank = target_rank + 1;
        file = target_file - 1;
        while rank <= 6 && file >= 1 {
            set_bit!(result, rank * 8 + file);
            rank += 1;
            file -= 1;
        }

        rank = target_rank - 1;
        file = target_file + 1;
        while rank >= 1 && file <= 6 {
            set_bit!(result, rank * 8 + file);
            rank -= 1;
            file += 1;
        }

        rank = target_rank - 1;
        file = target_file - 1;
        while rank >= 1 && file >= 1 {
            set_bit!(result, rank * 8 + file);
            rank -= 1;
            file -= 1;
        }

        result
    }

    pub fn bishop_attacks_otf(square: Square, blockers: Bitboard) -> Bitboard {
        let mut result = EMPTY;
        let target_rank = (square as i32) / 8;
        let target_file = (square as i32) % 8;

        let mut rank = target_rank + 1;
        let mut file = target_file + 1;
        while rank <= 7 && file <= 7 {
            set_bit!(result, rank * 8 + file);
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
            set_bit!(result, rank * 8 + file);
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
            set_bit!(result, rank * 8 + file);
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
            set_bit!(result, rank * 8 + file);
            if (blockers & (ONE << (rank * 8 + file))) != 0 {
                break;
            }
            rank -= 1;
            file -= 1;
        }

        result
    }

    pub fn get_bishop_attack(&self, square: Square, occupancy: Bitboard) -> Bitboard {
        let square_num: usize = square as usize;
        let mut occ = occupancy;

        occ &= self.bishop_masks[square_num];
        occ = occ.wrapping_mul(BISHOP_MAGICS[square_num]);
        occ >>= 64 - bishop_relevant_bits[square_num];

        self.bishop_attacks[square_num][occ as usize]
    }
}
