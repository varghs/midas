use super::bitboard::Bitboard;
use std::convert::TryFrom;
use std::fmt::Display;

use crate::get_bit;

#[allow(non_camel_case_types)]
#[rustfmt::skip]
pub enum Square {
  a1, b1, c1, d1, e1, f1, g1, h1,
  a2, b2, c2, d2, e2, f2, g2, h2,
  a3, b3, c3, d3, e3, f3, g3, h3,
  a4, b4, c4, d4, e4, f4, g4, h4,
  a5, b5, c5, d5, e5, f5, g5, h5,
  a6, b6, c6, d6, e6, f6, g6, h6,
  a7, b7, c7, d7, e7, f7, g7, h7,
  a8, b8, c8, d8, e8, f8, g8, h8,
}

impl TryFrom<u64> for Square {
    type Error = String;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Square::a1),
            1 => Ok(Square::b1),
            2 => Ok(Square::c1),
            3 => Ok(Square::d1),
            4 => Ok(Square::e1),
            5 => Ok(Square::f1),
            6 => Ok(Square::g1),
            7 => Ok(Square::h1),
            8 => Ok(Square::a2),
            9 => Ok(Square::b2),
            10 => Ok(Square::c2),
            11 => Ok(Square::d2),
            12 => Ok(Square::e2),
            13 => Ok(Square::f2),
            14 => Ok(Square::g2),
            15 => Ok(Square::h2),
            16 => Ok(Square::a3),
            17 => Ok(Square::b3),
            18 => Ok(Square::c3),
            19 => Ok(Square::d3),
            20 => Ok(Square::e3),
            21 => Ok(Square::f3),
            22 => Ok(Square::g3),
            23 => Ok(Square::h3),
            24 => Ok(Square::a4),
            25 => Ok(Square::b4),
            26 => Ok(Square::c4),
            27 => Ok(Square::d4),
            28 => Ok(Square::e4),
            29 => Ok(Square::f4),
            30 => Ok(Square::g4),
            31 => Ok(Square::h4),
            32 => Ok(Square::a5),
            33 => Ok(Square::b5),
            34 => Ok(Square::c5),
            35 => Ok(Square::d5),
            36 => Ok(Square::e5),
            37 => Ok(Square::f5),
            38 => Ok(Square::g5),
            39 => Ok(Square::h5),
            40 => Ok(Square::a6),
            41 => Ok(Square::b6),
            42 => Ok(Square::c6),
            43 => Ok(Square::d6),
            44 => Ok(Square::e6),
            45 => Ok(Square::f6),
            46 => Ok(Square::g6),
            47 => Ok(Square::h6),
            48 => Ok(Square::a7),
            49 => Ok(Square::b7),
            50 => Ok(Square::c7),
            51 => Ok(Square::d7),
            52 => Ok(Square::e7),
            53 => Ok(Square::f7),
            54 => Ok(Square::g7),
            55 => Ok(Square::h7),
            56 => Ok(Square::a8),
            57 => Ok(Square::b8),
            58 => Ok(Square::c8),
            59 => Ok(Square::d8),
            60 => Ok(Square::e8),
            61 => Ok(Square::f8),
            62 => Ok(Square::g8),
            63 => Ok(Square::h8),
            _ => Err("Invalid conversion".to_string()),
        }
    }
}

impl TryFrom<u16> for Square {
    type Error = String;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Square::a1),
            1 => Ok(Square::b1),
            2 => Ok(Square::c1),
            3 => Ok(Square::d1),
            4 => Ok(Square::e1),
            5 => Ok(Square::f1),
            6 => Ok(Square::g1),
            7 => Ok(Square::h1),
            8 => Ok(Square::a2),
            9 => Ok(Square::b2),
            10 => Ok(Square::c2),
            11 => Ok(Square::d2),
            12 => Ok(Square::e2),
            13 => Ok(Square::f2),
            14 => Ok(Square::g2),
            15 => Ok(Square::h2),
            16 => Ok(Square::a3),
            17 => Ok(Square::b3),
            18 => Ok(Square::c3),
            19 => Ok(Square::d3),
            20 => Ok(Square::e3),
            21 => Ok(Square::f3),
            22 => Ok(Square::g3),
            23 => Ok(Square::h3),
            24 => Ok(Square::a4),
            25 => Ok(Square::b4),
            26 => Ok(Square::c4),
            27 => Ok(Square::d4),
            28 => Ok(Square::e4),
            29 => Ok(Square::f4),
            30 => Ok(Square::g4),
            31 => Ok(Square::h4),
            32 => Ok(Square::a5),
            33 => Ok(Square::b5),
            34 => Ok(Square::c5),
            35 => Ok(Square::d5),
            36 => Ok(Square::e5),
            37 => Ok(Square::f5),
            38 => Ok(Square::g5),
            39 => Ok(Square::h5),
            40 => Ok(Square::a6),
            41 => Ok(Square::b6),
            42 => Ok(Square::c6),
            43 => Ok(Square::d6),
            44 => Ok(Square::e6),
            45 => Ok(Square::f6),
            46 => Ok(Square::g6),
            47 => Ok(Square::h6),
            48 => Ok(Square::a7),
            49 => Ok(Square::b7),
            50 => Ok(Square::c7),
            51 => Ok(Square::d7),
            52 => Ok(Square::e7),
            53 => Ok(Square::f7),
            54 => Ok(Square::g7),
            55 => Ok(Square::h7),
            56 => Ok(Square::a8),
            57 => Ok(Square::b8),
            58 => Ok(Square::c8),
            59 => Ok(Square::d8),
            60 => Ok(Square::e8),
            61 => Ok(Square::f8),
            62 => Ok(Square::g8),
            63 => Ok(Square::h8),
            _ => Err("Invalid conversion".to_string()),
        }
    }
}

#[derive(Clone, Copy)]
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
    pub en_pessant: bool,
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
