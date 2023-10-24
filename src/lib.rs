//! Bloom filter implementation for Dash.

#![deny(missing_docs)]

mod builder;
mod filter;
mod hasher;

pub use builder::{BadFilterParameters, BloomFilterBuilder};
pub use filter::{BloomFilter, BloomFilterData};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_filter() {
        let _filter = BloomFilter::builder(5, 0.001)
            .expect("parameters are correct")
            .build();
    }

    #[test]
    fn no_false_negatives() {
        let filter = BloomFilter::builder_n_tweak(3, 0.001, 5)
            .expect("parameters are correct")
            .add_element(b"kek1")
            .add_element(b"kek2")
            .add_element(b"kek3")
            .build();

        assert!(filter.probably_contains(b"kek1"));
        assert!(filter.probably_contains(b"kek2"));
        assert!(filter.probably_contains(b"kek3"));

        assert!(!filter.probably_contains(b"kek4"));
        assert!(!filter.probably_contains(b"kek5"));
    }
}
