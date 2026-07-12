use chess_kit_collections::Value;

#[derive(Clone, Copy, Default)]
pub struct CompactValue {
    priority: i8,
    score: i16,
    best_move: u16,
}

impl CompactValue {
    #[inline]
    pub fn new(seed: u64) -> Self {
        Self {
            priority: (seed & 0x7f) as i8,
            score: (seed as i16).wrapping_mul(17),
            best_move: seed as u16,
        }
    }

    #[inline]
    pub fn payload(&self) -> i32 {
        self.priority as i32 + self.score as i32 + self.best_move as i32
    }
}

impl Value for CompactValue {
    #[inline]
    fn priority(&self) -> i8 {
        self.priority
    }
}
