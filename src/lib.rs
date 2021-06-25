mod bloom_filter;
mod errors;

pub use bloom_filter::BloomFilter;

#[no_mangle]
pub unsafe extern "C" fn new_bloom_filter() -> *mut BloomFilter {
    Box::into_raw(Box::new(BloomFilter::new()))
}

#[no_mangle]
pub unsafe extern "C" fn drop_bloom_filter(bloom_filter: *mut BloomFilter) {
    drop(Box::from_raw(bloom_filter))
}
