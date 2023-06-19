/*
Terms:
alpha is the maximizing side
beta is the minimizing side
*/

use super::{
    bitboard::LS1B,
    board::{BoardState, Color, Piece},
    r#move::{Move, MoveType},
};

impl BoardState {
    pub fn quiescence(&mut self, mut alpha: i32, beta: i32) -> i32 {
        let evaluation = self.evaluate();
        // move fails high
        if evaluation >= beta {
            return beta;
        }

        if evaluation > alpha {
            // PV node (move)
            alpha = evaluation;
        }

        let move_list = self.board.generate_moves();

        for m in (&move_list.moves[..move_list.count]).to_vec() {
            let copy = self.preserve();

            self.ply += 1;

            // ILLEGAL MOVES
            if !self.make_move(m, MoveType::OnlyCaptures) {
                self.ply -= 1;
                continue;
            }

            let score = -self.quiescence(-beta, -alpha);
            self.restore(copy);
            self.ply -= 1;

            // move fails high
            if score >= beta {
                return beta;
            }

            if score > alpha {
                // PV node (move)
                alpha = score;
            }
        }
        
        // node (move) fails low
        alpha
    }
    pub fn negamax(&mut self, mut alpha: i32, beta: i32, depth: i32) -> i32 {
        if depth == 0 {
            return self.quiescence(alpha, beta);
        }

        self.nodes += 1;

        let mut best = Move::default();

        let mut legal_moves = 0;

        // LOOK AT THIS FANCY IF STATEMENT AHH
        // are we in-check?
        let in_check: bool = if self.board.side == Color::White {
            let white_king_bb = self
                .board
                .get_piece_of_color(Piece::King, Color::White)
                .index_of_lsb()
                .expect("there is somehow no white king on the board... what");

            // is our white king attacked by a black piece?
            self.board.is_square_attacked(white_king_bb, Color::Black)
        } else {
            let black_king_bb = self
                .board
                .get_piece_of_color(Piece::King, Color::Black)
                .index_of_lsb()
                .expect("there is somehow no black king on the board... what");

            // is our black king attacked by a white piece?
            self.board.is_square_attacked(black_king_bb, Color::White)
        };

        let old_alpha = alpha;

        let move_list = self.board.generate_moves();

        for m in (&move_list.moves[..move_list.count]).to_vec() {
            let copy = self.preserve();

            self.ply += 1;

            // ILLEGAL MOVES
            if !self.make_move(m, MoveType::AllMoves) {
                self.ply -= 1;
                continue;
            }

            legal_moves += 1;

            let score = -self.negamax(-beta, -alpha, depth - 1);
            self.restore(copy);
            self.ply -= 1;

            // move fails high
            if score >= beta {
                return beta;
            }

            if score > alpha {
                // PV node (move)
                alpha = score;

                if self.ply == 0 {
                    // associate best move with best score
                    best = m;
                }
            }
        }

        // we dont have any legal moves at all
        // meaning we are either in STALEMATE or in CHECKMATE
        if legal_moves == 0 {
            // checkmate
            if in_check {
                return -49000 + self.ply;
            } else {
                // draw!
                return 0;
            }
        }

        // found a better move
        if old_alpha != alpha {
            self.best_move = best;
        }

        // node (move) fails low
        return alpha;
    }
}
