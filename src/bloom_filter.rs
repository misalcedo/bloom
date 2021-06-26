#[repr(C)]
pub struct BloomFilter {
    capacity: usize,
}

impl BloomFilter {
    /// Creates an empty `BloomFilter`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bloom::BloomFilter;
    /// let filter: BloomFilter = BloomFilter::new(1);
    /// ```
    pub fn new(capacity: usize) -> BloomFilter {
        BloomFilter { capacity }
    }

    /// The capacity of the `BloomFilter`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bloom::BloomFilter;
    /// let filter: BloomFilter = BloomFilter::new(42);
    ///
    /// assert_eq!(s.capacity(42), 42);
    /// ```
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Adds a value to the bloom filter.
    /// If the filter did not have this value present, true is returned.
    /// If the filter potentially had this value present, false is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use bloom::BloomFilter;
    ///
    /// let filter: BloomFilter = BloomFilter::new();
    ///
    /// assert_eq!(filter.insert(2), true);
    /// assert_eq!(filter.insert(2), false);
    /// assert_eq!(filter.contains(&2), true);
    /// ```
    pub fn insert(&mut self, _value: i32) -> bool {
        true
    }

    /// Returns `true` if the filter contains a value.
    ///
    /// The value may be any borrowed form of the filter's value type, but
    /// [`Eq`] on the borrowed form *must* match those for the value type.
    ///
    /// # Examples
    ///
    /// ```
    /// use bloom::BloomFilter;
    ///
    /// let filter: BloomFilter = BloomFilter::new();
    ///
    /// assert_eq!(filter.contains(&1), false);
    /// set.insert(1);
    /// assert_eq!(filter.contains(&1), true);
    /// assert_eq!(filter.contains(&4), false);
    /// ```
    pub fn contains(&self, _value: &i32) -> bool {
        false
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

        assert!(!filter.contains(&1));
    }

    #[test]
    fn when_is_colliding_member() {
        let mut filter = BloomFilter::new(1);

        filter.insert(1);

        assert!(filter.contains(&1));
    }

    #[test]
    fn when_is_non_colliding_member() {
        let mut filter = BloomFilter::new(1);

        filter.insert(42);

        assert!(!filter.contains(&1));
    }
}
