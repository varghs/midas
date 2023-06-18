use std::fmt::Display;

use crate::engine::bitboard::print_bitboard;
use crate::get_bit;
use crate::pop_bit;
use crate::set_bit;

use super::bitboard::Bitboard;
use super::board::Board;
use super::board::BoardState;
use super::board::Color;
use super::board::Piece;
use super::square::Square;

#[derive(PartialEq)]
pub enum MoveType {
    AllMoves,
    OnlyCaptures,
}

// u16: 0000 0000 0011 1111 - source 
//      0000 1111 1100 0000 - target
//      0001 0000 0000 0000 - capture
//      0010 0000 0000 0000 - double push
//      0100 0000 0000 0000 - en-passant
//      1000 0000 0000 0000 - castling
//
//  Piece: piece moved
//  Color: color moved
//  Option<Piece>: piece promoted 
//  Option<Color>: color promoted 
#[derive(Clone, Copy)]
pub struct Move(u16, Piece, Color, Option<Piece>, Option<Color>);

impl Move {
    pub fn new(source: Square, target: Square, moved_piece: Piece, moved_color: Color, promoted_piece: Option<Piece>, promoted_color: Option<Color>, capture: bool, double_push: bool, en_passant: bool, castling: bool) -> Self {
        let num = (source as u16) | ((target as u16) << 6) | ((capture as u16) << 12) | ((double_push as u16) << 13) | ((en_passant as u16) << 14) | ((castling as u16) << 15); 
        Move(num, moved_piece, moved_color, promoted_piece, promoted_color)
    }

    pub fn default() -> Self {
        Move(0, Piece::Pawn, Color::White, None, None)
    }

    pub fn get_source(&self) -> Square {
        let s = (self.0 & 0x003F).try_into().unwrap();
        s
    }

    pub fn get_target(&self) -> Square {
        let s = ((self.0 & 0x0FC0) >> 6).try_into().unwrap();
        s
    }

    pub fn capture(&self) -> bool {
        self.0 & 0x1000 > 0
    }

    pub fn double_push(&self) -> bool {
        self.0 & 0x2000 > 0
    }

    pub fn en_passant(&self) -> bool {
        self.0 & 0x4000 > 0
    }

    pub fn castling(&self) -> bool {
        self.0 & 0x8000 > 0
    }

    pub fn get_piece(&self) -> Piece {
        self.1
    }

    pub fn get_color(&self) -> Color {
        self.2
    }

    pub fn get_promoted_piece(&self) -> Option<Piece> {
        self.3
    }
    pub fn get_promoted_color(&self) -> Option<Color> {
        self.4
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = format!("{}{}{}", self.get_source(), self.get_target(), match self.get_promoted_piece() {
            Some(i) => i.to_string().to_lowercase(),
            None => String::new(),
        });
        write!(f, "{}", output)
    }
}

pub struct MoveList {
    pub moves: [Move; 256],
    pub count: usize,
}

impl MoveList {
    pub fn new() -> Self {
        let moves = [Move::default(); 256];
        let count = 0;
        Self { moves, count }
    }

    pub fn add_move(&mut self, m: Move) {
        self.moves[self.count] = m;
        self.count += 1;
    }
}

impl Display for MoveList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.count == 0 {
            return write!(f, "No moves in move list.");
        }
        let mut output = String::from("source, target, moved_piece, moved_color, promoted_piece, promoted_color, capture, double_push, en_passant, castling\n");
        for move_count in 0..self.count {
            let m = &self.moves[move_count];
            output += format!("{:8}{:8}{:13}{:13}{:16}{:16}{:9}{:13}{:12}{:8}", m.get_source().to_string(), m.get_target().to_string(), m.get_piece().to_string(), m.get_color().to_string(), match m.get_promoted_piece() {
                Some(i) => i.to_string(),
                None => "None".to_string()
            }, match m.get_promoted_color() {
                    Some(i) => i.to_string(),
                    None => "None".to_string(),
                }, m.capture().to_string(), m.double_push().to_string(), m.en_passant().to_string(), m.castling().to_string()).as_str();
            output += "\n";
        }

        output += format!("Total moves: {}\n", self.count).as_str();

        write!(f, "{}", output)
    }
}

