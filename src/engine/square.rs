use super::bitboard::Bitboard;
use std::{ops::Shl, fmt::Display};

#[allow(non_camel_case_types)]
#[rustfmt::skip]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Square {
  a1, b1, c1, d1, e1, f1, g1, h1,
  a2, b2, c2, d2, e2, f2, g2, h2,
  a3, b3, c3, d3, e3, f3, g3, h3,
  a4, b4, c4, d4, e4, f4, g4, h4,
  a5, b5, c5, d5, e5, f5, g5, h5,
  a6, b6, c6, d6, e6, f6, g6, h6,
  a7, b7, c7, d7, e7, f7, g7, h7,
  a8, b8, c8, d8, e8, f8, g8, h8,
}

impl Shl<Square> for Bitboard {
    type Output = Bitboard;
    fn shl(self, rhs: Square) -> Self::Output {
        self << rhs as u64
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        match self {
            Square::a1 => { output += "a1"; },
            Square::b1 => { output += "b1"; },
            Square::c1 => { output += "c1"; },
            Square::d1 => { output += "d1"; },
            Square::e1 => { output += "e1"; },
            Square::f1 => { output += "f1"; },
            Square::g1 => { output += "g1"; },
            Square::h1 => { output += "h1"; },
            Square::a2 => { output += "a2"; },
            Square::b2 => { output += "b2"; },
            Square::c2 => { output += "c2"; },
            Square::d2 => { output += "d2"; },
            Square::e2 => { output += "e2"; },
            Square::f2 => { output += "f2"; },
            Square::g2 => { output += "g2"; },
            Square::h2 => { output += "h2"; },
            Square::a3 => { output += "a3"; },
            Square::b3 => { output += "b3"; },
            Square::c3 => { output += "c3"; },
            Square::d3 => { output += "d3"; },
            Square::e3 => { output += "e3"; },
            Square::f3 => { output += "f3"; },
            Square::g3 => { output += "g3"; },
            Square::h3 => { output += "h3"; },
            Square::a4 => { output += "a4"; },
            Square::b4 => { output += "b4"; },
            Square::c4 => { output += "c4"; },
            Square::d4 => { output += "d4"; },
            Square::e4 => { output += "e4"; },
            Square::f4 => { output += "f4"; },
            Square::g4 => { output += "g4"; },
            Square::h4 => { output += "h4"; },
            Square::a5 => { output += "a5"; },
            Square::b5 => { output += "b5"; },
            Square::c5 => { output += "c5"; },
            Square::d5 => { output += "d5"; },
            Square::e5 => { output += "e5"; },
            Square::f5 => { output += "f5"; },
            Square::g5 => { output += "g5"; },
            Square::h5 => { output += "h5"; },
            Square::a6 => { output += "a6"; },
            Square::b6 => { output += "b6"; },
            Square::c6 => { output += "c6"; },
            Square::d6 => { output += "d6"; },
            Square::e6 => { output += "e6"; },
            Square::f6 => { output += "f6"; },
            Square::g6 => { output += "g6"; },
            Square::h6 => { output += "h6"; },
            Square::a7 => { output += "a7"; },
            Square::b7 => { output += "b7"; },
            Square::c7 => { output += "c7"; },
            Square::d7 => { output += "d7"; },
            Square::e7 => { output += "e7"; },
            Square::f7 => { output += "f7"; },
            Square::g7 => { output += "g7"; },
            Square::h7 => { output += "h7"; },
            Square::a8 => { output += "a8"; },
            Square::b8 => { output += "b8"; },
            Square::c8 => { output += "c8"; },
            Square::d8 => { output += "d8"; },
            Square::e8 => { output += "e8"; },
            Square::f8 => { output += "f8"; },
            Square::g8 => { output += "g8"; },
            Square::h8 => { output += "h8"; },
        }
        write!(f, "{}", output)
    }
}

impl TryFrom<u64> for Square {
    type Error = String;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Square::a1),
            1 => Ok(Square::b1),
            2 => Ok(Square::c1),
            3 => Ok(Square::d1),
            4 => Ok(Square::e1),
            5 => Ok(Square::f1),
            6 => Ok(Square::g1),
            7 => Ok(Square::h1),
            8 => Ok(Square::a2),
            9 => Ok(Square::b2),
            10 => Ok(Square::c2),
            11 => Ok(Square::d2),
            12 => Ok(Square::e2),
            13 => Ok(Square::f2),
            14 => Ok(Square::g2),
            15 => Ok(Square::h2),
            16 => Ok(Square::a3),
            17 => Ok(Square::b3),
            18 => Ok(Square::c3),
            19 => Ok(Square::d3),
            20 => Ok(Square::e3),
            21 => Ok(Square::f3),
            22 => Ok(Square::g3),
            23 => Ok(Square::h3),
            24 => Ok(Square::a4),
            25 => Ok(Square::b4),
            26 => Ok(Square::c4),
            27 => Ok(Square::d4),
            28 => Ok(Square::e4),
            29 => Ok(Square::f4),
            30 => Ok(Square::g4),
            31 => Ok(Square::h4),
            32 => Ok(Square::a5),
            33 => Ok(Square::b5),
            34 => Ok(Square::c5),
            35 => Ok(Square::d5),
            36 => Ok(Square::e5),
            37 => Ok(Square::f5),
            38 => Ok(Square::g5),
            39 => Ok(Square::h5),
            40 => Ok(Square::a6),
            41 => Ok(Square::b6),
            42 => Ok(Square::c6),
            43 => Ok(Square::d6),
            44 => Ok(Square::e6),
            45 => Ok(Square::f6),
            46 => Ok(Square::g6),
            47 => Ok(Square::h6),
            48 => Ok(Square::a7),
            49 => Ok(Square::b7),
            50 => Ok(Square::c7),
            51 => Ok(Square::d7),
            52 => Ok(Square::e7),
            53 => Ok(Square::f7),
            54 => Ok(Square::g7),
            55 => Ok(Square::h7),
            56 => Ok(Square::a8),
            57 => Ok(Square::b8),
            58 => Ok(Square::c8),
            59 => Ok(Square::d8),
            60 => Ok(Square::e8),
            61 => Ok(Square::f8),
            62 => Ok(Square::g8),
            63 => Ok(Square::h8),
            _ => Err("Invalid conversion".to_string()),
        }
    }
}

