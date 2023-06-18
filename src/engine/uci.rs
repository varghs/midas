use super::{r#move::Move, square::Square};

pub fn parse_move_string(move_string: &str) -> Move {
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

    // Illegal
    return Move::default();
}
