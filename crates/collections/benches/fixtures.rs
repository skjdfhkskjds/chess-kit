#![allow(dead_code)]

use chess_kit_collections::{Copyable, HashFn, HashKey, Value};

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

pub struct SplitU64Hasher;

impl HashFn<u64> for SplitU64Hasher {
    #[inline]
    fn hash(key: &u64) -> HashKey {
        HashKey {
            index: (*key >> 32) as usize,
            tag: *key as u32,
        }
    }
}

#[inline]
pub fn spread_key(i: u64) -> u64 {
    let mut x = i.wrapping_add(0x9e37_79b9_7f4a_7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

#[inline]
pub fn colliding_key(bucket: u64, tag: u32) -> u64 {
    (bucket << 32) | tag as u64
}
