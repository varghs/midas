pub struct Bitboard(pub u64);

pub struct Board {
    white_pawn: Bitboard,
    white_rook: Bitboard,
    white_knight: Bitboard,
    white_bishop: Bitboard,
    white_queen: Bitboard,
    white_king: Bitboard,
    black_pawn: Bitboard,
    black_rook: Bitboard,
    black_knight: Bitboard,
    black_bishop: Bitboard,
    black_queen: Bitboard,
    black_king: Bitboard,
}
