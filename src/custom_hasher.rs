use std::collections::HashSet;
use std::hash::{BuildHasher, Hasher};
use std::ops::BitXor;

#[cfg(target_pointer_width = "64")]
const K: usize = 0x517cc1b727220a95;

pub struct FastHash {
    hash: usize,
}

impl Hasher for FastHash {
    #[inline]
    fn finish(&self) -> u64 {
        self.hash as u64
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.hash = self
                .hash
                .rotate_left(5)
                .bitxor(*byte as usize)
                .wrapping_mul(K);
        }
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.hash = self.hash.rotate_left(5).bitxor(i as usize).wrapping_mul(K);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.hash = self.hash.rotate_left(5).bitxor(i as usize).wrapping_mul(K);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.hash = self.hash.rotate_left(5).bitxor(i as usize).wrapping_mul(K);
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.hash = self.hash.rotate_left(5).bitxor(i as usize).wrapping_mul(K);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.hash = self.hash.rotate_left(5).bitxor(i).wrapping_mul(K);
    }
}

#[derive(Default, Clone)]
pub struct FastHashBuilder;

impl BuildHasher for FastHashBuilder {
    type Hasher = FastHash;

    #[inline]
    fn build_hasher(&self) -> FastHash {
        FastHash { hash: K }
    }
}

pub type FastHashSet<K> = HashSet<K, FastHashBuilder>;