impl BoardState {
    pub fn make_move(&mut self, m: Move, move_flag: MoveType) {
        // quiet move
        if move_flag == MoveType::AllMoves {
            self.preserve();
            let source = m.get_source();
            let target = m.get_target();
            let piece = m.get_piece();
            let color = m.get_color();
            let promoted_piece = m.get_promoted_piece();
            let promoted_color = m.get_promoted_color();
            let capture = m.capture();
            let double_pawn_push = m.double_push();
            let en_passant = m.en_passant();
            let castling = m.castling();
            pop_bit!(self.board.boards[piece as usize], source);
            pop_bit!(self.board.boards[color as usize], source);

            // handle captures
            if capture {
                for p in (Piece::Pawn as usize)..=(Piece::King as usize) {
                    if get_bit!(self.board.get_piece_of_color(p.try_into().unwrap(), !self.board.side), target) {
                        pop_bit!(self.board.boards[p], target);
                        pop_bit!(self.board.boards[(!self.board.side) as usize], target);
                        break;
                    }
                }
            }

            set_bit!(self.board.boards[piece as usize], target);
            set_bit!(self.board.boards[color as usize], target);

            // handle pawn promotion
            if promoted_piece.is_some() {
                pop_bit!(self.board.boards[Piece::Pawn as usize], target);
                pop_bit!(self.board.boards[color as usize], target);

                set_bit!(self.board.boards[promoted_piece.unwrap() as usize], target);
                set_bit!(self.board.boards[promoted_color.unwrap() as usize], target);
            }

            // handle enpassant captures
            if en_passant {
                if color == Color::White {
                    pop_bit!(self.board.boards[Piece::Pawn as usize], (target as i32) - 8);
                    pop_bit!(self.board.boards[Color::Black as usize], (target as i32) - 8)
                } else {
                    pop_bit!(self.board.boards[Piece::Pawn as usize], (target as i32) + 8);
                    pop_bit!(self.board.boards[Color::White as usize], (target as i32) + 8)
                }
            }

            self.board.en_passant_sq = None;

            if double_pawn_push {
                match color {
                    Color::White => self.board.en_passant_sq = Some((target as u64 - 8).try_into().unwrap()),
                    Color::Black => self.board.en_passant_sq = Some((target as u64 + 8).try_into().unwrap()),
                }
            }

            // handle castles
            if castling {
                match target {
                    // white kingside
                    Square::g1 => {
                        pop_bit!(self.board.boards[Piece::Rook as usize], Square::h1);
                        pop_bit!(self.board.boards[Color::White as usize], Square::h1);
                        set_bit!(self.board.boards[Piece::Rook as usize], Square::f1);
                        set_bit!(self.board.boards[Color::White as usize], Square::f1);
                    },
                    // white queenside
                    Square::c1 => {
                        pop_bit!(self.board.boards[Piece::Rook as usize], Square::a1);
                        pop_bit!(self.board.boards[Color::White as usize], Square::a1);
                        set_bit!(self.board.boards[Piece::Rook as usize], Square::d1);
                        set_bit!(self.board.boards[Color::White as usize], Square::d1);
                    },
                    // black kingside
                    Square::g8 => {
                        pop_bit!(self.board.boards[Piece::Rook as usize], Square::h8);
                        pop_bit!(self.board.boards[Color::Black as usize], Square::h8);
                        set_bit!(self.board.boards[Piece::Rook as usize], Square::f8);
                        set_bit!(self.board.boards[Color::Black as usize], Square::f8);
                    },
                    // black queenside
                    Square::c8 => {
                        pop_bit!(self.board.boards[Piece::Rook as usize], Square::a8);
                        pop_bit!(self.board.boards[Color::Black as usize], Square::a8);
                        set_bit!(self.board.boards[Piece::Rook as usize], Square::d8);
                        set_bit!(self.board.boards[Color::Black as usize], Square::d8);
                    },
                    _ => panic!("I don't know how it got here, and I don't wanna know how it got here.")
                }
            }
        } else { // capture
            // make sure move is capture
            if m.capture() {
                self.make_move(m, MoveType::AllMoves);
            } else { // don't make move
                return;
            }
        }
    }
}

