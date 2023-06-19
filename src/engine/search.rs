/*
Terms:
alpha is the maximizing side
beta is the minimizing side
*/

use super::{
    board::{Board, BoardState},
    r#move::{Move, MoveType},
};

impl BoardState {
    pub fn negamax(&mut self, mut alpha: i32, beta: i32, depth: i32) -> i32 {
        if depth == 0 {
            return self.evaluate();
        }
        let mut best_move_so_far: Move = Move::default();
        let old_value_alpha = alpha;
        let move_list = self.board.generate_moves();

        for m in (&move_list.moves[..move_list.count]).to_vec() {
            // preserve the state so u can later restore it
            let copy = self.preserve();

            // increment ply
            board_state.board.ply += 1;

            // ILLEGAL MOVE!!
            if !board_state.make_move(m, MoveType::AllMoves) {
                board_state.board.ply -= 1;
                board_state.restore(&copy);
            }

            // negate the stuff cuz its the other person's turn now
            let score = -negamax(board_state, -beta, -alpha, depth - 1);
            board_state.board.ply -= 1;
            board_state.restore(&copy);

            // fail-hard beta cutoff
            if score >= beta {
                // node (move) fails high
                // meaning that this is where the cutoff is
                return beta;
            }

            // what if there is a better move?
            if score > alpha {
                // principal variation (PV) node
                alpha = score;

                // roottt node
                if board_state.board.ply == 0 {
                    best_move_so_far = m;
                }
            }
        }

        // found a better move
        if old_value_alpha != alpha {
            board_state.board.best_move = best_move_so_far;
        }

        // move fails low
        return alpha;
    }
}
