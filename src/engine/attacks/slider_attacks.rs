use self::bishop_attacks::{bishop_relevant_bits, BishopAttacks};
use self::rook_attacks::{rook_relevant_bits, RookAttacks};
use crate::engine::{
    bitboard::{Bitboard, EMPTY, LS1B, ONE},
    board::Piece,
    square::Square,
};
use rand::Rng;

pub mod bishop_attacks;
pub mod rook_attacks;

pub struct SliderAttacks {
    pub bishops: BishopAttacks,
    pub rooks: RookAttacks,
}

impl SliderAttacks {
    pub fn new() -> Self {
        let bishops = BishopAttacks::new();
        let rooks = RookAttacks::new();

        Self { bishops, rooks }
    }
    pub fn set_occupancy(
        index: usize,
        bits_mask_recieved: usize,
        mut attack_mask: Bitboard,
    ) -> Bitboard {
        let mut occupancy: Bitboard = EMPTY;

        for i in 0..bits_mask_recieved {
            // get LS1B index of attack_mask
            if let Some(square_index) = attack_mask.pop_lsb() {
                // make sure occupancy is on the board
                if (index & (1 << i)) != 0 {
                    // populate occupancy map
                    occupancy |= ONE << square_index;
                }
            }
        }
        // return
        occupancy
    }

    pub fn get_queen_attack(&self, square: Square, occupancy: Bitboard) -> Bitboard {
        self.bishops.get_bishop_attack(square, occupancy)
            | self.rooks.get_rook_attack(square, occupancy)
    }

    // ===== MAGIC NUMBERS =======

    pub fn get_random_bitboard_nums() -> u64 {
        let mut rng = rand::thread_rng();
        let n1 = (rng.gen::<u32>() & 0xFFFF) as u64;
        let n2 = (rng.gen::<u32>() & 0xFFFF) as u64;
        let n3 = (rng.gen::<u32>() & 0xFFFF) as u64;
        let n4 = (rng.gen::<u32>() & 0xFFFF) as u64;

        return n1 | (n2 << 16) | (n3 << 32) | (n4 << 48);
    }

    pub fn generate_magic_number() -> u64 {
        return Self::get_random_bitboard_nums()
            & Self::get_random_bitboard_nums()
            & Self::get_random_bitboard_nums();
    }

