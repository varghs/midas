use self::slider_attacks::bishop_attacks::BishopAttacks;
use self::king_attacks::KingAttacks;
use self::knight_attacks::KnightAttacks;
use self::pawn_attacks::PawnAttacks;
use self::slider_attacks::rook_attacks::RookAttacks;

pub mod slider_attacks;
pub mod king_attacks;
pub mod knight_attacks;
pub mod pawn_attacks;

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
