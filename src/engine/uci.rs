use crate::engine::r#move::MoveList;

use super::{r#move::Move, square::Square};
use super::board::{Board, Piece};

impl Board {
    pub fn parse_move_string(&mut self, move_string: &str) -> Result<Move, String> {
        // create an instance of move_list
        let move_list = self.generate_moves();

        // parse source_square using some fancy ASCII isnt that kewl!?
        let source_square: Square = ((move_string.chars().nth(0).expect("UHHH BAD MOVE") as usize
            - 'a' as usize)
            + (move_string.chars().nth(1).expect("UHHH BAD MOVE LOL") as usize - '0' as usize - 1) * 8)
            .try_into()
            .expect("how did usize not turn into square?");


        // parse target_square using some fancy ASCII isnt that kewl!?
        let target_square: Square = ((move_string.chars().nth(2).expect("UHHH BAD MOVE") as usize
            - 'a' as usize)
            + (move_string.chars().nth(3).expect("UHHH BAD MOVE LOL") as usize - '0' as usize - 1) * 8)
            .try_into()
            .expect("how did usize not turn into square?");

        // loop over all the moves in the move_list
        for m in (&move_list.moves[..move_list.count]).to_vec() {
            // make sure source/target squares are actually valid
            if source_square == m.get_source() && target_square == m.get_target() {
                if let Some(promoted_piece) = m.get_promoted_piece() {
                    let promoted_char = move_string.chars().nth(4).unwrap_or('a');

                    if promoted_char == 'a' {
                        return Err("Must promote.".to_string());
                    }

                    // promoted to queen
                    if promoted_piece == Piece::Queen && promoted_char == 'q' {
                        return Ok(m);
                    }
                    if promoted_piece == Piece::Rook && promoted_char == 'r' {
                        return Ok(m);
                    }
                    if promoted_piece == Piece::Bishop && promoted_char == 'b' {
                        return Ok(m);
                    }
                    if promoted_piece == Piece::Knight && promoted_char == 'n' {
                        return Ok(m);
                    }
                    continue;
                }
                return Ok(m);
            }
        }
        // Illegal
        return Err("Illegal move.".to_string());
    }

    pub fn 
}
