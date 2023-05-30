use std::fmt::{self, Display};

macro_rules! get_bit {
    ($bitboard:expr, $square:expr) => {
        $bitboard & ((1 as u64) << $square)
    };
}

pub type Bitboard = u64;

pub fn print_bitboard(b: Bitboard) {
    for rank in (0..=7).rev() {
        for file in 0..8 {
            let square = rank * 8 + file;
            print!("{} ", match get_bit!(b, square) {
                0 => 0,
                _ => 1,
            });
        }
        print!("\n");
    }
}

const WIDTH: u8 = 8;
const HEIGHT: u8 = 8;

enum Piece {
    Pawn = 2,
    Rook = 3,
    Knight = 4,
    Bishop = 5,
    Queen = 6,
    King = 7,
}

enum Color {
    White = 0,
    Black = 1,
}

pub struct Board {
    boards: [Bitboard; 8],
}

impl Board {
    fn new() -> Self {
        // let mut arr: [Bitboard; 8] = [];
        // let mut b = Board { boards: arr };
        todo!()
    }

    fn get_piece(&self, p: Piece) -> Bitboard {
        self.boards[p as usize]
    }

    fn get_color(&self, c: Color) -> Bitboard {
        self.boards[c as usize]
    }

    fn get_piece_of_color(&self, p: Piece, c: Color) -> Bitboard {
        self.get_piece(p) & self.get_color(c)
    }
}


// TODO!
/*
impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        let mut c: u64 = 1;
        while c != 0 {
            let mut s = String::new();
            let mut filled = false;
            for board in &self.boards[2..] {
                if board & c > 0 {
                    s += "1";
                    filled = true;
                    break;
                }
            }

            if !filled {
                s += "0";
            }

            output = s + &output;
            c = c << 1;
            if c % 2_u64.pow(8) == 0 {
                output += "\n";
            }
        }
        write!(f, "{}", output)
    }
}
*/
