use midas::{engine::{
    bitboard::*,
    board::{Square, Color},
    attacks::pawn::PawnAttacks,
}, set_bit};

#[test]
fn pawn_attacks_reg_white() {
    let mut bitboard: Bitboard = EMPTY;
    set_bit!(bitboard, Square::e4);
    set_bit!(bitboard, Square::c4);
    assert_eq!(PawnAttacks::mask_pawn_attacks(Square::d3, Color::White), bitboard);
}

#[test]
fn pawn_attacks_reg_black() {
    let mut bitboard: Bitboard = EMPTY;
    set_bit!(bitboard, Square::e5);
    set_bit!(bitboard, Square::c5);
    assert_eq!(PawnAttacks::mask_pawn_attacks(Square::d6, Color::Black), bitboard);
}

#[test]
fn pawn_attacks_h_white() {
    let mut bitboard: Bitboard = EMPTY;
    set_bit!(bitboard, Square::g7);
    assert_eq!(PawnAttacks::mask_pawn_attacks(Square::h6, Color::White), bitboard);
}

#[test]
fn pawn_attacks_h_black() {
    let mut bitboard: Bitboard = EMPTY;
    set_bit!(bitboard, Square::g4);
    assert_eq!(PawnAttacks::mask_pawn_attacks(Square::h5, Color::Black), bitboard);
}

#[test]
fn pawn_attacks_a_waite() {
    let mut bitboard: Bitboard = EMPTY;
    set_bit!(bitboard, Square::b4);
    assert_eq!(PawnAttacks::mask_pawn_attacks(Square::a3, Color::White), bitboard);
}

#[test]
fn pawn_attacks_a_black() {
    let mut bitboard: Bitboard = EMPTY;
    set_bit!(bitboard, Square::b6);
    assert_eq!(PawnAttacks::mask_pawn_attacks(Square::a7, Color::Black), bitboard);
}

#[test]
fn pawn_attacks_fail() {
    let mut bitboard: Bitboard = EMPTY;
    set_bit!(bitboard, Square::e4);
    set_bit!(bitboard, Square::c4);
    assert_ne!(PawnAttacks::mask_pawn_attacks(Square::d6, Color::Black), bitboard);
}