    pub fn find_magic_number(square: Square, relevant_bits: usize, piece_type: Piece) -> Bitboard {
        let mut occupancies = [EMPTY; 4096];
        let mut attacks = [EMPTY; 4096];
        let mut used_attacks = [EMPTY; 4096];
        let attack_mask: Bitboard = match piece_type {
            Piece::Bishop => BishopAttacks::mask_bishop_attack(square),
            Piece::Rook => RookAttacks::mask_rook_attack(square),
            _ => panic!("bro its gotta be rook or bishop LOLS"),
        };
        // print_bitboard(attack_mask);
        let occupancy_indicies = 1 << relevant_bits;
        // println!("occupancy_indicies is {}", occupancy_indicies);

        for index in 0..occupancy_indicies {
            // println!("index in occupancy indicies is {}", index);
            occupancies[index] = Self::set_occupancy(index, relevant_bits, attack_mask);

            // initialize the attacks
            attacks[index] = if piece_type == Piece::Bishop {
                BishopAttacks::bishop_attacks_otf(square, occupancies[index])
            } else {
                // eventually
                RookAttacks::rook_attacks_otf(square, occupancies[index])
            };

            // // initialize the attacks
            // attacks[index] = if piece_type == Piece::Bishop {
            //     BishopAttacks::get_bishop_attack(square, occupancies[index])
            // } else {
            //     // eventually
            //     // RookAttacks::get_rook_attack(square, occupancies[index])
            //     0
            // }
            // attacks[index] = BishopAttacks::get_bishop_attack(square, occupancies[index]);
        }

        // for i in 0..occupancy_indicies {
        //     println!("i: {} occupancies[]: {}", i, occupancies[i]);
        // }

        // test magic numbers
        for _ in 0..10000000 {
            // lol
            let magic_number: u64 = Self::generate_magic_number();
            // println!("testing this {:x?}", magic_number);

            // skip too small of magic numbers
            if (magic_number.wrapping_mul(attack_mask) & 0xFF00000000000000).count_bits() < 6 {
                // println!("number too small lol");
                continue;
            }

            used_attacks = [EMPTY; 4096];

            // println!("found magic_number: {:?}", magic_number);

            let mut index = 0;
            let mut fail = false;

            while fail == false && index < occupancy_indicies {
                // init magic index
                // println!("occupancies[index] is {}", occupancy_indicies);
                let magic_index = ((occupancies[index].wrapping_mul(magic_number))
                    >> (64 - relevant_bits)) as usize;

                // println!("index: {}; magic_index: {:064b}", index, magic_index);

                // println!(
                //     "used_attacks[magic_index] is {:?}",
                //     used_attacks[magic_index]
                // );
                if used_attacks[magic_index] == EMPTY {
                    used_attacks[magic_index] = attacks[index];
                    // println!(
                    //     "set used_attacks[{}] to attacks[{}] {}",
                    //     magic_index, index, attacks[index]
                    // );
                } else if used_attacks[magic_index] != attacks[index] {
                    // magic index doesnt work
                    // COLLISION!!!
                    // println!(
                    //     "have used_attacks[{}] {} but attacks[{}] {}",
                    //     magic_index, used_attacks[magic_index], index, attacks[index]
                    // );
                    fail = true;
                }

                index += 1;
            }

            if !fail {
                return magic_number;
            }
        }

        println!("lol magic number somehow failed.. uh oh");
        return EMPTY;
    }

    pub fn init_magic_testing() {
        println!("Bishops");
        for square_num in 0..64_usize {
            // init bishop magics
            let square: Square = square_num.try_into().unwrap();
            let magic =
                Self::find_magic_number(square, bishop_relevant_bits[square_num], Piece::Bishop);
            // TODO: Assignments
            println!("{:#x?}", magic);
        }

        println!("Rooks");
        for square_num in 0..64_usize {
            // init rook magics
            let square: Square = square_num.try_into().unwrap();
            let magic =
                Self::find_magic_number(square, rook_relevant_bits[square_num], Piece::Rook);
            // TODO: Assignments
            println!("{:#x?}", magic);
        }
    }

    pub fn populate(&mut self, piece_type: Piece) {
        for square_num in 0..64_usize {
            let square: Square = square_num.try_into().unwrap();
            self.bishops.bishop_masks[square_num] = BishopAttacks::mask_bishop_attack(square);
            self.rooks.rook_masks[square_num] = RookAttacks::mask_rook_attack(square);

            let attack_mask = match piece_type {
                Piece::Bishop => self.bishops.bishop_masks[square_num],
                Piece::Rook => self.rooks.rook_masks[square_num],
                _ => panic!("Shouldn't be anything other than rook or bishop."),
            };

            let relevant_bits_count = attack_mask.count_bits();
            let occupancy_indicies = ONE << relevant_bits_count;

            for index in 0..occupancy_indicies as usize {
                match piece_type {
                    Piece::Bishop => {
                        let occupancy =
                            Self::set_occupancy(index, relevant_bits_count, attack_mask);
                        let magic_index = occupancy.wrapping_mul(BISHOP_MAGICS[square_num])
                            >> (64 - bishop_relevant_bits[square_num]);
                        self.bishops.bishop_attacks[square_num][magic_index as usize] =
                            BishopAttacks::bishop_attacks_otf(square, occupancy);
                    }
                    Piece::Rook => {
                        let occupancy =
                            Self::set_occupancy(index, relevant_bits_count, attack_mask);
                        let magic_index = occupancy.wrapping_mul(ROOK_MAGICS[square_num])
                            >> (64 - rook_relevant_bits[square_num]);
                        self.rooks.rook_attacks[square_num][magic_index as usize] =
                            RookAttacks::rook_attacks_otf(square, occupancy);
                    }
                    _ => panic!("I don't even know how it got to this point."),
                }
            }
        }
    }
}

