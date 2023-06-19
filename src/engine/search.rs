use super::{board::BoardState, r#move::{MoveType, Move}};

impl BoardState {
    pub fn negamax(&mut self, mut alpha: i32, beta: i32, depth: i32) -> i32 {
        if depth == 0 {
            return self.evaluate();
        }

        self.nodes += 1;

        let mut best = Move::default();

        let old_alpha = alpha;

        let move_list = self.board.generate_moves();

        for m in (&move_list.moves[..move_list.count]).to_vec() {
            let copy = self.preserve();

            self.ply += 1;

            if !self.make_move(m, MoveType::AllMoves) {
                self.ply -= 1;
                continue;
            }

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

        if old_alpha != alpha {
            self.best_move = best;
        }

        // node (move) fails low
        return alpha;
    }
}
