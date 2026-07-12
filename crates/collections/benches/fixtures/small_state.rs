use chess_kit_collections::Copyable;

#[derive(Clone, Copy)]
pub struct SmallState {
    fields: [u64; 2],
}

impl SmallState {
    #[inline]
    pub fn with_seed(seed: u64) -> Self {
        Self {
            fields: [seed, seed.rotate_left(17)],
        }
    }

    #[inline]
    pub fn update(&mut self, seed: u64) {
        self.fields[0] ^= seed;
        self.fields[1] = self.fields[1].wrapping_add(seed.rotate_left(7));
    }
}

impl Default for SmallState {
    #[inline]
    fn default() -> Self {
        Self { fields: [0; 2] }
    }
}

impl Copyable for SmallState {
    #[inline]
    fn copy_from(&mut self, other: &Self) {
        *self = *other;
    }
}
