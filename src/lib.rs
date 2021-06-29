mod bloom_filter;
mod errors;

pub use bloom_filter::BloomFilter;
use libc::{c_char, size_t};
use std::ffi::CStr;

#[no_mangle]
pub unsafe extern "C" fn bloom_new(capacity: size_t) -> *mut BloomFilter {
    Box::into_raw(Box::new(BloomFilter::new(capacity)))
}

#[no_mangle]
pub unsafe extern "C" fn bloom_drop(bloom_filter: *mut BloomFilter) {
    if !bloom_filter.is_null() {
        Box::from_raw(bloom_filter);
    }
}

#[no_mangle]
pub unsafe extern "C" fn bloom_capacity(bloom_filter: Option<&BloomFilter>) -> size_t {
    match bloom_filter {
        Some(bloom_filter) => bloom_filter.capacity(),
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn bloom_insert(
    bloom_filter: Option<&mut BloomFilter>,
    value: *const c_char,
) -> bool {
    if value.is_null() {
        return false;
    }

    let value = CStr::from_ptr(value);

    match bloom_filter {
        Some(bloom_filter) => bloom_filter.insert(value.to_bytes()),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn bloom_contains(
    bloom_filter: Option<&BloomFilter>,
    value: *const c_char,
) -> bool {
    if value.is_null() {
        return false;
    }

    let value = CStr::from_ptr(value);

    match bloom_filter {
        Some(bloom_filter) => bloom_filter.contains(value.to_bytes()),
        _ => false,
    }
}
