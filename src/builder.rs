//! Bloom filter builder module.

use bitvec::vec::BitVec;

use crate::{hasher::Hasher, BloomFilter};

/// Error type to indicate incompatible Bloom filter parameters.
#[derive(Debug, thiserror::Error)]
#[error(
    "max filter size exceeded, try increasing FP rate and/or lower the number of expected items"
)]
pub struct BadFilterParameters;

/// Builder structure for Bloom filter
pub struct BloomFilterBuilder {
    n_tweak: u32,
    filter_bits: BitVec<u8>,
    hasher: Hasher,
}

impl BloomFilterBuilder {
    /// Create new Bloom filter builder with `n_elements` maximum expected elements that
    /// should satisfy provided false positives rate.  [BadFilterParameters] returned if
    /// the false positives rate cannot be satisfied for that number of items.
    pub fn new(n_elements: u32, false_positives_rate: f64) -> Result<Self, BadFilterParameters> {
        Self::new_n_tweak(n_elements, false_positives_rate, 0)
    }

    /// Create new Bloom filter builer like at [Self::new], except setting `nTweak`
    /// parameter used in murmur hasher initialization.
    pub fn new_n_tweak(
        n_elements: u32,
        false_positives_rate: f64,
        n_tweak: u32,
    ) -> Result<Self, BadFilterParameters> {
        let filter_size_bytes = Self::filter_size(n_elements, false_positives_rate)?;

        let n_hashes = Self::hash_fns_number(n_elements, filter_size_bytes);
        let hash_seeds = (0..n_hashes)
            .map(|i| i.overflowing_mul(0xFBA4C795).0 + n_tweak)
            .collect();

        let data = BitVec::<u8>::repeat(false, filter_size_bytes * 8);

        let hasher = Hasher {
            filter_bits_len: data.len(),
            hash_seeds,
        };

        Ok(BloomFilterBuilder {
            n_tweak,
            filter_bits: data,
            hasher,
        })
    }

    fn filter_size(
        n_elements: u32,
        false_positives_rate: f64,
    ) -> Result<usize, BadFilterParameters> {
        let filter_size =
            ((-1.0 / 2.0_f64.ln().powi(2) * n_elements as f64 * false_positives_rate.ln()) as u64
                / 8)
            .try_into();

        match filter_size {
            Ok(s) if s < 36_000 => Ok(s),
            _ => Err(BadFilterParameters),
        }
    }

    fn hash_fns_number(n_elements: u32, filter_size: usize) -> u32 {
        ((filter_size * 8) as f64 / n_elements as f64 * 2_f64.ln()) as u32
    }

    /// Add element to Bloom filter
    pub fn add_element(mut self, element: &[u8]) -> Self {
        let indexes = self.hasher.hash_indexes(element);
        for hash in indexes {
            *self
                .filter_bits
                .get_mut(hash)
                .expect("hash result is normalized") = true;
        }

        self
    }

    /// Finalize Bloom filter
    pub fn build(self) -> BloomFilter {
        BloomFilter {
            filter_bits: self.filter_bits,
            n_tweak: self.n_tweak,
            n_flags: 0,
            hasher: self.hasher,
        }
    }

    /// Finalize Bloom filter with `nFlags` setting.
    pub fn build_with_n_flags(self, n_flags: u32) -> BloomFilter {
        BloomFilter {
            filter_bits: self.filter_bits,
            n_tweak: self.n_tweak,
            n_flags,
            hasher: self.hasher,
        }
    }
}
