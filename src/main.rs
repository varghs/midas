const WIDTH: u8 = 8;
const HEIGHT: u8 = 8;

mod debug;
mod test;

use debug::*;

use midas::print_bitboard;

fn main() {
    let b: u64 = 0x00000000000000FF;
    print_bitboard(b);
}
