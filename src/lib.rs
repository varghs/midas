pub type BitBoard = u64;

pub struct Board {
    pub white_pawn: BitBoard,
    pub white_rook: BitBoard,
    pub white_knight: BitBoard,
    pub white_bishop: BitBoard,
    pub white_queen: BitBoard,
    pub white_king: BitBoard,
    pub black_pawn: BitBoard,
    pub black_rook: BitBoard,
    pub black_knight: BitBoard,
    pub black_bishop: BitBoard,
    pub black_queen: BitBoard,
    pub black_king: BitBoard,
}
