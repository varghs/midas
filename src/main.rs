mod debug;
mod test;

use debug::*;

use midas::engine::bitboard::print_bitboard;

fn main() {
    let b = 0x8040201008040201;
    print_bitboard(b);
}
