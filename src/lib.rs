mod bloom_filter;
mod errors;

pub use bloom_filter::BloomFilter;
use libc::size_t;

#[no_mangle]
pub unsafe extern "C" fn BloomFilterNew(capacity: size_t) -> *mut BloomFilter {
    Box::into_raw(Box::new(BloomFilter::new(capacity)))
}

#[no_mangle]
pub unsafe extern "C" fn BloomFilterDrop(bloom_filter: *mut BloomFilter) {
    if !bloom_filter.is_null() {
        Box::from_raw(bloom_filter);
    }
}

#[no_mangle]
pub unsafe extern "C" fn BloomFilterCapacity(bloom_filter: Option<&BloomFilter>) -> usize {
    match bloom_filter {
        Some(bloom_filter) => bloom_filter.capacity(),
        None => 0,
    }
}
