use midas::engine::bitboard::{print_bitboard, Bitboard, EMPTY, LS1B, NOTHFILE, UNIVERSE};

#[test]
fn count_universe_1() {
    let b = UNIVERSE;
    assert_eq!(b.count_bits(), 64);
}

#[test]
fn count_empty_2() {
    let b = EMPTY;
    assert_eq!(b.count_bits(), 0);
}

#[test]
fn count_3() {
    let b: Bitboard = 3;
    assert_eq!(b.count_bits(), 2);
}

#[test]
fn count_4() {
    let b: Bitboard = 67;
    assert_eq!(b.count_bits(), 3);
}

#[test]
fn show_count() {
    let b = NOTHFILE;

    print_bitboard(b);
    println!("there are {}", b.count_bits());
}
