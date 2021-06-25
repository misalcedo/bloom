mod bloom_filter;
mod errors;

pub use bloom_filter::BloomFilter;
use std::ffi::c_void;
use std::hash::Hash;

#[no_mangle]
pub unsafe extern "C" fn new_bloom_filter() -> BloomFilter<i32> {
    BloomFilter::new()
}

#[no_mangle]
pub unsafe extern "C" fn drop_bloom_filter(bloom_filter: BloomFilter<i32>) {
    drop(bloom_filter)
}
