use super::board::Square;
use super::board::Piece;
use super::board::Color;
use super::bitboard::Bitboard;
use super::board::Board;
// 000000, 000000, 0        , 0      , 0        , 0
//  from ,   to  , promotion, capture, special 1, special 0
pub struct Move(u16, Piece, Color); 

impl Move {
    fn get_from(&self) -> Square {
        let s: Square = (self.0 >> 10).try_into().unwrap();
        s
    }

    fn get_to(&self) -> Square {
        let s: Square = ((0x3F0 & self.0) >> 4).try_into().unwrap();
        s
    }

    fn promotion(&self) -> bool {
        0x8 & self.0 > 0
    }

    fn capture(&self) -> bool {
        0x4 & self.0 > 0
    }

    fn special_one(&self) -> bool {
        0x2 & self.0 > 0
    }

    fn special_zero(&self) -> bool {
        0x1 & self.0 > 0
    }

    fn get_piece(&self) -> Piece {
        self.1
    }

    fn get_color(&self) -> Color {
        self.2
    }
}

trait Moves {
    fn quiet_moves(&self, b: &mut Board);
    fn double_pawn_push(&self, b: &mut Board);
    fn king_castle(&self, b: &mut Board);
    fn queen_castle(&self, b: &mut Board);
    fn captures(&self, b: &mut Board);
    fn ep_capture(&self, b: &mut Board);
    fn knight_promotion(&self, b: &mut Board);
    fn bishop_promotion(&self, b: &mut Board);
    fn rook_promotion(&self, b: &mut Board);
    fn queen_promotion(&self, b: &mut Board);
    fn knight_promo_capture(&self, b: &mut Board);
    fn bishop_promo_capture(&self, b: &mut Board);
    fn rook_promo_capture(&self, b: &mut Board);
    fn queen_promo_capture(&self, b: &mut Board);
}

const ONE: Bitboard = 1;

impl Moves for Move {
    fn quiet_moves(&self, b: &mut Board) {
        let from_bb = ONE << (self.get_from() as u64);
        let to_bb = ONE << (self.get_to() as u64);
        let from_to_bb  = from_bb ^ to_bb;
        b.boards[self.get_piece() as usize] ^= from_to_bb;
        b.boards[self.get_color() as usize] ^= from_to_bb;
    }
}
