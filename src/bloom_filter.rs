use std::collections::HashSet;

pub struct BloomFilter {
    set: HashSet<String>,
}

impl BloomFilter {
    /// Creates an empty `BloomFilter`.
    pub fn new(capacity: usize) -> BloomFilter {
        BloomFilter {
            set: HashSet::with_capacity(capacity),
        }
    }

    /// The capacity of the `BloomFilter`.
    pub fn capacity(&self) -> usize {
        self.set.capacity()
    }

    /// Adds a value to the bloom filter.
    /// If the filter did not have this value present, true is returned.
    /// If the filter potentially had this value present, false is returned.
    pub fn insert(&mut self, value: &str) -> bool {
        self.set.insert(value.to_string())
    }

    /// Returns `true` if the filter contains a value.
    ///
    /// The value may be any borrowed form of the filter's value type, but
    /// [`Eq`] on the borrowed form *must* match those for the value type.
    pub fn contains(&self, value: &str) -> bool {
        self.set.contains(value)
    }
}

impl Drop for BloomFilter {
    fn drop(&mut self) {
        println!("> Dropping a bloom filter!");
    }
}

#[cfg(test)]
mod tests {
    use super::BloomFilter;

    #[test]
    fn when_empty() {
        let filter: BloomFilter = BloomFilter::new(1);

        assert!(!filter.contains("1"));
    }

    #[test]
    fn when_is_colliding_member() {
        let mut filter = BloomFilter::new(1);

        filter.insert("1");

        assert!(filter.contains("1"));
    }

    #[test]
    fn when_is_non_colliding_member() {
        let mut filter = BloomFilter::new(1);

        filter.insert("42");

        assert!(!filter.contains("1"));
    }
}
