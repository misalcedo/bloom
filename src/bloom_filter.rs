use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::Hash;

pub struct BloomFilter<T> {
    set: HashSet<T>,
}

impl<T: Hash + Eq> BloomFilter<T> {
    /// Creates an empty `BloomFilter`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bloom::BloomFilter;
    /// let filter: BloomFilter<i32> = BloomFilter::new();
    /// ```
    pub fn new() -> BloomFilter<T> {
        BloomFilter {
            set: HashSet::new(),
        }
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
    /// let filter: BloomFilter<i32> = BloomFilter::new();
    ///
    /// assert_eq!(set.insert(2), true);
    /// assert_eq!(set.insert(2), false);
    /// assert_eq!(set.contains(&2), true);
    /// ```
    pub fn insert(&mut self, value: T) -> bool {
        self.set.insert(value)
    }

    /// Returns `true` if the filter contains a value.
    ///
    /// The value may be any borrowed form of the filter's value type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the value type.
    ///
    /// # Examples
    ///
    /// ```
    /// use bloom::BloomFilter;
    ///
    /// let filter: BloomFilter<i32> = BloomFilter::new();
    ///
    /// assert_eq!(set.contains(&1), false);
    /// set.insert(1);
    /// assert_eq!(set.contains(&1), true);
    /// assert_eq!(set.contains(&4), false);
    /// ```
    pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.set.contains(value)
    }
}

impl<T> Drop for BloomFilter<T> {
    fn drop(&mut self) {
        println!("> Dropping a bloom filter!");
    }
}

#[cfg(test)]
mod tests {
    use super::BloomFilter;

    #[test]
    fn when_empty() {
        let filter: BloomFilter<i32> = BloomFilter::new();

        assert!(!filter.contains(&1));
    }

    #[test]
    fn when_is_colliding_member() {
        let mut filter = BloomFilter::new();

        filter.insert(1);

        assert!(filter.contains(&1));
    }

    #[test]
    fn when_is_non_colliding_member() {
        let mut filter = BloomFilter::new();

        filter.insert(42);

        assert!(!filter.contains(&1));
    }
}
