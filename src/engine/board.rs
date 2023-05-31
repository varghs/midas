use super::bitboard::Bitboard;
use std::convert::TryFrom;
use std::fmt::Display;

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

// King:     K
// Bishop:   B
// Pawn:     P
// Knight:   N
// Rook:     R
// Queen:    Q
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

// TODO!
/*
impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        let mut c: u64 = 1;
        while c != 0 {
            let mut s = String::new();
            let mut filled = false;
            for board in &self.boards[2..] {
                if board & c > 0 {
                    s += "1";
                    filled = true;
                    break;
                }
            }

            if !filled {
                s += "0";
            }

            output = s + &output;
            c = c << 1;
            if c % 2_u64.pow(8) == 0 {
                output += "\n";
            }
        }
        write!(f, "{}", output)
    }
}
*/
