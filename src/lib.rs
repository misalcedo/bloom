mod bloom_filter;
mod errors;

pub use bloom_filter::BloomFilter;

#[no_mangle]
pub unsafe extern "C" fn new_bloom_filter() -> *mut BloomFilter {
    &mut BloomFilter::new()
}

#[no_mangle]
pub unsafe extern "C" fn drop_bloom_filter(bloom_filter: *const BloomFilter) {
    drop(bloom_filter)
}
