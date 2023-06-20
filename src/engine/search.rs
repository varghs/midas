/*
Terms:
alpha is the maximizing side
beta is the minimizing side
*/

use crate::get_bit;

use super::{
    bitboard::LS1B,
    board::{BoardState, Color, Piece},
    r#move::{Move, MoveType, MoveList},
};

// most valuable victim & less valuable attacker

/*
                          
    (Victims) Pawn Knight Bishop   Rook  Queen   King
  (Attackers)
        Pawn   105    205    305    405    505    605
      Knight   104    204    304    404    504    604
      Bishop   103    203    303    403    503    603
        Rook   102    202    302    402    502    602
       Queen   101    201    301    401    501    601
        King   100    200    300    400    500    600

*/

// [attacker_color][attacker_piece][victim_color][victim_piece]
pub const MVV_LVA: [[[[i32; 6]; 2]; 6]; 2] = [
    [[[105, 405, 205, 305, 505, 605], [105, 405, 205, 305, 505, 605]],
    [[102, 402, 202, 302, 502, 602], [102, 402, 202, 302, 502, 602]],
    [[104, 404, 204, 304, 504, 604], [104, 404, 204, 304, 504, 604]],
    [[103, 403, 203, 303, 503, 603], [103, 403, 203, 303, 503, 603]],
    [[101, 401, 201, 301, 501, 601], [101, 401, 201, 301, 501, 601]],
    [[100, 400, 200, 300, 500, 600], [100, 400, 200, 300, 500, 600]]],

    [[[105, 405, 205, 305, 505, 605], [105, 405, 205, 305, 505, 605]],
    [[102, 402, 202, 302, 502, 602], [102, 402, 202, 302, 502, 602]],
    [[104, 404, 204, 304, 504, 604], [104, 404, 204, 304, 504, 604]],
    [[103, 403, 203, 303, 503, 603], [103, 403, 203, 303, 503, 603]],
    [[101, 401, 201, 301, 501, 601], [101, 401, 201, 301, 501, 601]],
    [[100, 400, 200, 300, 500, 600], [100, 400, 200, 300, 500, 600]]],
];

impl BoardState {
    pub fn score_move(&mut self, m: Move) -> i32 {
        if m.capture() {
            let mut target = Piece::Pawn;

            for p in (Piece::Pawn as usize)..=(Piece::King as usize) {
                if get_bit!(self.board.get_piece_of_color(p.try_into().unwrap(), !self.board.side), m.get_target()) {
                    target = p.try_into().unwrap();
                    break;
                }
            }

            return MVV_LVA[self.board.side as usize][m.get_piece() as usize - 2][!self.board.side as usize][target as usize - 2];
        }
        0
    }

    pub fn sort_moves(&mut self, move_list: &mut MoveList) {
        let mut move_scores: Vec<i32> = vec![0; move_list.count];

        for i in 0..move_list.count {
            *move_scores.get_mut(i).unwrap() = self.score_move(move_list.moves[i]);
        }

        for current_move in 0..move_list.count {
            for next_move in 0..move_list.count {
                if move_scores.get(current_move).unwrap() > move_scores.get(next_move).unwrap() {
                    let temp_score = *move_scores.get(current_move).unwrap();
                    *move_scores.get_mut(current_move).unwrap() = *move_scores.get(next_move).unwrap();
                    *move_scores.get_mut(next_move).unwrap() = temp_score;

                    let temp_move = move_list.moves[current_move];
                    move_list.moves[current_move] = move_list.moves[next_move];
                    move_list.moves[next_move] = temp_move;
                }
            }
        }
    }

    pub fn quiescence(&mut self, mut alpha: i32, beta: i32) -> i32 {
        self.nodes += 1;
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
    pub fn negamax(&mut self, mut alpha: i32, beta: i32, mut depth: i32) -> i32 {
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

        if in_check {
            depth += 1;
        }

        let old_alpha = alpha;

        let mut move_list = self.board.generate_moves();

        self.sort_moves(&mut move_list);

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
