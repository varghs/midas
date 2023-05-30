// Place for temporary debugging stuff
use crate::{HEIGHT, WIDTH};
use midas::{Bitboard, Board};

fn draw_board(board: &Board) {
    // draw lines
    // King:     Ki
    // Bishop:   Bi
    // Pawn:     Pa
    // Knight:   Kn
    // Rook:     Ro
    // Queen:
    // 64 squares, 3 characters each
    let output: String = String::with_capacity(64 * 3);
    // a1 is 0 * 3, b1 is 1 * 3, h8 is 59 * 3
    // using lerf

    /* pseudo code
    get_position(white_pawn)
    get_position(white_king)
    etc....

    get_position(output: String):
        every 3 steps put the ouput from a1 to h8
        change the stuff at certain position
        so 64 sqaures * 3 characters each == length of string
    */
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
