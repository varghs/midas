use super::bitboard::Bitboard;
use std::convert::TryFrom;
use std::fmt::Display;

use crate::get_bit;

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

#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
}

pub struct Board {
    pub boards: [Bitboard; 8],
    pub double_pawn_push: bool,
    pub king_side_castle: bool,
    pub queen_side_castle: bool,
    pub en_passant: bool,
}

impl Board {
    fn new() -> Self {
        let pawns: Bitboard = 0x00FF00000000FF00;
        let rooks: Bitboard = 0x8100000000000081;
        let knights: Bitboard = 0x4200000000000042;
        let bishops: Bitboard = 0x2400000000000024;
        let queens: Bitboard = 0x0800000000000008;
        let kings: Bitboard = 0x1000000000000010;
        let black: Bitboard = 0xFFFF000000000000;
        let white: Bitboard = 0x000000000000FFFF;

        let boards: [Bitboard; 8] = [black, white, pawns, rooks, knights, bishops, queens, kings];
        Board {
            boards,
            double_pawn_push: false,
            king_side_castle: false,
            queen_side_castle: false,
            en_passant: false,
        }
    }

    fn get_piece(&self, p: Piece) -> Bitboard {
        self.boards[p as usize]
    }

    fn get_color(&self, c: Color) -> Bitboard {
        self.boards[c as usize]
    }

    fn get_piece_of_color(&self, p: Piece, c: Color) -> Bitboard {
        self.get_piece(p) & self.get_color(c)
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
                            output += format!("{} ", p).as_str();
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
        output += "\n    a b c d e f g h ";

        write!(f, "{}", output)
    }
}
