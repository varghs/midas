use super::{square::Square, bitboard::Bitboard, board::{Board, Piece, Color}};

impl Board {
    pub fn generate_moves(&self) {
        let (source, target): (Square, Square);
        let (mut bitboard, attacks): (Bitboard, Bitboard);

        for piece in (Piece::Pawn as usize)..(Piece::King as usize) {
            bitboard = self.boards[piece];

            match self.side {
                Color::White => {} // white pawns and castling,
                Color::Black => {} // black pawns and castling,
            }

            // knight
            
            // bishop
            
            // rook
            
            // queen
            
            // king
        }
    }
}
