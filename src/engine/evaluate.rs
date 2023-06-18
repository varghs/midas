use super::{
    bitboard::EMPTY,
    board::{BoardState, Color},
};

impl BoardState {
    pub fn evaluate_material(&self) -> isize {
        let mut score = 0_isize;

        // let boards: [Bitboard; 8] = [white, black, pawns, rooks, knights, bishops, queens, kings];
        // for piece_index in 0..self.board.boards.len() {
        //     let piece = piece_index
        //     while cur_bitboard != 0 {

        //     }
        // }
        if self.board.side == Color::White {
            score
        } else {
            -score
        }
    }
    pub fn evaluate(&self) -> isize {
        self.evaluate_material()
    }
}
