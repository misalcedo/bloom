mod bloom_filter;
mod errors;

pub use bloom_filter::BloomFilter;

#[no_mangle]
pub unsafe extern "C" fn BloomFilterNew(capacity: usize) -> *mut BloomFilter {
    Box::into_raw(Box::new(BloomFilter::new(capacity)))
}

#[no_mangle]
pub unsafe extern "C" fn BloomFilterDrop(bloom_filter: *mut BloomFilter) {
    drop(Box::from_raw(bloom_filter))
}

#[no_mangle]
pub unsafe extern "C" fn BloomFilterCapacity(bloom_filter: *mut BloomFilter) -> usize {
    &*bloom_filter.capacity()
}
