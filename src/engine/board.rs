use super::{bitboard::{Bitboard, print_bitboard}, attacks::AttackTables};
use std::convert::TryFrom;
use std::fmt::Display;
use crate::engine::square::Square;
use super::fen::FEN;

use crate::{get_bit, set_bit};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Pawn = 2,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str;
        match self {
            Piece::Pawn => s = "P",
            Piece::Rook => s = "R",
            Piece::Knight => s = "N",
            Piece::Bishop => s = "B",
            Piece::Queen => s = "Q",
            Piece::King => s = "K",
        }

        write!(f, "{}", s)
    }
}

impl TryFrom<char> for Piece {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'P' | 'p' => Ok(Piece::Pawn),
            'R' | 'r' => Ok(Piece::Rook),
            'N' | 'n' => Ok(Piece::Knight),
            'B' | 'b' => Ok(Piece::Bishop),
            'Q' | 'q' => Ok(Piece::Queen),
            'K' | 'k' => Ok(Piece::King),
            _ => Err("Invalid char to piece conversion".to_string())
        }
    }
}

impl TryFrom<usize> for Piece {
    type Error = String;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Piece::Pawn),
            3 => Ok(Piece::Rook),
            4 => Ok(Piece::Knight),
            5 => Ok(Piece::Bishop),
            6 => Ok(Piece::Queen),
            7 => Ok(Piece::King),
            _ => Err("Invalid conversion".to_string()),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        match self {
            Color::White => output += "White",
            Color::Black => output += "Black",
        }
        write!(f, "{}", output)
    }
}

#[derive(Clone, Copy)]
pub enum Castle {
    WhiteKing = 1,
    WhiteQueen = 2,
    BlackKing = 4,
    BlackQueen = 8,
}

pub struct CastleRep(u8);

impl CastleRep {
    pub fn new() -> Self {
        Self(0xF)
    }
    pub fn can_castle(&self, castle: Castle) -> bool {
        self.0 & (castle as u8) > 0
    }

    pub fn set_castle(&mut self, castle: Castle) {
        self.0 |= castle as u8;
    }

    pub fn unset_castle(&mut self, castle: Castle) {
        self.0 &= !(castle as u8);
    }

    pub fn tog_castle(&mut self, castle: Castle) {
        self.0 ^= castle as u8;
    }
}

impl Display for CastleRep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        if self.can_castle(Castle::WhiteKing) {
            output += "K";
        } else {
            output += "-";
        }
        if self.can_castle(Castle::WhiteQueen) {
            output += "Q";
        } else {
            output += "-";
        }
        if self.can_castle(Castle::BlackKing) {
            output += "k";
        } else {
            output += "-";
        }
        if self.can_castle(Castle::BlackQueen) {
            output += "q";
        } else {
            output += "-";
        }

        write!(f, "{}", output)
    }
}

pub struct Board {
    pub boards: [Bitboard; 8],
    pub side: Color,
    pub en_passant_sq: Option<Square>,
    pub castle: CastleRep,
    pub attack_tables: AttackTables,
}

impl Board {
    pub fn new() -> Self {
        let pawns: Bitboard = 0x00FF00000000FF00;
        let rooks: Bitboard = 0x8100000000000081;
        let knights: Bitboard = 0x4200000000000042;
        let bishops: Bitboard = 0x2400000000000024;
        let queens: Bitboard = 0x0800000000000008;
        let kings: Bitboard = 0x1000000000000010;
        let black: Bitboard = 0xFFFF000000000000;
        let white: Bitboard = 0x000000000000FFFF;

        let side = Color::White;
        let en_passant_sq: Option<Square> = Some(Square::e3);

        let mut attack_tables = AttackTables::new();
        attack_tables.populate();

        let boards: [Bitboard; 8] = [white, black, pawns, rooks, knights, bishops, queens, kings];
        Board {
            boards,
            side,
            castle: CastleRep::new(),
            en_passant_sq,
            attack_tables,
        }
    }

