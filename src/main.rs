use std::{mem::size_of, io::stdin};

use midas::{
    engine::{
        attacks::slider_attacks::SliderAttacks,
        attacks::AttackTables,
        bitboard::{print_bitboard, Bitboard, EMPTY, LS1B, ONE},
        board::{Board, Castle, Color, Piece, BoardState},
        fen::*,
        square::Square,
        move_gen::*,
        r#move::{Move, MoveList, MoveType}
    },
    set_bit,
};

fn main() {
    /*
    for i in 0..64 {
        print_bitboard(RookAttacks::mask_rook_attack((i as usize).try_into().unwrap()));
        println!();
    }
    */
    const N: usize = 1_000_000_000;

    std::thread::Builder::new()
        .stack_size(size_of::<u64>() * N)
        .spawn(|| {
            let mut b = BoardState::new();
            let mut buf = String::new();
            let fen = FEN("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1 ");

            b.board.parse_fen(TRICKY_POSITION);
            b.board.side = Color::Black;
            println!("{}", b.board);
            b.board.generate_moves();

            for m in (&b.board.move_list.moves[..b.board.move_list.count]).to_vec() {
                b.preserve();

                println!("{}", m);
                b.make_move(m, MoveType::AllMoves);
                println!("{}", b.board);
                stdin().read_line(&mut buf).expect("fail");

                b.restore();
                println!("{}", b.board);
                stdin().read_line(&mut buf).expect("fail");
            }
        })
        .unwrap()
        .join()
        .unwrap();
}
