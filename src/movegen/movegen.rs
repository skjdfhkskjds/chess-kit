use crate::attack_table::AttackTable;

pub struct MoveGenerator<A: AttackTable> {
    pub(crate) attack_table: A,
}

impl<A: AttackTable> MoveGenerator<A> {
    pub fn new(attack_table: A) -> Self {
        Self { attack_table }
    }
}
