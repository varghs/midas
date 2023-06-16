pub struct FEN<'a>(pub &'a str);

pub const EMPTY_BOARD: FEN = FEN("8/8/8/8/8/8/8/8 w - - ");
pub const START_POSITION: FEN = FEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ");
pub const TRICKY_POSITION: FEN = FEN("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ");
pub const KILLER_POSITION: FEN = FEN("rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1");
pub const CMK_POSITION: FEN = FEN("r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9 ");
