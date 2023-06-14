use midas::engine::bitboard::{print_bitboard, Bitboard, EMPTY, LS1B, NOTHFILE, UNIVERSE};

#[test]
fn count_universe_1() {
    let b = UNIVERSE;
    assert_eq!(Bitboard::count_bits(b), 64);
}

#[test]
fn count_empty_2() {
    let b = EMPTY;
    assert_eq!(Bitboard::count_bits(b), 0);
}

#[test]
fn count_3() {
    let b = 3;
    assert_eq!(Bitboard::count_bits(b), 2);
}

#[test]
fn count_4() {
    let b = 67;
    assert_eq!(Bitboard::count_bits(b), 3);
}

#[test]
fn show_count() {
    let b = NOTHFILE;

    print_bitboard(b);
    println!("there are {}", Bitboard::count_bits(b));
}
