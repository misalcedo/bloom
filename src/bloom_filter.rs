use digest::{Digest, Output};
use sha3::Sha3_512;

pub struct BloomFilter {
    counts: Vec<bool>,
}

fn hash<T: Digest>(value: &[u8]) -> Output<T> {
    let mut hasher = T::new();

    hasher.update(value);

    hasher.finalize()
}

fn indices<T: Digest>(hash: &Output<T>) -> &[usize] {
    let (_prefix, indices, _suffix) = unsafe { hash.align_to::<usize>() };

    indices
}

impl BloomFilter {
    /// Creates an empty `BloomFilter` with the given capacity.
    pub fn new(capacity: usize) -> BloomFilter {
        BloomFilter {
            counts: vec![false; capacity],
        }
    }

    /// The capacity of the `BloomFilter`.
    pub fn capacity(&self) -> usize {
        self.counts.capacity()
    }

    /// Adds a value to the bloom filter.
    /// If the filter did not have this value present, true is returned.
    /// If the filter potentially had this value present, false is returned.
    pub fn insert(&mut self, value: &[u8]) -> bool {
        let mut contained = true;
        let n = self.counts.len();
        let output = hash::<Sha3_512>(value);
        let indices = indices::<Sha3_512>(&output);

        for i in indices {
            if let Some(marked) = self.counts.get_mut(i % n) {
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
        let mut contained = true;
        let n = self.counts.len();
        let output = hash::<Sha3_512>(value);
        let indices = indices::<Sha3_512>(&output);

        for i in indices {
            if let Some(marked) = self.counts.get(i % n) {
                contained = *marked && contained;
            }
        }

        contained
    }
}

#[cfg(test)]
mod tests {
    use super::BloomFilter;

    #[test]
    fn when_empty() {
        let filter: BloomFilter = BloomFilter::new(1);

        assert!(!filter.contains("1".as_bytes()));
    }

    #[test]
    fn when_is_colliding_member() {
        let mut filter = BloomFilter::new(1);

        filter.insert("1".as_bytes());

        assert!(filter.contains("1".as_bytes()));
    }

    #[test]
    fn when_is_non_colliding_member() {
        let mut filter = BloomFilter::new(100);

        filter.insert("42".as_bytes());

        assert!(!filter.contains("1".as_bytes()));
    }
}
