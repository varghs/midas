use super::board::Piece;
use super::board::*;
use crate::{get_bit, pop_bit, set_bit, tog_bit};

pub struct FEN<'a>(pub &'a str);

pub const EMPTY_BOARD: FEN = FEN("8/8/8/8/8/8/8/8 w - - ");
pub const START_POSITION: FEN = FEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ");
pub const TRICKY_POSITION: FEN =
    FEN("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ");
pub const KILLER_POSITION: FEN =
    FEN("rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1");
pub const CMK_POSITION: FEN =
    FEN("r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9 ");

impl Board {
    pub fn parse_fen(&mut self, fen: FEN) {
        self.boards.fill(0);
        self.side = Color::White;
        self.en_passant_sq = None;
        self.castle = CastleRep(0);

        let mut fen_iter = fen.0.chars();

        let mut break_condition = false;

        let mut rank: i32 = 7;
        let mut file: i32 = 0;

        while rank >= 0 {
            while file < 8 {
                let square = rank * 8 + file;
                let char = fen_iter.next().unwrap();
                if char >= 'a' && char <= 'z' {
                    let piece: Piece = char.try_into().unwrap_or(Piece::Pawn);
                    set_bit!(self.boards[piece as usize], square);
                    set_bit!(self.boards[Color::Black as usize], square);
                    file += 1;
                } else if char >= 'A' && char <= 'Z' {
                    let piece: Piece = char.try_into().unwrap_or(Piece::Pawn);
                    set_bit!(self.boards[piece as usize], square);
                    set_bit!(self.boards[Color::White as usize], square);
                    file += 1;
                } else if char >= '0' && char <= '9' {
                    let offset = (char as i32) - ('0' as i32);
                    file += offset;
                } else if char == '/' {
                } else {
                    break_condition = true;
                    break;
                }
            }
            rank -= 1;
            file = 0;
            if break_condition {
                break;
            }
        }
        fen_iter.next();
        match fen_iter.next() {
            Some('w') => self.side = Color::White,
            Some('b') => self.side = Color::Black,
            _ => panic!("Invalid side in FEN."),
        }

        fen_iter.next();
        let mut char = fen_iter.next().unwrap();
        while char != ' ' {
            match char {
                'K' => self.castle.set_castle(Castle::WhiteKing),
                'Q' => self.castle.set_castle(Castle::WhiteQueen),
                'k' => self.castle.set_castle(Castle::BlackKing),
                'q' => self.castle.set_castle(Castle::BlackQueen),
                '-' => {}
                _ => panic!("Invalid castling in FEN."),
            }
            char = fen_iter.next().unwrap();
        }

        char = fen_iter.next().unwrap();
        let mut sq = String::new();
        if char != '-' {
            sq.push(char);
            sq.push(fen_iter.next().unwrap());
            self.en_passant_sq = Some(sq.as_str().try_into().unwrap());
        } else {
            self.en_passant_sq = None;
        }

        // todo: half move clock and fullmove counter
    }
}
