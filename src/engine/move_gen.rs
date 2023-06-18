use crate::{engine::r#move::Move, get_bit};

use super::{
    bitboard::{Bitboard, LS1B, ONE},
    board::{Board, Castle, Color, Piece},
    square::{Square, ASCII_SQUARE},
};

impl Board {
    pub fn generate_moves(&mut self) {
        self.move_list.count = 0;
        let (mut source, mut target): (i32, i32);
        let (mut bitboard, mut attacks): (Bitboard, Bitboard);

        for piece in (Piece::Pawn as usize)..=(Piece::King as usize) {
            bitboard = self.get_piece_of_color(piece.try_into().unwrap(), self.side);

            match self.side {
                // white pawns and castling,
                Color::White => {
                    if piece == (Piece::Pawn as usize) {
                        while bitboard > 0 {
                            source = bitboard.index_of_lsb().unwrap() as i32;
                            target = source + 8;

                            // valid target
                            if !(target > Square::h8 as i32)
                                && !(get_bit!(self.get_occupancies(), target))
                            {
                                //
                                if source >= Square::a7 as i32 && source <= Square::h7 as i32 {
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::White,
                                        Some(Piece::Queen),
                                        Some(Color::White),
                                        false,
                                        false,
                                        false,
                                        false,
                                    ));
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::White,
                                        Some(Piece::Rook),
                                        Some(Color::White),
                                        false,
                                        false,
                                        false,
                                        false,
                                    ));
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::White,
                                        Some(Piece::Bishop),
                                        Some(Color::White),
                                        false,
                                        false,
                                        false,
                                        false,
                                    ));
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::White,
                                        Some(Piece::Knight),
                                        Some(Color::White),
                                        false,
                                        false,
                                        false,
                                        false,
                                    ));
                                } else {
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::White,
                                        None,
                                        None,
                                        false,
                                        false,
                                        false,
                                        false,
                                    ));
                                    if (source >= Square::a2 as i32 && source <= Square::h2 as i32)
                                        && !(get_bit!(self.get_occupancies(), target + 8))
                                    {
                                        self.move_list.add_move(Move::new(
                                            (source as u64).try_into().unwrap(),
                                            ((target + 8) as u64).try_into().unwrap(),
                                            Piece::Pawn,
                                            Color::White,
                                            None,
                                            None,
                                            false,
                                            true,
                                            false,
                                            false,
                                        ));
                                    }
                                }
                            }

                            attacks = self.attack_tables.pawns.pawn_attacks[self.side as usize]
                                [source as usize]
                                & self.get_color(Color::Black);

                            // pawn captures
                            while attacks > 0 {
                                target = attacks.index_of_lsb().unwrap() as i32;

                                if source >= Square::a7 as i32 && source <= Square::h7 as i32 {
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::White,
                                        Some(Piece::Queen),
                                        Some(Color::White),
                                        true,
                                        false,
                                        false,
                                        false,
                                    ));
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::White,
                                        Some(Piece::Rook),
                                        Some(Color::White),
                                        true,
                                        false,
                                        false,
                                        false,
                                    ));
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::White,
                                        Some(Piece::Bishop),
                                        Some(Color::White),
                                        true,
                                        false,
                                        false,
                                        false,
                                    ));
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::White,
                                        Some(Piece::Knight),
                                        Some(Color::White),
                                        true,
                                        false,
                                        false,
                                        false,
                                    ));
                                } else {
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::White,
                                        None,
                                        None,
                                        true,
                                        false,
                                        false,
                                        false,
                                    ));
                                }

                                attacks.pop_lsb();
                            }

                            // en passant
                            if self.en_passant_sq.is_some() {
                                let en_passant_attacks = self.attack_tables.pawns.pawn_attacks
                                    [self.side as usize][source as usize]
                                    & (ONE << self.en_passant_sq.unwrap() as u64);

                                if en_passant_attacks > 0 {
                                    let target_en_passant =
                                        en_passant_attacks.index_of_lsb().unwrap();
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target_en_passant as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::White,
                                        None,
                                        None,
                                        true,
                                        false,
                                        true,
                                        false,
                                    ));
                                }
                            }

                            bitboard.pop_lsb();
                        }
                    }

                    // generate castling moves
                    if piece == (Piece::King as usize) {
                        // king side
                        if self.castle.can_castle(Castle::WhiteKing) {
                            if !get_bit!(self.get_occupancies(), Square::f1)
                                && !get_bit!(self.get_occupancies(), Square::g1)
                            {
                                if !self.is_square_attacked(Square::e1, Color::Black)
                                    && !self.is_square_attacked(Square::f1, Color::Black)
                                {
                                    self.move_list.add_move(Move::new(
                                        Square::e1,
                                        Square::g1,
                                        Piece::King,
                                        Color::White,
                                        None,
                                        None,
                                        false,
                                        false,
                                        false,
                                        true,
                                    ));
                                }
                            }
                        }

                        // queen side
                        if self.castle.can_castle(Castle::WhiteQueen) {
                            if !get_bit!(self.get_occupancies(), Square::d1)
                                && !get_bit!(self.get_occupancies(), Square::c1)
                                && !get_bit!(self.get_occupancies(), Square::b1)
                            {
                                if !self.is_square_attacked(Square::e1, Color::Black)
                                    && !self.is_square_attacked(Square::d1, Color::Black)
                                {
                                    self.move_list.add_move(Move::new(
                                        Square::e1,
                                        Square::c1,
                                        Piece::King,
                                        Color::White,
                                        None,
                                        None,
                                        false,
                                        false,
                                        false,
                                        true,
                                    ));
                                }
                            }
                        }
                    }
                }
                Color::Black => {
                    if piece == (Piece::Pawn as usize) {
                        while bitboard > 0 {
                            source = bitboard.index_of_lsb().unwrap() as i32;
                            target = source - 8;

                            // valid target
                            if !(target < Square::a1 as i32)
                                && !(get_bit!(self.get_occupancies(), target))
                            {
                                //
                                if source >= Square::a2 as i32 && source <= Square::h2 as i32 {
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::Black,
                                        Some(Piece::Queen),
                                        Some(Color::Black),
                                        false,
                                        false,
                                        false,
                                        false,
                                    ));
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::Black,
                                        Some(Piece::Rook),
                                        Some(Color::Black),
                                        false,
                                        false,
                                        false,
                                        false,
                                    ));
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::Black,
                                        Some(Piece::Bishop),
                                        Some(Color::Black),
                                        false,
                                        false,
                                        false,
                                        false,
                                    ));
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::Black,
                                        Some(Piece::Knight),
                                        Some(Color::Black),
                                        false,
                                        false,
                                        false,
                                        false,
                                    ));
                                } else {
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::Black,
                                        None,
                                        None,
                                        false,
                                        false,
                                        false,
                                        false,
                                    ));
                                    if (source >= Square::a7 as i32 && source <= Square::h7 as i32)
                                        && !(get_bit!(self.get_occupancies(), target - 8))
                                    {
                                        self.move_list.add_move(Move::new(
                                            (source as u64).try_into().unwrap(),
                                            ((target - 8) as u64).try_into().unwrap(),
                                            Piece::Pawn,
                                            Color::Black,
                                            None,
                                            None,
                                            false,
                                            true,
                                            false,
                                            false,
                                        ));
                                    }
                                }
                            }

                            attacks = self.attack_tables.pawns.pawn_attacks[self.side as usize]
                                [source as usize]
                                & self.get_color(Color::White);

                            // pawn captures
                            while attacks > 0 {
                                target = attacks.index_of_lsb().unwrap() as i32;

                                if source >= Square::a2 as i32 && source <= Square::h2 as i32 {
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::Black,
                                        Some(Piece::Queen),
                                        Some(Color::Black),
                                        true,
                                        false,
                                        false,
                                        false,
                                    ));
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::Black,
                                        Some(Piece::Rook),
                                        Some(Color::Black),
                                        true,
                                        false,
                                        false,
                                        false,
                                    ));
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::Black,
                                        Some(Piece::Bishop),
                                        Some(Color::Black),
                                        true,
                                        false,
                                        false,
                                        false,
                                    ));
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::Black,
                                        Some(Piece::Knight),
                                        Some(Color::Black),
                                        true,
                                        false,
                                        false,
                                        false,
                                    ));
                                } else {
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::Black,
                                        None,
                                        None,
                                        true,
                                        false,
                                        false,
                                        false,
                                    ));
                                }

                                attacks.pop_lsb();
                            }

                            // en passant
                            if self.en_passant_sq.is_some() {
                                let en_passant_attacks = self.attack_tables.pawns.pawn_attacks
                                    [self.side as usize][source as usize]
                                    & (ONE << self.en_passant_sq.unwrap() as u64);

                                if en_passant_attacks > 0 {
                                    let target_en_passant =
                                        en_passant_attacks.index_of_lsb().unwrap();
                                    self.move_list.add_move(Move::new(
                                        (source as u64).try_into().unwrap(),
                                        (target_en_passant as u64).try_into().unwrap(),
                                        Piece::Pawn,
                                        Color::Black,
                                        None,
                                        None,
                                        true,
                                        false,
                                        true,
                                        false,
                                    ));
                                }
                            }
                            bitboard.pop_lsb();
                        }
                    }

                    // generate castling moves
                    if piece == (Piece::King as usize) {
                        // king side
                        if self.castle.can_castle(Castle::BlackKing) {
                            if !get_bit!(self.get_occupancies(), Square::f8)
                                && !get_bit!(self.get_occupancies(), Square::g8)
                            {
                                if !self.is_square_attacked(Square::e8, Color::White)
                                    && !self.is_square_attacked(Square::f8, Color::White)
                                {
                                    self.move_list.add_move(Move::new(
                                        Square::e8,
                                        Square::g8,
                                        Piece::King,
                                        Color::Black,
                                        None,
                                        None,
                                        false,
                                        false,
                                        false,
                                        true,
                                    ));
                                }
                            }
                        }

                        // queen side
                        if self.castle.can_castle(Castle::BlackQueen) {
                            if !get_bit!(self.get_occupancies(), Square::d8)
                                && !get_bit!(self.get_occupancies(), Square::c8)
                                && !get_bit!(self.get_occupancies(), Square::b8)
                            {
                                if !self.is_square_attacked(Square::e8, Color::White)
                                    && !self.is_square_attacked(Square::d8, Color::White)
                                {
                                    self.move_list.add_move(Move::new(
                                        Square::e8,
                                        Square::c8,
                                        Piece::King,
                                        Color::Black,
                                        None,
                                        None,
                                        false,
                                        false,
                                        false,
                                        true,
                                    ));
                                }
                            }
                        }
                    }
                } // black pawns and castling,
            }

            // knight
            if piece == (Piece::Knight as usize) {
                while bitboard > 0 {
                    source = bitboard.index_of_lsb().unwrap() as i32;
                    attacks = self.attack_tables.knights.knight_attacks[source as usize]
                        & !self.get_color(self.side);

                    while attacks > 0 {
                        target = attacks.index_of_lsb().unwrap() as i32;

                        // quiet move
                        if !get_bit!(self.get_color(!self.side), target) {
                            self.move_list.add_move(Move::new(
                                (source as u64).try_into().unwrap(),
                                (target as u64).try_into().unwrap(),
                                Piece::Knight,
                                self.side,
                                None,
                                None,
                                false,
                                false,
                                false,
                                false,
                            ));
                        } else {
                            // capture move
                            self.move_list.add_move(Move::new(
                                (source as u64).try_into().unwrap(),
                                (target as u64).try_into().unwrap(),
                                Piece::Knight,
                                self.side,
                                None,
                                None,
                                true,
                                false,
                                false,
                                false,
                            ));
                        }

                        attacks.pop_lsb();
                    }
                    bitboard.pop_lsb();
                }
            }

            // bishop
            if piece == (Piece::Bishop as usize) {
                while bitboard > 0 {
                    source = bitboard.index_of_lsb().unwrap() as i32;
                    attacks = self.attack_tables.sliders.bishops.get_bishop_attack(
                        (source as u64).try_into().unwrap(),
                        self.get_occupancies(),
                    ) & !self.get_color(self.side);

                    while attacks > 0 {
                        target = attacks.index_of_lsb().unwrap() as i32;

                        // quiet move
                        if !get_bit!(self.get_color(!self.side), target) {
                            self.move_list.add_move(Move::new(
                                (source as u64).try_into().unwrap(),
                                (target as u64).try_into().unwrap(),
                                Piece::Bishop,
                                self.side,
                                None,
                                None,
                                false,
                                false,
                                false,
                                false,
                            ));
                        } else {
                            // capture move
                            self.move_list.add_move(Move::new(
                                (source as u64).try_into().unwrap(),
                                (target as u64).try_into().unwrap(),
                                Piece::Bishop,
                                self.side,
                                None,
                                None,
                                true,
                                false,
                                false,
                                false,
                            ));
                        }

                        attacks.pop_lsb();
                    }
                    bitboard.pop_lsb();
                }
            }

            // rook
            if piece == (Piece::Rook as usize) {
                while bitboard > 0 {
                    source = bitboard.index_of_lsb().unwrap() as i32;
                    attacks = self.attack_tables.sliders.rooks.get_rook_attack(
                        (source as u64).try_into().unwrap(),
                        self.get_occupancies(),
                    ) & !self.get_color(self.side);

                    while attacks > 0 {
                        target = attacks.index_of_lsb().unwrap() as i32;

                        // quiet move
                        if !get_bit!(self.get_color(!self.side), target) {
                            self.move_list.add_move(Move::new(
                                (source as u64).try_into().unwrap(),
                                (target as u64).try_into().unwrap(),
                                Piece::Rook,
                                self.side,
                                None,
                                None,
                                false,
                                false,
                                false,
                                false,
                            ));
                        } else {
                            // capture move
                            self.move_list.add_move(Move::new(
                                (source as u64).try_into().unwrap(),
                                (target as u64).try_into().unwrap(),
                                Piece::Rook,
                                self.side,
                                None,
                                None,
                                true,
                                false,
                                false,
                                false,
                            ));
                        }

                        attacks.pop_lsb();
                    }
                    bitboard.pop_lsb();
                }
            }

            // queen
            if piece == (Piece::Queen as usize) {
                while bitboard > 0 {
                    source = bitboard.index_of_lsb().unwrap() as i32;
                    attacks = self.attack_tables.sliders.get_queen_attack(
                        (source as u64).try_into().unwrap(),
                        self.get_occupancies(),
                    ) & !self.get_color(self.side);

                    while attacks > 0 {
                        target = attacks.index_of_lsb().unwrap() as i32;

                        // quiet move
                        if !get_bit!(self.get_color(!self.side), target) {
                            self.move_list.add_move(Move::new(
                                (source as u64).try_into().unwrap(),
                                (target as u64).try_into().unwrap(),
                                Piece::Queen,
                                self.side,
                                None,
                                None,
                                false,
                                false,
                                false,
                                false,
                            ));
                        } else {
                            // capture move
                            self.move_list.add_move(Move::new(
                                (source as u64).try_into().unwrap(),
                                (target as u64).try_into().unwrap(),
                                Piece::Queen,
                                self.side,
                                None,
                                None,
                                true,
                                false,
                                false,
                                false,
                            ));
                        }

                        attacks.pop_lsb();
                    }
                    bitboard.pop_lsb();
                }
            }

            // king
            if piece == (Piece::King as usize) {
                while bitboard > 0 {
                    source = bitboard.index_of_lsb().unwrap() as i32;
                    attacks = self.attack_tables.kings.king_attacks[source as usize]
                        & !self.get_color(self.side);

                    while attacks > 0 {
                        target = attacks.index_of_lsb().unwrap() as i32;

                        // quiet move
                        if !get_bit!(self.get_color(!self.side), target) {
                            self.move_list.add_move(Move::new(
                                (source as u64).try_into().unwrap(),
                                (target as u64).try_into().unwrap(),
                                Piece::King,
                                self.side,
                                None,
                                None,
                                false,
                                false,
                                false,
                                false,
                            ));
                        } else {
                            // capture move
                            self.move_list.add_move(Move::new(
                                (source as u64).try_into().unwrap(),
                                (target as u64).try_into().unwrap(),
                                Piece::King,
                                self.side,
                                None,
                                None,
                                true,
                                false,
                                false,
                                false,
                            ));
                        }

                        attacks.pop_lsb();
                    }
                    bitboard.pop_lsb();
                }
            }
        }
    }
}
