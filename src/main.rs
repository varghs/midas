const WIDTH: u8 = 8;
const HEIGHT: u8 = 8;

mod debug;
mod test;

use debug::*;

use midas::print_bitboard;
use midas::{pop_bit, set_bit};
use midas::Square;

fn main() {
    let b = 0x8040201008040201;
    print_bitboard(b);
}
