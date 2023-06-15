use crate::engine::bitboard::Bitboard;
use crate::engine::square::Square;
use crate::engine::bitboard::{NOTABFILE, NOTAFILE, NOTHFILE, NOTHGFILE, EMPTY, ONE, print_bitboard};
use crate::set_bit;

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

pub struct RookAttacks {
    rook_masks: [Bitboard; 64], // [color][square]
}

impl RookAttacks {
    pub fn new() -> Self {
        let rook_masks = [0; 64];
        Self { rook_masks }
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

    pub fn get_rook_attack(square: Square, blockers: Bitboard) -> Bitboard {
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

    pub fn populate() {
        todo!()
    }
}
