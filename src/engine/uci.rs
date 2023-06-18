use crate::engine::r#move::MoveList;

use super::{r#move::Move, square::Square};
use super::board::{Board, Piece};

impl Board {
    pub fn parse_move_string(&mut self, move_string: &str) -> Result<Move, String> {
        // TODO
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

        println!(
            "move_string: {}, source: {}, target: {}",
            move_string, source_square, target_square
        );

        // loop through all moves..
        for m in (&move_list.moves[..move_list.count]).to_vec() {
            if source_square == m.get_source() && target_square == m.get_target() {
                let promoted_piece = m.get_promoted_piece();
                if promoted_piece.is_none() && move_string.chars().nth(4).is_none() {
                    return Ok(m);
                }
            }
        }
        //     check if move is legal.. then return

        // Illegal
        return Move::default();
    }
}
