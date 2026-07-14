use chess_kit_collections::Copyable;

#[derive(Clone, Copy)]
pub struct HistoryState {
    // Mirrors PositionMetadata's asserted 24-byte layout.
    metadata: [u64; 3],
    capture_info: u8,
    _padding: [u8; 7],
    // Mirrors the 96-byte tactical CheckInfo cache.
    check_info: [u64; 12],
}

impl HistoryState {
    #[inline]
    pub fn with_seed(seed: u64) -> Self {
        let mut check_info = [0; 12];
        for (idx, value) in check_info.iter_mut().enumerate() {
            *value = seed.wrapping_mul((idx as u64) + 1);
        }

        Self {
            metadata: [seed, seed.rotate_left(11), seed.rotate_right(5)],
            capture_info: seed as u8,
            _padding: [0; 7],
            check_info,
        }
    }

    #[inline]
    pub fn update_metadata(&mut self, seed: u64) {
        self.metadata[0] ^= seed;
        self.metadata[1] = self.metadata[1].wrapping_add(seed);
        self.metadata[2] = self.metadata[2].rotate_left((seed & 31) as u32);
        self.capture_info = seed as u8;
    }
}

impl Default for HistoryState {
    #[inline]
    fn default() -> Self {
        Self {
            metadata: [0; 3],
            capture_info: 0,
            _padding: [0; 7],
            check_info: [0; 12],
        }
    }
}

impl Copyable for HistoryState {
    #[inline]
    fn copy_from(&mut self, other: &Self) {
        self.metadata = other.metadata;
    }
}
