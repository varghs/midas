const WIDTH: u8 = 8;
const HEIGHT: u8 = 8;

mod debug;
mod test;

use debug::*;

use midas::print_bitboard;
use midas::{pop_bit, set_bit};
use midas::Square;

fn main() {
    let mut b: u64 = 0;
    set_bit!(b, Square::e2 as u64);
    set_bit!(b, Square::e4 as u64);
    print_bitboard(b);
    pop_bit!(b, Square::e4 as u64);
    println!();
    print_bitboard(b);
}
