use crate::errors::{BloomFilterError, ErrorKind};
use crate::hash::{indices, sha512};
use bitvec::prelude::*;
use std::sync::{Arc, Mutex};

pub struct AtomicBloomFilter {
    counts: Arc<Mutex<BitVec>>,
}

impl AtomicBloomFilter {
    /// Creates an empty `AtomicBloomFilter` with the given capacity.
    pub fn new(capacity: usize) -> AtomicBloomFilter {
        AtomicBloomFilter {
            counts: Arc::new(Mutex::new(bitvec![0; capacity])),
        }
    }

    /// The capacity of the `AtomicBloomFilter`.
    pub fn capacity(&self) -> usize {
        let guard = match self.counts.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        guard.capacity()
    }

    /// Adds a value to the bloom filter.
    /// If the filter did not have this value present, true is returned.
    /// If the filter potentially had this value present, false is returned.
    pub fn insert(&mut self, value: &[u8]) -> bool {
        let mut guard = match self.counts.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        let mut contained = true;
        let n = guard.len();
        let output = sha512(value);
        let indices = indices(&output);

        for i in indices {
            if let Some(mut marked) = guard.get_mut(i % n) {
                contained = *marked && contained;
                *marked = true;
            }
        }

        contained
    }

    /// Returns `true` if the filter contains a value.
    ///
    /// The value may be any borrowed form of the filter's value type, but
    /// [`Eq`] on the borrowed form *must* match those for the value type.
    pub fn contains(&self, value: &[u8]) -> bool {
        let guard = match self.counts.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let mut contained = true;
        let n = guard.len();
        let output = sha512(value);
        let indices = indices(&output);

        for i in indices {
            if let Some(marked) = guard.get(i % n) {
                contained = *marked && contained;
            }
        }

        contained
    }

    /// Removes a value from the filter. Returns whether the value was probably
    /// present in the filter.
    ///
    /// The value may be any borrowed form of the filter's value type, but
    /// [`Eq`] on the borrowed form *must* match those for the value type.
    ///
    /// # Note
    /// Not all bloom filters support removal of elements.
    pub fn remove(&self, _value: &[u8]) -> Result<bool, BloomFilterError> {
        Err(ErrorKind::NotSupported.into())
    }
}

#[cfg(test)]
mod tests {
    use super::AtomicBloomFilter;
    use crate::errors::ErrorKind;

    #[test]
    fn when_empty() {
        let filter: AtomicBloomFilter = AtomicBloomFilter::new(1);

        assert!(!filter.contains("1".as_bytes()));
    }

    #[test]
    fn when_is_colliding_member() {
        let mut filter = AtomicBloomFilter::new(1);

        filter.insert("1".as_bytes());

        assert!(filter.contains("1".as_bytes()));
    }

    #[test]
    fn when_is_non_colliding_member() {
        let mut filter = AtomicBloomFilter::new(100);

        filter.insert("42".as_bytes());

        assert!(!filter.contains("1".as_bytes()));
    }

    #[test]
    fn remove_when_not_member() {
        let filter = AtomicBloomFilter::new(100);

        let result = filter.remove("1".as_bytes());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ErrorKind::NotSupported);
    }

    #[test]
    fn remove_when_is_member() {
        let mut filter = AtomicBloomFilter::new(100);

        filter.insert("42".as_bytes());

        let result = filter.remove("1".as_bytes());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, ErrorKind::NotSupported);
    }
}
