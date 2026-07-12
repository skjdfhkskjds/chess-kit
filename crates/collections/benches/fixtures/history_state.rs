use chess_kit_collections::Copyable;

#[derive(Clone, Copy)]
pub struct HistoryState {
    header: [u64; 3],
    derived: [u64; 12],
}

impl HistoryState {
    #[inline]
    pub fn with_seed(seed: u64) -> Self {
        let mut derived = [0; 12];
        for (idx, value) in derived.iter_mut().enumerate() {
            *value = seed.wrapping_mul((idx as u64) + 1);
        }

        Self {
            header: [seed, seed.rotate_left(11), seed.rotate_right(5)],
            derived,
        }
    }

    #[inline]
    pub fn update_header(&mut self, seed: u64) {
        self.header[0] ^= seed;
        self.header[1] = self.header[1].wrapping_add(seed);
        self.header[2] = self.header[2].rotate_left((seed & 31) as u32);
    }
}

impl Default for HistoryState {
    #[inline]
    fn default() -> Self {
        Self {
            header: [0; 3],
            derived: [0; 12],
        }
    }
}

impl Copyable for HistoryState {
    #[inline]
    fn copy_from(&mut self, other: &Self) {
        self.header = other.header;
    }
}