pub trait Moves {
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

/*
impl Moves for Move {
    fn quiet_moves(&self, b: &mut Board) {
        let from_bb = ONE << (self.get_from() as u64);
        let to_bb = ONE << (self.get_to() as u64);
        let from_to_bb = from_bb ^ to_bb;
        b.boards[self.get_piece() as usize] ^= from_to_bb;
        b.boards[self.get_color() as usize] ^= from_to_bb;
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
        // let from_bb = ONE << (self.get_from() as u64);
        // let to_bb = ONE << (self.get_to() as u64);
        // let from_to_bb = from_bb ^ to_bb;
        // // king moves
        // b.boards[self.get_piece() as usize] ^= from_to_bb;
        // b.boards[self.get_color() as usize] ^= from_to_bb;
        // // rook moves
    }

    fn queen_castle(&self, b: &mut Board) {
        todo!()
    }

    fn ep_capture(&self, b: &mut Board) {
        todo!()
    }

    fn knight_promo_capture(&self, b: &mut Board) {
        let from_bb = ONE << (self.get_from() as u64);
        let to_bb = ONE << (self.get_to() as u64);
        let from_to_bb = from_bb ^ to_bb;
        b.boards[Piece::Knight as usize] ^= to_bb;
        b.boards[self.get_piece() as usize] ^= from_bb;
        b.boards[self.get_color() as usize] ^= from_to_bb;
        // captured piece
        b.boards[self.get_captured_piece().unwrap() as usize] ^= to_bb;
        b.boards[self.get_captured_color().unwrap() as usize] ^= to_bb;
    }

    fn bishop_promo_capture(&self, b: &mut Board) {
        let from_bb = ONE << (self.get_from() as u64);
        let to_bb = ONE << (self.get_to() as u64);
        let from_to_bb = from_bb ^ to_bb;
        b.boards[Piece::Bishop as usize] ^= to_bb;
        b.boards[self.get_piece() as usize] ^= from_bb;
        b.boards[self.get_color() as usize] ^= from_to_bb;
        // captured piece
        b.boards[self.get_captured_piece().unwrap() as usize] ^= to_bb;
        b.boards[self.get_captured_color().unwrap() as usize] ^= to_bb;
    }

    fn rook_promo_capture(&self, b: &mut Board) {
        let from_bb = ONE << (self.get_from() as u64);
        let to_bb = ONE << (self.get_to() as u64);
        let from_to_bb = from_bb ^ to_bb;
        b.boards[Piece::Rook as usize] ^= to_bb;
        b.boards[self.get_piece() as usize] ^= from_bb;
        b.boards[self.get_color() as usize] ^= from_to_bb;
        // captured piece
        b.boards[self.get_captured_piece().unwrap() as usize] ^= to_bb;
        b.boards[self.get_captured_color().unwrap() as usize] ^= to_bb;
    }
    fn queen_promo_capture(&self, b: &mut Board) {
        let from_bb = ONE << (self.get_from() as u64);
        let to_bb = ONE << (self.get_to() as u64);
        let from_to_bb = from_bb ^ to_bb;
        b.boards[Piece::Queen as usize] ^= to_bb;
        b.boards[self.get_piece() as usize] ^= from_bb;
        b.boards[self.get_color() as usize] ^= from_to_bb;
        // captured piece
        b.boards[self.get_captured_piece().unwrap() as usize] ^= to_bb;
        b.boards[self.get_captured_color().unwrap() as usize] ^= to_bb;
    }
}
*/
