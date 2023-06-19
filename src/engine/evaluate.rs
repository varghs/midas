use super::{
    bitboard::{EMPTY, LS1B},
    board::{BoardState, Color, Piece},
    square::Square,
};

// [color][piece]
#[rustfmt::skip]
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

// pawn positional score
#[rustfmt::skip]
const PAWN_SCORE: [i32; 64] = 
[
     0,   0,   0,   0,   0,   0,   0,   0,  // a1
     0,   0,   0, -10, -10,   0,   0,   0, // a2
     0,   0,   0,   5,   5,   0,   0,   0, // a3
     5,   5,  10,  20,  20,   5,   5,   5, // a4
    10,  10,  10,  20,  20,  10,  10,  10, // a5
    20,  20,  20,  30,  30,  30,  20,  20, // a6
    30,  30,  30,  40,  40,  30,  30,  30, // a7
    90,  90,  90,  90,  90,  90,  90,  90, // a8
];

// knight positional score
#[rustfmt::skip]
const KNIGHT_SCORE: [i32; 64] = 
[
    -5, -10,   0,   0,   0,   0, -10,  -5, // a1
    -5,   0,   0,   0,   0,   0,   0,  -5, // a2
    -5,   5,  20,  10,  10,  20,   5,  -5, // a3
    -5,  10,  20,  30,  30,  20,  10,  -5, // a4
    -5,  10,  20,  30,  30,  20,  10,  -5, // a5
    -5,   5,  20,  20,  20,  20,   5,  -5, // a6
    -5,   0,   0,  10,  10,   0,   0,  -5, // a7
    -5,   0,   0,   0,   0,   0,   0,  -5, // a8
];

// bishop positional score
#[rustfmt::skip]
const BISHOP_SCORE: [i32; 64] = 
[
     0,   0, -10,   0,   0, -10,   0,   0, // a1
     0,  30,   0,   0,   0,   0,  30,   0, // a2
     0,  10,   0,   0,   0,   0,  10,   0, // a3
     0,   0,  10,  20,  20,  10,   0,   0, // a4
     0,   0,  10,  20,  20,  10,   0,   0, // a5
     0,   0,   0,  10,  10,   0,   0,   0, // a6
     0,   0,   0,   0,   0,   0,   0,   0, // a7
     0,   0,   0,   0,   0,   0,   0,   0, // a8
];

// rook positional score
#[rustfmt::skip]
const ROOK_SCORE: [i32; 64] =
[
     0,   0,   0,  20,  20,   0,   0,   0, // a1
     0,   0,  10,  20,  20,  10,   0,   0, // a2
     0,   0,  10,  20,  20,  10,   0,   0, // a3
     0,   0,  10,  20,  20,  10,   0,   0, // a4
     0,   0,  10,  20,  20,  10,   0,   0, // a5
     0,   0,  10,  20,  20,  10,   0,   0, // a6
    50,  50,  50,  50,  50,  50,  50,  50, // a7
    50,  50,  50,  50,  50,  50,  50,  50, // a8
];

// king positional score
#[rustfmt::skip]
const KING_SCORE: [i32; 64] = 
[
     0,   0,   5,   5,   5,   5,   0,   0, // a7
     0,   0,   5,   0, -15,   0,  10,   0, // a1
     0,   5,   5,  -5,  -5,   0,   5,   0, // a2
     0,   0,   5,  10,  10,   5,   0,   0, // a3
     0,   5,  10,  20,  20,  10,   5,   0, // a4
     0,   5,  10,  20,  20,  10,   5,   0, // a5
     0,   5,   5,  10,  10,   5,   5,   0, // a6
     0,   0,   0,   0,   0,   0,   0,   0, // a8
];

// mirror positional score tables for opposite side
#[rustfmt::skip]
const MIRROR_SCORE: [Square; 64] =
[
	Square::a8, Square::b8, Square::c8, Square::d8, Square::e8, Square::f8, Square::g8, Square::h8,
	Square::a7, Square::b7, Square::c7, Square::d7, Square::e7, Square::f7, Square::g7, Square::h7,
	Square::a6, Square::b6, Square::c6, Square::d6, Square::e6, Square::f6, Square::g6, Square::h6,
	Square::a5, Square::b5, Square::c5, Square::d5, Square::e5, Square::f5, Square::g5, Square::h5,
	Square::a4, Square::b4, Square::c4, Square::d4, Square::e4, Square::f4, Square::g4, Square::h4,
	Square::a3, Square::b3, Square::c3, Square::d3, Square::e3, Square::f3, Square::g3, Square::h3,
	Square::a2, Square::b2, Square::c2, Square::d2, Square::e2, Square::f2, Square::g2, Square::h2,
	Square::a1, Square::b1, Square::c1, Square::d1, Square::e1, Square::f1, Square::g1, Square::h1,
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
                    // material score
                    score += MATERIAL_SCORE[color][piece - 2];

                    let c: Color = color.try_into().unwrap();
                    let p: Piece = piece.try_into().unwrap();
                    let s = bitboard.index_of_lsb().unwrap();

                    // positional scoring
                    match (c, p) {
                        (Color::White, Piece::Pawn) => score += PAWN_SCORE[s as usize],
                        (Color::White, Piece::Rook) => score += ROOK_SCORE[s as usize],
                        (Color::White, Piece::Knight) => score += KNIGHT_SCORE[s as usize],
                        (Color::White, Piece::Bishop) => score += BISHOP_SCORE[s as usize],
                        (Color::White, Piece::King) => score += KING_SCORE[s as usize],

                        (Color::Black, Piece::Pawn) => {
                            score -= PAWN_SCORE[MIRROR_SCORE[s as usize] as usize]
                        }
                        (Color::Black, Piece::Rook) => {
                            score -= ROOK_SCORE[MIRROR_SCORE[s as usize] as usize]
                        }
                        (Color::Black, Piece::Knight) => {
                            score -= KNIGHT_SCORE[MIRROR_SCORE[s as usize] as usize]
                        }
                        (Color::Black, Piece::Bishop) => {
                            score -= BISHOP_SCORE[MIRROR_SCORE[s as usize] as usize]
                        }
                        (Color::Black, Piece::King) => {
                            score -= KING_SCORE[MIRROR_SCORE[s as usize] as usize]
                        }

                        _ => {}
                    }
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
