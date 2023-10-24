//! Bloom filter implementation for Dash.

#![deny(missing_docs)]

/// BIP-37 Bloom filter
pub struct BloomFilter {
    data: Vec<u8>,
    n_hashes: u32,
    n_tweak: u32,
    n_flags: u32,
}

/// Error type to indicate incompatible Bloom filter parameters.
#[derive(Debug, thiserror::Error)]
#[error(
    "max filter size exceeded, try increasing FP rate and/or lower the number of expected items"
)]
pub struct BadFilterParameters;

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
}

/// Builder structure for Bloom filter
pub struct BloomFilterBuilder {
    n_tweak: u32,
    n_elements: u32,
    elements_count: u32,
    data: Vec<u8>,
    hash_seeds: Vec<u32>,
}

impl BloomFilterBuilder {
    /// Create new Bloom filter builder with [N_ELEMENTS] maximum expected elements that should satisfy provided false positives rate.
    /// [BadFilterParameters] returned if the false positives rate cannot be satisfied for that number of items.
    pub fn new(n_elements: u32, false_positives_rate: f64) -> Result<Self, BadFilterParameters> {
        Self::new_n_tweak(n_elements, false_positives_rate, 0)
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

    /// Create new Bloom filter builer like at [Self::new], except setting `nTweak` parameter used in murmur hasher initialization.
    pub fn new_n_tweak(
        n_elements: u32,
        false_positives_rate: f64,
        n_tweak: u32,
    ) -> Result<Self, BadFilterParameters> {
        let filter_size = Self::filter_size(n_elements, false_positives_rate)?;

        let n_hashes = Self::hash_fns_number(n_elements, filter_size);
        let hash_seeds = (0..n_hashes)
            .map(|i| i.overflowing_mul(0xFBA4C795).0 + n_tweak)
            .collect();

        let data = vec![0u8; filter_size];

        Ok(BloomFilterBuilder {
            n_elements,
            n_tweak,
            data,
            hash_seeds,
            elements_count: 0,
        })
    }

    /// Finalize Bloom filter
    pub fn build(self) -> BloomFilter {
        BloomFilter {
            data: self.data,
            n_hashes: self.hash_seeds.len() as u32,
            n_tweak: self.n_tweak,
            n_flags: 0,
        }
    }

    /// Finalize Bloom filter with `nFlags` setting.
    pub fn build_with_n_flags(self, n_flags: u32) -> BloomFilter {
        BloomFilter {
            data: self.data,
            n_hashes: self.hash_seeds.len() as u32,
            n_tweak: self.n_tweak,
            n_flags,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_filter() {
        let _filter: BloomFilter = BloomFilter::builder(5, 0.001)
            .expect("parameters are correct")
            .build();
    }
}
