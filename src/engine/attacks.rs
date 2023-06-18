use self::king_attacks::KingAttacks;
use self::knight_attacks::KnightAttacks;
use self::pawn_attacks::PawnAttacks;
use self::slider_attacks::SliderAttacks;
use crate::engine::board::Piece;

pub mod king_attacks;
pub mod knight_attacks;
pub mod pawn_attacks;
pub mod slider_attacks;

#[derive(Clone)]
pub struct AttackTables {
    pub pawns: PawnAttacks,
    pub knights: KnightAttacks,
    pub kings: KingAttacks,
    pub sliders: SliderAttacks,
}

impl AttackTables {
    pub fn new() -> Self {
        let pawns = PawnAttacks::new();
        let knights = KnightAttacks::new();
        let kings = KingAttacks::new();
        let sliders = SliderAttacks::new();

        Self {
            pawns,
            knights,
            kings,
            sliders,
        }
    }
    // for every attack_table calls populate method
    pub fn populate(&mut self) {
        self.populate_leapers_attacks();
        self.populate_sliders_attacks();
    }

    pub fn populate_leapers_attacks(&mut self) {
        self.pawns.populate();
        self.knights.populate();
        self.kings.populate();
    }

    pub fn populate_sliders_attacks(&mut self) {
        self.sliders.populate(Piece::Bishop);
        self.sliders.populate(Piece::Rook);
    }
}
