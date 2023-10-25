//! Family of murmur3 hash functions.

use std::io::Cursor;

#[derive(Debug, Clone)]
pub(crate) struct Hasher {
    pub(crate) filter_bits_len: usize,
    pub(crate) hash_seeds: Vec<u32>,
}

impl Hasher {
    /// Apply multiple hash functions to input and return an iterator of hash results
    pub(crate) fn hash_indexes<'a>(&'a self, item: &'a [u8]) -> impl Iterator<Item = usize> + 'a {
        self.hash_seeds.iter().map(move |seed| {
            (murmur3::murmur3_32(&mut Cursor::new(item), *seed).expect("no IO happens") as usize)
                % self.filter_bits_len
        })
    }
}
