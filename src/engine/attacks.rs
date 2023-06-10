use self::pawn::PawnAttacks;

pub mod pawn;

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
