use super::{
    bitboard::{EMPTY, LS1B},
    board::{BoardState, Color, Piece},
};

// [color][piece]
const MATERIAL_SCORE: [[i32; 6]; 2] = [
    [
        100,  // white pawn
        500,  // white rook
        300,  // white knight
        350,  // white bishop
        1000, // white queen
        10000,
    ], // white king
    [
        -100,  // black pawn
        -500,  // black rook
        -300,  // black knight
        -350,  // black bishop
        -1000, // black queen
        -10000,
    ], // black king
];

impl BoardState {
    pub fn evaluate(&self) -> i32 {
        let mut score = 0;
        for color in (Color::White as usize)..=(Color::Black as usize) {
            for piece in (Piece::Pawn as usize)..=(Piece::King as usize) {
                let mut bitboard = self
                    .board
                    .get_piece_of_color(piece.try_into().unwrap(), color.try_into().unwrap());
                while bitboard > 0 {
                    score += MATERIAL_SCORE[color][piece - 2];
                    bitboard.pop_lsb();
                }
            }
        }
        match self.board.side {
            Color::White => score,
            Color::Black => -score,
        }
    }
}
