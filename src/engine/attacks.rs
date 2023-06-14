use self::king_attacks::KingAttacks;
use self::bishop_attacks::BishopAttacks;
use self::knight_attacks::KnightAttacks;
use self::pawn_attacks::PawnAttacks;
use self::rook_attacks::RookAttacks;

pub mod king_attacks;
pub mod knight_attacks;
pub mod pawn_attacks;
pub mod bishop_attacks;
pub mod rook_attacks;

struct AttackTables {
    pawns: PawnAttacks,
    knights: KnightAttacks,
    kings: KingAttacks,
    bishops: BishopAttacks,
    rooks: RookAttacks,
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
