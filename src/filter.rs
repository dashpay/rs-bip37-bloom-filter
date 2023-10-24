//! Bloom filter type and a data representation of it

use bitvec::vec::BitVec;

use crate::{hasher::Hasher, BadFilterParameters, BloomFilterBuilder};

/// Bloom filter fields exposed for serialization
#[derive(Debug, Clone)]
pub struct BloomFilterData {
    /// Bloom filter byte array
    pub v_data: Vec<u8>,
    /// Number of hash functions used
    pub n_hash_funcs: u32,
    /// Hash functions initialization adjustment
    pub n_tweak: u32,
    /// TODO
    pub n_flags: u32,
}

impl From<BloomFilter> for BloomFilterData {
    fn from(bloom_filter: BloomFilter) -> Self {
        BloomFilterData {
            v_data: bloom_filter.filter_bits.into(),
            n_hash_funcs: bloom_filter.hasher.hash_seeds.len() as u32,
            n_tweak: bloom_filter.n_tweak,
            n_flags: bloom_filter.n_flags,
        }
    }
}

/// BIP-37 Bloom filter
#[derive(Debug, Clone)]
pub struct BloomFilter {
    pub(crate) filter_bits: BitVec<u8>,
    pub(crate) n_tweak: u32,
    pub(crate) n_flags: u32,
    pub(crate) hasher: Hasher,
}

impl BloomFilter {
    /// Get a new builder
    pub fn builder(
        n_elements: u32,
        false_positives_rate: f64,
    ) -> Result<BloomFilterBuilder, BadFilterParameters> {
        BloomFilterBuilder::new(n_elements, false_positives_rate)
    }

    /// Get a new builder with `nTweak`
    pub fn builder_n_tweak(
        n_elements: u32,
        false_positives_rate: f64,
        n_tweak: u32,
    ) -> Result<BloomFilterBuilder, BadFilterParameters> {
        BloomFilterBuilder::new_n_tweak(n_elements, false_positives_rate, n_tweak)
    }

    /// Check if the filter possibly contains the item
    pub fn probably_contains(&self, item: &[u8]) -> bool {
        let mut indexes = self.hasher.hash_indexes(item);
        indexes.all(|i| {
            self.filter_bits
                .get(i)
                .as_deref()
                .copied()
                .unwrap_or_default()
        })
    }
}