    pub fn set_piece_color(&mut self, p: Piece, c: Color, s: Square) {
        set_bit!(self.boards[p as usize], s);
        set_bit!(self.boards[c as usize], s);
    }

    pub fn get_piece(&self, p: Piece) -> Bitboard {
        self.boards[p as usize]
    }

    pub fn get_color(&self, c: Color) -> Bitboard {
        self.boards[c as usize]
    }

    pub fn get_occupancies(&self) -> Bitboard {
        self.get_color(Color::White) | self.get_color(Color::Black)
    }

    pub fn get_piece_of_color(&self, p: Piece, c: Color) -> Bitboard {
        self.get_piece(p) & self.get_color(c)
    }

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
            _ => panic!("Invalid side in FEN.")
        }

        fen_iter.next();
        let mut char = fen_iter.next().unwrap();
        while char != ' ' {
            match char {
                'K' => self.castle.set_castle(Castle::WhiteKing),
                'Q' => self.castle.set_castle(Castle::WhiteQueen),
                'k' => self.castle.set_castle(Castle::BlackKing),
                'q' => self.castle.set_castle(Castle::BlackQueen),
                '-' => {},
                _ => panic!("Invalid castling in FEN.")
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

        println!("{}", fen_iter.as_str());

        // todo: half move clock and fullmove counter
    }

    pub fn is_square_attacked(&self, square: Square, side: Color) -> bool {

        // attacked by white pawns 
        if side == Color::White && (self.attack_tables.pawns.pawn_attacks[Color::Black as usize][square as usize] & self.get_piece_of_color(Piece::Pawn, Color::White) > 0) {
            return true;
        }

        // attacked by black pawns 
        if side == Color::Black && (self.attack_tables.pawns.pawn_attacks[Color::White as usize][square as usize] & self.get_piece_of_color(Piece::Pawn, Color::White) > 0) {
            return true;
        }

        // attacked by knight
        if self.attack_tables.knights.knight_attacks[square as usize] & self.get_piece_of_color(Piece::Knight, side) > 0 {
            return true;
        }

        // attacked by bishop
        if (self.attack_tables.sliders.bishops.get_bishop_attack(square, self.get_occupancies()) & self.get_piece_of_color(Piece::Bishop, side)) > 0 {
            return true;
        }

        // attacked by rook 
        if (self.attack_tables.sliders.rooks.get_rook_attack(square, self.get_occupancies()) & self.get_piece_of_color(Piece::Rook, side)) > 0 {
            return true;
        }

        // attacked by queen 
        if (self.attack_tables.sliders.get_queen_attack(square, self.get_occupancies()) & self.get_piece_of_color(Piece::Queen, side)) > 0 {
            return true;
        }
        // attacked by king
        if self.attack_tables.kings.king_attacks[square as usize] & self.get_piece_of_color(Piece::King, side) > 0 {
            return true;
        }

        false
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: String = String::new();
        output += "\n";
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = rank * 8 + file;
                let mut filled = false;

                // print ranks
                if file == 0 {
                    output += format!(" {}  ", rank + 1).as_str();
                }

                for board_idx in 2..8 {
                    match get_bit!(self.boards[board_idx], square) {
                        true => {
                            filled = true;
                            let p: Piece = board_idx.try_into().unwrap();
                            match self.get_color(Color::White) & (1 << square) {
                                0 => {
                                    output += format!("{} ", p).to_lowercase().as_str();
                                },
                                _ => {
                                    output += format!("{} ", p).as_str();
                                }
                            }
                        }
                        false => (),
                    }
                }

                if !filled {
                    output += ". ";
                }
            }
            output += "\n";
        }
        // print files
        output += "\n    a b c d e f g h \n";
        output += format!("Side to move: {} \n", self.side).as_str();
        output += format!("En passant square: {}\n", match self.en_passant_sq {
            Some(i) => i.to_string(),
            None => String::from("None"),
        }).as_str();

        output += format!("Castling: {}", self.castle).as_str();

        write!(f, "{}", output)
    }
}
