use crate::engine::bitboard::Bitboard;
use crate::engine::bitboard::{
    print_bitboard, EMPTY, NOTABFILE, NOTAFILE, NOTHFILE, NOTHGFILE, ONE,
};
use crate::engine::square::Square;
use crate::set_bit;

use super::ROOK_MAGICS;

// rook relevant occupancy bit count for every square on board
#[rustfmt::skip]
pub const rook_relevant_bits: [usize; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 
    12, 11, 11, 11, 11, 11, 11, 12,
];

#[derive(Clone)]
pub struct RookAttacks {
    pub rook_masks: [Bitboard; 64],           // [square]
    pub rook_attacks: [[Bitboard; 4096]; 64], // [square][occupancy]
}

impl RookAttacks {
    pub fn new() -> Self {
        let rook_masks = [0; 64];
        let rook_attacks = [[0; 4096]; 64];
        Self {
            rook_masks,
            rook_attacks,
        }
    }

    pub fn mask_rook_attack(square: Square) -> Bitboard {
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

    pub fn rook_attacks_otf(square: Square, blockers: Bitboard) -> Bitboard {
        let mut attacks: Bitboard = EMPTY;
        let (target_rank, target_file) = ((square as i32) / 8, (square as i32) % 8);
        let (mut rank, mut file): (i32, i32) = (target_rank + 1, target_file + 1);

        while rank <= 7 {
            set_bit!(attacks, rank * 8 + target_file);
            if (blockers & (ONE << (rank * 8 + target_file))) != 0 {
                break;
            }
            rank += 1;
        }

        while file <= 7 {
            set_bit!(attacks, target_rank * 8 + file);
            if (blockers & (ONE << (target_rank * 8 + file))) != 0 {
                break;
            }
            file += 1;
        }

        (rank, file) = (target_rank - 1, target_file - 1);

        while rank >= 0 {
            set_bit!(attacks, rank * 8 + target_file);
            if (blockers & (ONE << (rank * 8 + target_file))) != 0 {
                break;
            }
            rank -= 1;
        }

        while file >= 0 {
            set_bit!(attacks, target_rank * 8 + file);
            if (blockers & (ONE << (target_rank * 8 + file))) != 0 {
                break;
            }
            file -= 1;
        }

        attacks
    }

    pub fn get_rook_attack(&self, square: Square, occupancy: Bitboard) -> Bitboard {
        let square_num: usize = square as usize;
        let mut occ = occupancy;

        occ &= self.rook_masks[square_num];
        occ = occ.wrapping_mul(ROOK_MAGICS[square_num]);
        occ >>= 64 - rook_relevant_bits[square_num];

        self.rook_attacks[square_num][occ as usize]
    }
}
