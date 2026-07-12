use chess_kit_collections::Value;

#[derive(Clone, Copy, Default)]
pub struct WideValue {
    priority: i8,
    depth: i8,
    score: i16,
    key_data: [u64; 3],
}

impl WideValue {
    #[inline]
    pub fn new(seed: u64) -> Self {
        Self {
            priority: (seed & 0x7f) as i8,
            depth: ((seed >> 8) & 0x7f) as i8,
            score: (seed as i16).wrapping_mul(31),
            key_data: [seed, seed.rotate_left(19), seed.rotate_right(13)],
        }
    }

    #[inline]
    pub fn payload(&self) -> i64 {
        self.priority as i64
            + self.depth as i64
            + self.score as i64
            + self
                .key_data
                .iter()
                .fold(0_i64, |acc, value| acc ^ *value as i64)
    }
}

impl Value for WideValue {
    #[inline]
    fn priority(&self) -> i8 {
        self.priority
    }
}
