use self::pawn_attacks::PawnAttacks;
pub mod knight_attacks;
pub mod pawn_attacks;

struct AttackTables {
    pawn_attacks: PawnAttacks,
}

impl AttackTables {
    // for every attack_table calls populate method
    fn populate(&mut self) {
        todo!();
    }

    fn populate_leapers_attacks(&mut self) {
        self.pawn_attacks.populate();
    }
}
