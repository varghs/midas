// Place for temporary debugging stuff
use midas::engine::bitboard::Bitboard;
use midas::engine::board::Board;
use midas::get_bit;
use midas::engine::board::Piece;

pub fn draw_board(board: &Board) {
    // draw lines
    // King:     K
    // Bishop:   B
    // Pawn:     P
    // Knight:   N 
    // Rook:     R
    // Queen:    Q
    // 64 squares, 2 characters each
    let mut output: String = String::new();
    // using lerf
    for rank in (0..8).rev() {
        for file in 0..8 {
            let square = rank * 8 + file;
            let mut filled = false;

            for board_idx in 2..8 {
                match get_bit!(board.boards[board_idx], square) {
                    true => {
                        filled = true;
                        let p: Piece = board_idx.try_into().unwrap();
                        output += format!("{} ", p).as_str();
                    },
                    false => (),
                }
            }

            if !filled {
                output += ". ";
            }
        }
        output += "\n";
    }

    print!("{}", output);
}

/// Modifies out to add the newly addded bits
pub fn get_position(out: &mut String, mut bits: Bitboard, piece: String) {
    let mut counter = 0;
    while bits != 0 {
        let cur_bit = bits % 2;

        if cur_bit == 1 {
            out.replace_range(counter * 3..counter * 3 + 3, &piece);
        }
        counter += 1;
        bits /= 2;
    }
}
