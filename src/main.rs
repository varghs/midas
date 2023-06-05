mod test;

use midas::{
    engine::{
        bitboard::Bitboard,
        board::{Board, Square},
    },
    set_bit,
};

fn main() {
    let pawns: Bitboard = 0x00FF00000000FF00;
    let rooks: Bitboard = 0x8100000000000081;
    let knights: Bitboard = 0x4200000000000042;
    let bishops: Bitboard = 0x2400000000000024;
    let queens: Bitboard = 0x0800000000000008;
    let kings: Bitboard = 0x1000000000000010;
    let black: Bitboard = 0xFFFF000000000000;
    let white: Bitboard = 0x000000000000FFFF;

    let mut boards: [Bitboard; 8] = [black, white, pawns, rooks, knights, bishops, queens, kings];
    let board: Board = Board { boards };

    print!("{}", board);
}
