use std::convert::From;
use std::ops::BitAnd;

enum Piece {
    Pawn = 2,
    Rook = 3,
    Knight = 4,
    Bishop = 5,
    Queen = 6,
    King = 7,
}

enum Color {
    White = 0,
    Black = 1,
}

#[derive(Clone, Copy)]
pub struct Bitboard(pub u64);

impl From<u64> for Bitboard {
    fn from(n: u64) -> Self {
        Bitboard(n)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

pub struct Board {
    boards: [Bitboard; 8],
}

impl Board {
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
