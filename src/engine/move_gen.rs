use crate::get_bit;

use super::{square::{Square, ASCII_SQUARE}, bitboard::{Bitboard, LS1B, ONE}, board::{Board, Piece, Color}};

impl Board {
    pub fn generate_moves(&self) {
        let (mut source, mut target): (i32, i32);
        let (mut bitboard, mut attacks): (Bitboard, Bitboard);

        for piece in (Piece::Pawn as usize)..(Piece::King as usize) {
            bitboard = self.boards[piece];

            match self.side {
                // white pawns and castling,
                Color::White => {
                    bitboard &= self.boards[Color::White as usize];
                    if piece == (Piece::Pawn as usize) {
                        while bitboard > 0 {
                            source = bitboard.index_of_lsb().unwrap() as i32;
                            target = source + 8;

                            // valid target
                            if !(target > Square::h8 as i32) && 
                            !(get_bit!(self.get_occupancies(), target)) {
                                // 
                                if source >= Square::a7 as i32 && source <= Square::h7 as i32 {
                                    println!("pawn promotion: {}{}q", ASCII_SQUARE[source as usize], ASCII_SQUARE[target as usize]);
                                    println!("pawn promotion: {}{}r", ASCII_SQUARE[source as usize], ASCII_SQUARE[target as usize]);
                                    println!("pawn promotion: {}{}b", ASCII_SQUARE[source as usize], ASCII_SQUARE[target as usize]);
                                    println!("pawn promotion: {}{}n", ASCII_SQUARE[source as usize], ASCII_SQUARE[target as usize]);
                                } else {
                                    println!("pawn push: {}{}", ASCII_SQUARE[source as usize], ASCII_SQUARE[target as usize]);
                                    if (source >= Square::a2 as i32 && source <= Square::h2 as i32) &&
                                    !(get_bit!(self.get_occupancies(), target + 8)){
                                        println!("double pawn push: {}{}", ASCII_SQUARE[source as usize], ASCII_SQUARE[(target + 8) as usize])
                                    }
                                }
                            }

                            attacks = self.attack_tables.pawns.pawn_attacks[self.side as usize][source as usize] & self.get_color(Color::Black);

                            // pawn captures
                            while attacks > 0 {
                                target = attacks.index_of_lsb().unwrap() as i32;

                                if source >= Square::a7 as i32 && source <= Square::h7 as i32 {
                                    println!("pawn capture promotion: {}{}q", ASCII_SQUARE[source as usize], ASCII_SQUARE[target as usize]);
                                    println!("pawn capture promotion: {}{}r", ASCII_SQUARE[source as usize], ASCII_SQUARE[target as usize]);
                                    println!("pawn capture promotion: {}{}b", ASCII_SQUARE[source as usize], ASCII_SQUARE[target as usize]);
                                    println!("pawn capture promotion: {}{}n", ASCII_SQUARE[source as usize], ASCII_SQUARE[target as usize]);
                                } else {
                                    println!("pawn capture: {}{}", ASCII_SQUARE[source as usize], ASCII_SQUARE[target as usize]);
                                }

                                attacks.pop_lsb();
                            }

                            // en passant
                            if self.en_passant_sq.is_some() {
                                let en_passant_attacks = self.attack_tables.pawns.pawn_attacks[self.side as usize][source as usize] & (ONE << self.en_passant_sq.unwrap() as u64);

                                if en_passant_attacks > 0 {
                                    let target_en_passant = en_passant_attacks.index_of_lsb().unwrap();
                                    println!("pawn en passant capture: {}{}", ASCII_SQUARE[source as usize], ASCII_SQUARE[target_en_passant as usize])
                                }
                            }

                            bitboard.pop_lsb();
                        }
                    }
                } 
                Color::Black => {
                    bitboard &= self.boards[Color::Black as usize];
                    if piece == (Piece::Pawn as usize) {
                        while bitboard > 0 {
                            source = bitboard.index_of_lsb().unwrap() as i32;
                            target = source - 8;

                            // valid target
                            if !(target < Square::a1 as i32) && 
                            !(get_bit!(self.get_occupancies(), target)) {
                                // 
                                if source >= Square::a2 as i32 && source <= Square::h2 as i32 {
                                    println!("pawn promotion: {}{}q", ASCII_SQUARE[source as usize], ASCII_SQUARE[target as usize]);
                                    println!("pawn promotion: {}{}r", ASCII_SQUARE[source as usize], ASCII_SQUARE[target as usize]);
                                    println!("pawn promotion: {}{}b", ASCII_SQUARE[source as usize], ASCII_SQUARE[target as usize]);
                                    println!("pawn promotion: {}{}n", ASCII_SQUARE[source as usize], ASCII_SQUARE[target as usize]);
                                } else {
                                    println!("pawn push: {}{}", ASCII_SQUARE[source as usize], ASCII_SQUARE[target as usize]);
                                    if (source >= Square::a7 as i32 && source <= Square::h7 as i32) &&
                                    !(get_bit!(self.get_occupancies(), target - 8)){
                                        println!("double pawn push: {}{}", ASCII_SQUARE[source as usize], ASCII_SQUARE[(target - 8) as usize])
                                    }
                                }
                            }

                            attacks = self.attack_tables.pawns.pawn_attacks[self.side as usize][source as usize] & self.get_color(Color::White);

                            // pawn captures
                            while attacks > 0 {
                                target = attacks.index_of_lsb().unwrap() as i32;

                                if source >= Square::a2 as i32 && source <= Square::h2 as i32 {
                                    println!("pawn capture promotion: {}{}q", ASCII_SQUARE[source as usize], ASCII_SQUARE[target as usize]);
                                    println!("pawn capture promotion: {}{}r", ASCII_SQUARE[source as usize], ASCII_SQUARE[target as usize]);
                                    println!("pawn capture promotion: {}{}b", ASCII_SQUARE[source as usize], ASCII_SQUARE[target as usize]);
                                    println!("pawn capture promotion: {}{}n", ASCII_SQUARE[source as usize], ASCII_SQUARE[target as usize]);
                                } else {
                                    println!("pawn capture: {}{}", ASCII_SQUARE[source as usize], ASCII_SQUARE[target as usize]);
                                }

                                attacks.pop_lsb();
                            }

                            // en passant
                            if self.en_passant_sq.is_some() {
                                let en_passant_attacks = self.attack_tables.pawns.pawn_attacks[self.side as usize][source as usize] & (ONE << self.en_passant_sq.unwrap() as u64);

                                if en_passant_attacks > 0 {
                                    let target_en_passant = en_passant_attacks.index_of_lsb().unwrap();
                                    println!("pawn en passant capture: {}{}", ASCII_SQUARE[source as usize], ASCII_SQUARE[target_en_passant as usize])
                                }
                            }
                            bitboard.pop_lsb();
                        }
                    }
                } // black pawns and castling,
            }

            // knight
            
            // bishop
            
            // rook
            
            // queen
            
            // king
        }
    }
}
