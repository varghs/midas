use crate::engine::board::Piece;

use super::{board::BoardState, r#move::Move, square::Square};

pub fn parse_move_string(board_state: &mut BoardState, move_string: &str) -> Move {
    let mut move_list = board_state.board.generate_moves();

    // TODO
    // create an instance of move_list
    // loop through all moves..
    //     check if move is legal.. then return
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

    // loop over all the moves in the move_list
    for m in (&move_list.moves[..move_list.count]).to_vec() {
        // make sure source/target squares are actually valid
        if source_square == m.get_source() && target_square == m.get_target() {
            if let Some(promoted_piece) = m.get_promoted_piece() {
                let promoted_char = move_string.chars().nth(4).unwrap_or('a');
                // promoted to queen
                if promoted_piece == Piece::Queen && promoted_char == 'q' {
                    return m;
                }
                if promoted_piece == Piece::Rook && promoted_char == 'q' {
                    return m;
                }
                if promoted_piece == Piece::Bishop && promoted_char == 'q' {
                    return m;
                }
                if promoted_piece == Piece::Knight && promoted_char == 'q' {
                    return m;
                }
            }
        }
    }

    // Illegal
    return Move::default();
}
