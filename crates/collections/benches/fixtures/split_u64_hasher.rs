use chess_kit_collections::{HashFn, HashKey};

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
