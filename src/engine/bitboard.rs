use super::board::Color;
use super::board::Square;

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

pub const EMPTY: Bitboard = 0;
pub const UNIVERSE: Bitboard = 0xffffffffffffffff;

pub const NOTAFILE: Bitboard = 0xfefefefefefefefe;
pub const NOTHFILE: Bitboard = 0x7f7f7f7f7f7f7f7f;

pub fn print_bitboard(b: Bitboard) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let square = rank * 8 + file;

            // prints rank numbers
            if file == 0 {
                print!("{} ", rank + 1);
            }

            print!("{} ", get_bit!(b, square) as u8);
        }
        print!("\n");
    }

    // prints file letters
    print!("  a b c d e f g h\n")
}

const INDEX_64_KIM: [u64; 64] = [
    0, 47, 1, 56, 48, 27, 2, 60, 57, 49, 41, 37, 28, 16, 3, 61, 54, 58, 35, 52, 50, 42, 21, 44, 38,
    32, 29, 23, 17, 11, 4, 62, 46, 55, 26, 59, 40, 36, 15, 53, 34, 51, 20, 43, 31, 22, 10, 45, 25,
    39, 14, 33, 19, 30, 9, 24, 13, 18, 8, 12, 7, 6, 5, 63,
];

pub trait LS1B {
    fn index_of_lsb(&self) -> Option<Square>;
    fn pop_lsb(&mut self) -> Option<Square>;
}
impl LS1B for Bitboard {
    /**
     * bitScanForward
     * @author Kim Walisch (2012)
     * @param bb bitboard to scan
     * @precondition bb != 0
     * @return index (0..63) of least significant one bit
     */
    fn index_of_lsb(&self) -> Option<Square> {
        // ********* KIM WALISCH *******
        // using some fancy stuff
        const DEBRUIJN_64: u64 = 0x03f79d71b4cb0a89;
        if *self == 0 {
            return None;
        }
        // println!(
        //     "deb index is {}",
        //     (((*self ^ (*self).wrapping_sub(1)).wrapping_mul(DEBRUIJN_64)).wrapping_shr(58))
        //         as usize
        // );
        return Some(
            INDEX_64_KIM[(((*self ^ (*self).wrapping_sub(1)).wrapping_mul(DEBRUIJN_64))
                .wrapping_shr(58)) as usize]
                .try_into()
                .unwrap(),
        );
    }
    fn pop_lsb(&mut self) -> Option<Square> {
        // ********* KIM WALISCH *******
        // using some fancy stuff
        const DEBRUIJN_64: u64 = 0x03f79d71b4cb0a89;
        if *self == 0 {
            return None;
        }
        // println!(
        //     "deb index is {}",
        //     (((*self ^ (*self).wrapping_sub(1)).wrapping_mul(DEBRUIJN_64)).wrapping_shr(58))
        //         as usize
        // );
        let index_of_lsb = INDEX_64_KIM[(((*self ^ (*self).wrapping_sub(1))
            .wrapping_mul(DEBRUIJN_64))
        .wrapping_shr(58)) as usize];

        // pop the lsb
        let ls1b_bb = (*self) & (*self).wrapping_neg();
        // println!("{:#066b}\n{:#066b}", *self, (*self).wrapping_neg());
        // println!("ls1b is this \n{:#066b}", ls1b_bb);

        *self ^= ls1b_bb;

        // println!("final bb: \n{:#066b}", *self);
        // println!();

        return Some(index_of_lsb.try_into().unwrap());
    }
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
        *self & o == EMPTY
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
    fn nort_one(&self) -> Self {
        *self << 8
    }
    fn sout_one(&self) -> Self {
        *self >> 8
    }
    fn east_one(&self) -> Self {
        (*self & NOTHFILE) << 1
    }
    fn no_ea_one(&self) -> Self {
        (*self & NOTHFILE) << 9
    }
    fn so_ea_one(&self) -> Self {
        (*self & NOTHFILE) >> 7
    }
    fn west_one(&self) -> Self {
        (*self & NOTAFILE) >> 1
    }
    fn so_we_one(&self) -> Self {
        (*self & NOTAFILE) >> 9
    }
    fn no_we_one(&self) -> Self {
        (*self & NOTAFILE) << 7
    }
}

trait RotateShiftOperations {
    fn rotate_left(&self, s: isize) -> Self;
    fn rotate_right(&self, s: isize) -> Self;
}

impl RotateShiftOperations for Bitboard {
    fn rotate_left(&self, s: isize) -> Self {
        (*self << s) | (*self >> (64 - s))
    }
    fn rotate_right(&self, s: isize) -> Self {
        (*self >> s) | (*self << (64 - s))
    }
}
