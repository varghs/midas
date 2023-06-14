use self::bishop_attacks::BishopAttacks;
use self::king_attacks::KingAttacks;
use self::knight_attacks::KnightAttacks;
use self::pawn_attacks::PawnAttacks;

use super::bitboard::{Bitboard, EMPTY, LS1B, ONE};
use super::board::Piece;
use super::square::Square;

pub mod bishop_attacks;
pub mod king_attacks;
pub mod knight_attacks;
pub mod pawn_attacks;

// bishop relevant occupancy bit count for every square on board
#[rustfmt::skip]
const bishop_relevant_bits: [usize; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6,
];

// rook relevant occupancy bit count for every square on board
#[rustfmt::skip]
const rook_relevant_bits: [usize; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 
    12, 11, 11, 11, 11, 11, 11, 12,
];

struct AttackTables {
    pawns: PawnAttacks,
    knights: KnightAttacks,
    kings: KingAttacks,
    bishops: BishopAttacks,
}

impl AttackTables {
    // for every attack_table calls populate method
    fn populate(&mut self) {
        todo!();
    }

    fn populate_leapers_attacks(&mut self) {
        self.pawns.populate();
        self.knights.populate();
        self.kings.populate();
    }
}

pub fn set_occupancy(
    index: usize,
    bits_mask_recieved: usize,
    mut attack_mask: Bitboard,
) -> Bitboard {
    let mut occupancy: Bitboard = EMPTY;

    let bits_in_mask = Bitboard::count_bits(attack_mask);
    // println!(
    //     "bits counted: {:4?} bits recieved: {:4?}",
    //     bits_in_mask, bits_mask_recieved
    // );
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

use rand::Rng;
pub fn get_random_bitboard_nums() -> u64 {
    let mut rng = rand::thread_rng();
    let n1 = (rng.gen::<u32>() & 0xFFFF) as u64;
    let n2 = (rng.gen::<u32>() & 0xFFFF) as u64;
    let n3 = (rng.gen::<u32>() & 0xFFFF) as u64;
    let n4 = (rng.gen::<u32>() & 0xFFFF) as u64;

    return n1 | (n2 << 16) | (n3 << 32) | (n4 << 48);
}

pub fn generate_magic_number() -> u64 {
    return get_random_bitboard_nums() & get_random_bitboard_nums() & get_random_bitboard_nums();
}

pub fn find_magic_number(square: Square, relevant_bits: usize, piece_type: Piece) -> Bitboard {
    let mut occupancies = [EMPTY; 4096];
    let mut attacks = [EMPTY; 4096];
    let mut used_attacks = [EMPTY; 4096];
    let attack_mask: Bitboard = match piece_type {
        Piece::Bishop => BishopAttacks::mask_bishop_attacks(square),
        // Piece::Rook => {
        //     RookAttacks::mask_rook_attacks(square)
        // }
        _ => panic!("bro its gotta be rook or bishop LOLS"),
    };
    let occupancy_indicies = 1 << relevant_bits;

    for index in 0..occupancy_indicies {
        occupancies[index] = set_occupancy(index, relevant_bits, attack_mask);

        // // initialize the attacks
        // attacks[index] = if piece_type == Piece::Bishop {
        //     BishopAttacks::get_bishop_attack(square, occupancies[index])
        // } else {
        //     // eventually
        //     // RookAttacks::get_rook_attack(square, occupancies[index])
        //     0
        // }
        attacks[index] = BishopAttacks::get_bishop_attack(square, occupancies[index]);
    }

    // test magic numbers
    for i in 0..1000000 {
        // lol
        let magic_number: u64 = generate_magic_number();
        // println!("testing this {:x?}", magic_number);

        // skip too small of magic numbers
        if Bitboard::count_bits(magic_number.wrapping_mul(attack_mask) & 0xFF00000000000000) < 6 {
            // println!("number too small lol");
            continue;
        }

        println!("found magic_number: {:?}", magic_number);

        let mut index = 0;
        let mut fail = false;

        while fail == false && index < occupancy_indicies {
            // init magic index
            let magic_index =
                ((occupancies[index].wrapping_mul(magic_number)) >> (64 - relevant_bits)) as usize;

            println!("magic_index: {:064b}", magic_index);

            if used_attacks[magic_index] == EMPTY {
                used_attacks[magic_index] = attacks[index];
            } else if used_attacks[magic_index] != attacks[index] {
                // magic index doesnt work
                // COLLISION!!!
                // println!(
                //     "we failed...found this \n{:?}.. but have this \n{:?}\n\n",
                //     used_attacks[magic_index], attacks[index]
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
    for square_num in 0..64_usize {
        // init bishop magics
        let square: Square = square_num.try_into().unwrap();
        let magic = find_magic_number(square, bishop_relevant_bits[square_num], Piece::Bishop);

        println!("{:x?}", magic);
    }
}
