mod bloom_filter;
mod errors;

pub use bloom_filter::BloomFilter;
use libc::{c_char, size_t};
use std::ffi::CStr;

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
pub unsafe extern "C" fn BloomFilterCapacity(bloom_filter: Option<&BloomFilter>) -> size_t {
    match bloom_filter {
        Some(bloom_filter) => bloom_filter.capacity(),
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn BloomFilterInsert(
    bloom_filter: Option<&mut BloomFilter>,
    value: *const c_char,
) -> bool {
    if value.is_null() {
        return false;
    }

    let value = CStr::from_ptr(value);

    match (bloom_filter, value.to_str()) {
        (Some(bloom_filter), Ok(value)) => bloom_filter.insert(value),
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn BloomFilterContains(
    bloom_filter: Option<&BloomFilter>,
    value: *const c_char,
) -> bool {
    if value.is_null() {
        return false;
    }

    let value = CStr::from_ptr(value);

    match (bloom_filter, value.to_str()) {
        (Some(bloom_filter), Ok(value)) => bloom_filter.contains(value),
        _ => false,
    }
}
