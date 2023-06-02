use super::bitboard::Bitboard;
use std::convert::TryFrom;
use std::fmt::Display;

use crate::get_bit;

#[rustfmt::skip]
pub enum Square {
  a1, b1, c1, d1, e1, f1, g1, h1,
  a2, b2, c2, d2, e2, f2, g2, h2,
  a3, b3, c3, d3, e3, f3, g3, h3,
  a4, b4, c4, d4, e4, f4, g4, h4,
  a5, b5, c5, d5, e5, f5, g5, h5,
  a6, b6, c6, d6, e6, f6, g6, h6,
  a7, b7, c7, d7, e7, f7, g7, h7,
  a8, b8, c8, d8, e8, f8, g8, h8
}

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

enum Color {
    White,
    Black,
}

pub struct Board {
    pub boards: [Bitboard; 8],
}

impl Board {
    fn new() -> Self {
        // let mut arr: [Bitboard; 8] = [];
        // let mut b = Board { boards: arr };
        todo!()
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
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = rank * 8 + file;
                let mut filled = false;

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

        write!(f, "{}", output)
    }
}