impl TryFrom<u16> for Square {
    type Error = String;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Square::a1),
            1 => Ok(Square::b1),
            2 => Ok(Square::c1),
            3 => Ok(Square::d1),
            4 => Ok(Square::e1),
            5 => Ok(Square::f1),
            6 => Ok(Square::g1),
            7 => Ok(Square::h1),
            8 => Ok(Square::a2),
            9 => Ok(Square::b2),
            10 => Ok(Square::c2),
            11 => Ok(Square::d2),
            12 => Ok(Square::e2),
            13 => Ok(Square::f2),
            14 => Ok(Square::g2),
            15 => Ok(Square::h2),
            16 => Ok(Square::a3),
            17 => Ok(Square::b3),
            18 => Ok(Square::c3),
            19 => Ok(Square::d3),
            20 => Ok(Square::e3),
            21 => Ok(Square::f3),
            22 => Ok(Square::g3),
            23 => Ok(Square::h3),
            24 => Ok(Square::a4),
            25 => Ok(Square::b4),
            26 => Ok(Square::c4),
            27 => Ok(Square::d4),
            28 => Ok(Square::e4),
            29 => Ok(Square::f4),
            30 => Ok(Square::g4),
            31 => Ok(Square::h4),
            32 => Ok(Square::a5),
            33 => Ok(Square::b5),
            34 => Ok(Square::c5),
            35 => Ok(Square::d5),
            36 => Ok(Square::e5),
            37 => Ok(Square::f5),
            38 => Ok(Square::g5),
            39 => Ok(Square::h5),
            40 => Ok(Square::a6),
            41 => Ok(Square::b6),
            42 => Ok(Square::c6),
            43 => Ok(Square::d6),
            44 => Ok(Square::e6),
            45 => Ok(Square::f6),
            46 => Ok(Square::g6),
            47 => Ok(Square::h6),
            48 => Ok(Square::a7),
            49 => Ok(Square::b7),
            50 => Ok(Square::c7),
            51 => Ok(Square::d7),
            52 => Ok(Square::e7),
            53 => Ok(Square::f7),
            54 => Ok(Square::g7),
            55 => Ok(Square::h7),
            56 => Ok(Square::a8),
            57 => Ok(Square::b8),
            58 => Ok(Square::c8),
            59 => Ok(Square::d8),
            60 => Ok(Square::e8),
            61 => Ok(Square::f8),
            62 => Ok(Square::g8),
            63 => Ok(Square::h8),
            _ => Err("Invalid conversion".to_string()),
        }
    }
}

impl TryFrom<usize> for Square {
    type Error = String;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Square::a1),
            1 => Ok(Square::b1),
            2 => Ok(Square::c1),
            3 => Ok(Square::d1),
            4 => Ok(Square::e1),
            5 => Ok(Square::f1),
            6 => Ok(Square::g1),
            7 => Ok(Square::h1),
            8 => Ok(Square::a2),
            9 => Ok(Square::b2),
            10 => Ok(Square::c2),
            11 => Ok(Square::d2),
            12 => Ok(Square::e2),
            13 => Ok(Square::f2),
            14 => Ok(Square::g2),
            15 => Ok(Square::h2),
            16 => Ok(Square::a3),
            17 => Ok(Square::b3),
            18 => Ok(Square::c3),
            19 => Ok(Square::d3),
            20 => Ok(Square::e3),
            21 => Ok(Square::f3),
            22 => Ok(Square::g3),
            23 => Ok(Square::h3),
            24 => Ok(Square::a4),
            25 => Ok(Square::b4),
            26 => Ok(Square::c4),
            27 => Ok(Square::d4),
            28 => Ok(Square::e4),
            29 => Ok(Square::f4),
            30 => Ok(Square::g4),
            31 => Ok(Square::h4),
            32 => Ok(Square::a5),
            33 => Ok(Square::b5),
            34 => Ok(Square::c5),
            35 => Ok(Square::d5),
            36 => Ok(Square::e5),
            37 => Ok(Square::f5),
            38 => Ok(Square::g5),
            39 => Ok(Square::h5),
            40 => Ok(Square::a6),
            41 => Ok(Square::b6),
            42 => Ok(Square::c6),
            43 => Ok(Square::d6),
            44 => Ok(Square::e6),
            45 => Ok(Square::f6),
            46 => Ok(Square::g6),
            47 => Ok(Square::h6),
            48 => Ok(Square::a7),
            49 => Ok(Square::b7),
            50 => Ok(Square::c7),
            51 => Ok(Square::d7),
            52 => Ok(Square::e7),
            53 => Ok(Square::f7),
            54 => Ok(Square::g7),
            55 => Ok(Square::h7),
            56 => Ok(Square::a8),
            57 => Ok(Square::b8),
            58 => Ok(Square::c8),
            59 => Ok(Square::d8),
            60 => Ok(Square::e8),
            61 => Ok(Square::f8),
            62 => Ok(Square::g8),
            63 => Ok(Square::h8),
            _ => Err("Invalid conversion".to_string()),
        }
    }
}
