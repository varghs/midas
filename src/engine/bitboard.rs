// Gets value at bit
#[macro_export]
macro_rules! get_bit {
    ($bitboard:expr, $square:expr) => {
        $bitboard & ((1 as u64) << $square) != 0
    };
}


// Sets bit to 1
#[macro_export]
macro_rules! set_bit {
    ($bitboard:expr, $square:expr) => {
        $bitboard |= ((1 as u64) << $square)
    };
}


// Resets bit to 0
#[macro_export]
macro_rules! pop_bit {
    ($bitboard:expr, $square:expr) => {
        $bitboard &= !((1 as u64) << $square)
    };
}

#[macro_export]
macro_rules! tog_bit {
    ($bitboard:expr, $square:expr) => {
        $bitboard ^= ((1 as u64) << $square)
    };
}

pub type Bitboard = u64;

const EMPTY: Bitboard = 0;
const UNIVERSE: Bitboard = 0xffffffffffffffff;

const NOTAFILE: Bitboard = 0xfefefefefefefefe;
const NOTHFILE: Bitboard = 0x7f7f7f7f7f7f7f7f;

pub fn print_bitboard(b: Bitboard) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let square = rank * 8 + file;

            // prints rank numbers
            if file == 0 {
                print!("{} ", rank + 1);
            }

            print!(
                "{} ",
                get_bit!(b, square) as u8
            );
        }
        print!("\n");
    }

    // prints file letters
    print!("  a b c d e f g h\n")
}

trait BitwiseOperations {
    fn subset_of(&self, o: Self) -> bool;
    fn disjoint(&self, o: Self) -> bool;
}

impl BitwiseOperations for Bitboard {
    fn subset_of(&self, o: Self) -> bool {
        *self & o == *self
    }

    fn disjoint(&self, o: Self) -> bool {
        *self & o == 0
    }
}

trait PreShiftOperations {
    fn nort_one(&self) -> Self;
    fn sout_one(&self) -> Self;
    fn east_one(&self) -> Self;
    fn no_ea_one(&self) -> Self;
    fn so_ea_one(&self) -> Self;
    fn west_one(&self) -> Self;
    fn so_we_one(&self) -> Self;
    fn no_we_one(&self) -> Self;
}

impl PreShiftOperations for Bitboard {
    fn nort_one(&self) -> Self {*self << 8}
    fn sout_one(&self) -> Self {*self >> 8}
    fn east_one(&self) -> Self {(*self & NOTHFILE) << 1}
    fn no_ea_one(&self) -> Self {(*self & NOTHFILE) << 9}
    fn so_ea_one(&self) -> Self {(*self & NOTHFILE) >> 7}
    fn west_one(&self) -> Self {(*self & NOTAFILE) >> 1}
    fn so_we_one(&self) -> Self {(*self & NOTAFILE) >> 9}
    fn no_we_one(&self) -> Self {(*self & NOTAFILE) << 7}
}
