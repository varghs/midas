use super::bitboard::Bitboard;
use super::board::Board;
use super::board::Color;
use super::board::Piece;
use super::board::Square;
// 000000, 000000, 0        , 0      , 0        , 0
//  from ,   to  , promotion, capture, special 1, special 0
pub struct Move(u16, Piece, Color, Option<Piece>, Option<Color>);

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

    fn get_captured_piece(&self) -> Option<Piece> {
        self.3
    }
    fn get_captured_color(&self) -> Option<Color> {
        self.4
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
        let from_to_bb = from_bb ^ to_bb;
        b.boards[self.get_piece() as usize] ^= from_to_bb;
        b.boards[self.get_color() as usize] ^= from_to_bb;
        b.double_pawn_push = false;
    }

    fn double_pawn_push(&self, b: &mut Board) {
        let from_bb = ONE << (self.get_from() as u64);
        let to_bb = ONE << (self.get_to() as u64);
        let from_to_bb = from_bb ^ to_bb;
        b.boards[self.get_piece() as usize] ^= from_to_bb;
        b.boards[self.get_color() as usize] ^= from_to_bb;
        b.double_pawn_push = true;
    }
    fn captures(&self, b: &mut Board) {
        let from_bb = ONE << (self.get_from() as u64);
        let to_bb = ONE << (self.get_to() as u64);
        let from_to_bb = from_bb ^ to_bb;
        b.boards[self.get_piece() as usize] ^= from_to_bb;
        b.boards[self.get_color() as usize] ^= from_to_bb;
        // xor with to_bb to get rid of the captured piece
        b.boards[self.get_captured_piece().unwrap() as usize] ^= to_bb;
        b.boards[self.get_captured_color().unwrap() as usize] ^= to_bb;
        b.double_pawn_push = false;
    }
    fn knight_promotion(&self, b: &mut Board) {
        let from_bb = ONE << (self.get_from() as u64);
        let to_bb = ONE << (self.get_to() as u64);
        let from_to_bb = from_bb ^ to_bb;
        b.boards[Piece::Knight as usize] ^= to_bb;
        b.boards[self.get_piece() as usize] ^= from_bb;
        b.boards[self.get_color() as usize] ^= from_to_bb;
    }
    fn bishop_promotion(&self, b: &mut Board) {
        let from_bb = ONE << (self.get_from() as u64);
        let to_bb = ONE << (self.get_to() as u64);
        let from_to_bb = from_bb ^ to_bb;
        b.boards[Piece::Bishop as usize] ^= to_bb;
        b.boards[self.get_piece() as usize] ^= from_bb;
        b.boards[self.get_color() as usize] ^= from_to_bb;
    }
    fn rook_promotion(&self, b: &mut Board) {
        let from_bb = ONE << (self.get_from() as u64);
        let to_bb = ONE << (self.get_to() as u64);
        let from_to_bb = from_bb ^ to_bb;
        b.boards[Piece::Rook as usize] ^= to_bb;
        b.boards[self.get_piece() as usize] ^= from_bb;
        b.boards[self.get_color() as usize] ^= from_to_bb;
    }
    fn queen_promotion(&self, b: &mut Board) {
        let from_bb = ONE << (self.get_from() as u64);
        let to_bb = ONE << (self.get_to() as u64);
        let from_to_bb = from_bb ^ to_bb;
        b.boards[Piece::Queen as usize] ^= to_bb;
        b.boards[self.get_piece() as usize] ^= from_bb;
        b.boards[self.get_color() as usize] ^= from_to_bb;
    }

    fn king_castle(&self, b: &mut Board) {
        todo!()
    }

    fn queen_castle(&self, b: &mut Board) {
        todo!()
    }

    fn ep_capture(&self, b: &mut Board) {
        todo!()
    }

    fn knight_promo_capture(&self, b: &mut Board) {
        todo!()
    }

    fn bishop_promo_capture(&self, b: &mut Board) {
        todo!()
    }

    fn rook_promo_capture(&self, b: &mut Board) {
        todo!()
    }

    fn queen_promo_capture(&self, b: &mut Board) {
        todo!()
    }
}
