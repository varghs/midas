use super::super::bitboard::Bitboard;
use super::super::square::Square;
use crate::engine::bitboard::{NOTABFILE, NOTAFILE, NOTHFILE, NOTHGFILE, EMPTY, ONE};
use crate::set_bit;

pub struct BishopAttacks {
    bishop_attacks: [Bitboard; 64], // [color][square]
}

impl BishopAttacks {
    pub fn new() -> Self {
        let bishop_attacks = [0; 64];
        Self { bishop_attacks }
    }

    pub fn bishop_attacks_otf(square: Square, block: Bitboard) -> Bitboard {
        let mut attacks: Bitboard = EMPTY;
        let (target_rank, target_file) = ((square as i32) / 8, (square as i32) % 8 );
        let (mut rank, mut file): (i32, i32) = (target_rank + 1, target_file + 1);

        while rank <= 7 && file <= 7 {
            set_bit!(attacks, rank * 8 + file);
            rank += 1;
            file += 1;
        }

        (rank, file) = (target_rank - 1, target_file + 1);

        while rank >= 0 && file <= 7 {
            set_bit!(attacks, rank * 8 + file);
            rank -= 1;
            file += 1;
        }

        (rank, file) = (target_rank + 1, target_file - 1);

        while rank <= 7 && file >= 0 {
            set_bit!(attacks, rank * 8 + file);
            rank += 1;
            file -= 1;
        }

        (rank, file) = (target_rank - 1, target_file - 1);

        while rank >= 0 && file >= 0 {
            set_bit!(attacks, rank * 8 + file);
            rank -= 1;
            file -= 1;
        }

        attacks
    }

    pub fn populate() {
        todo!()
    }
}