// if necessary
const BISHOP_MAGICS: [u64; 64] = [
    0x8022800410200,
    0x24210862a004010,
    0x808420c002300a2,
    0x820820404010014a,
    0x202121000000000,
    0x2014520000022,
    0xe9011031040240,
    0x4040d148041c2044,
    0x40840804a122,
    0x2050020841190200,
    0x22010810a4008800,
    0x80a002c4250,
    0x5000020211080010,
    0x28160802280009,
    0x221420111601103,
    0x88304044504800,
    0x41984410020a40,
    0x24080801080a20,
    0x44008208041100,
    0x8a8040082004001,
    0x66600a422010908,
    0x1000200808441,
    0x6bb0000444a2006,
    0x35000080482200,
    0x10106004041024,
    0x1044610844011401,
    0x4000920010040110,
    0x10040000401120,
    0x9001009004000,
    0x1010008088804,
    0x1090422081108,
    0x420100808400,
    0x2008208800840855,
    0x8220801020801,
    0x202080408020420,
    0x2008200900200900,
    0x21100400008020,
    0x808100100002080,
    0x6c01084880210420,
    0x502008204010040,
    0x4006182018100c01,
    0x44c240442008402,
    0x8002010041000804,
    0x8000362019000800,
    0x1020089100430400,
    0x4014112059000200,
    0x291020204000044,
    0x4001410102080504,
    0x102008460080250,
    0x308804101548,
    0xa000008400880104,
    0x28245c108480000,
    0x2002002540844,
    0x1801202802182048,
    0x1010821044428000,
    0x2020a2a0200a4,
    0x210c10090800,
    0x9048080401,
    0x1000800042180402,
    0xc32000000842400,
    0xa2a00808102410,
    0x1028000910010200,
    0x10080204880208,
    0x3730200200720420,
];

// if necessary
const ROOK_MAGICS: [u64; 64] = [
    0x2080002010400884,
    0x4140002000100244,
    0x980100081282000,
    0x280080050008044,
    0x2080040002800801,
    0x8500040001000208,
    0x400520485081004,
    0x1200010084004022,
    0x4000800890204000,
    0x320400050002004,
    0x201001100402004,
    0x8001801001280080,
    0x8405000800241101,
    0xc01000804010002,
    0x11000100244200,
    0x8401000200408100,
    0x40008000384081,
    0x130024020004000,
    0x110040200100,
    0x400808010000802,
    0x1000808004000800,
    0x80104402010,
    0x8040008020150,
    0x20000440081,
    0x9081a280024002,
    0x1000400040201002,
    0x8c0401100200109,
    0x100080080086,
    0x4000080080040080,
    0x22000200081004,
    0x8011000900120004,
    0x440842000100ac,
    0x2080004001402000,
    0x20400103002384,
    0x100082802000,
    0x2030004400400800,
    0x20040080800800,
    0x4000200808004,
    0xc00200081a000421,
    0x80004841a0000c3,
    0x80008040008021,
    0x200050014000,
    0x1400110020010040,
    0x81011004090020,
    0xc8001100090004,
    0xa11000400890002,
    0x2102020110040088,
    0x441040068820001,
    0x988208000400480,
    0x400851040042100,
    0x200010008080,
    0x290210408100100,
    0xb00080080040080,
    0x2020800400020080,
    0x5002821001080400,
    0x1204140106608a00,
    0x16080001100c3,
    0x104000210081,
    0x8089016000425089,
    0x882210108041001,
    0x4001000800304205,
    0x4001000400020801,
    0x181041020084,
    0x1000400411c2082,
];
