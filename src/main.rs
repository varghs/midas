mod debug;
mod test;

use debug::*;

use midas::engine::{bitboard::{print_bitboard, Bitboard}, board::{self, Board}};

fn main() {
    let pawns: Bitboard = 0x00FF00000000FF00;
    let rooks: Bitboard = 0x8100000000000081;
    let knights: Bitboard = 0x4200000000000042;
    let bishops: Bitboard = 0x2400000000000024;
    let queens: Bitboard = 0x0800000000000008;
    let kings: Bitboard = 0x1000000000000010;
    let def: Bitboard = 0;

    let boards: [Bitboard; 8] = [def, def, pawns, rooks, knights, bishops, queens, kings];

    let board: Board = Board { boards };

    draw_board(&board);
}
