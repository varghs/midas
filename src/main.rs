mod debug;
mod test;

use debug::*;

use midas::engine::{bitboard::{print_bitboard, Bitboard}, board::{self, Board}};

fn main() {
    let pawns: Bitboard = 0x00000000000000FF;
    let def: Bitboard = 0;

    let boards: [Bitboard; 8] = [def, def, pawns, def, def, def, def, def];

    let board: Board = Board { boards };

    draw_board(&board);
}
