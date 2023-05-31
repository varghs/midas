const WIDTH: u8 = 8;
const HEIGHT: u8 = 8;

mod debug;
mod test;

use debug::*;

use midas::engine::bitboard::print_bitboard;

fn main() {
    let b = 0x8040201008040201;
    print_bitboard(b);
}
