mod atomic_bloom_filter;
mod bloom_filter;
mod errors;
mod hash;

pub use atomic_bloom_filter::AtomicBloomFilter;
pub use bloom_filter::BloomFilter;
use libc::{c_char, size_t};
use std::ffi::CStr;

/// Type used for returning and propagating errors.
#[repr(C)]
pub enum BloomResult<O, E> {
    Ok(O),
    Err(E),
}

impl<O, E> From<Result<O, E>> for BloomResult<O, E> {
    fn from(result: Result<O, E>) -> Self {
        match result {
            Ok(value) => BloomResult::Ok(value),
            Err(error) => BloomResult::Err(error),
        }
    }
}

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

#[no_mangle]
pub unsafe extern "C" fn bloom_remove(
    bloom_filter: Option<&mut BloomFilter>,
    value: *const c_char,
) -> BloomResult<bool, errors::BloomFilterError> {
    if value.is_null() {
        return BloomResult::Ok(false);
    }

    let value = CStr::from_ptr(value);

    BloomResult::from(match bloom_filter {
        Some(bloom_filter) => bloom_filter.remove(value.to_bytes()),
        _ => Ok(false),
    })
}

#[no_mangle]
pub unsafe extern "C" fn atomic_bloom_new(capacity: size_t) -> *mut AtomicBloomFilter {
    Box::into_raw(Box::new(AtomicBloomFilter::new(capacity)))
}

#[no_mangle]
pub unsafe extern "C" fn atomic_bloom_drop(bloom_filter: *mut AtomicBloomFilter) {
    if !bloom_filter.is_null() {
        Box::from_raw(bloom_filter);
    }
}

#[no_mangle]
pub unsafe extern "C" fn atomic_bloom_capacity(bloom_filter: Option<&AtomicBloomFilter>) -> size_t {
    match bloom_filter {
        Some(bloom_filter) => bloom_filter.capacity(),
        None => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn atomic_bloom_insert(
    bloom_filter: Option<&mut AtomicBloomFilter>,
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
pub unsafe extern "C" fn atomic_bloom_contains(
    bloom_filter: Option<&AtomicBloomFilter>,
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

#[no_mangle]
pub unsafe extern "C" fn atomic_bloom_remove(
    bloom_filter: Option<&mut AtomicBloomFilter>,
    value: *const c_char,
) -> BloomResult<bool, errors::BloomFilterError> {
    if value.is_null() {
        return BloomResult::Ok(false);
    }

    let value = CStr::from_ptr(value);

    BloomResult::from(match bloom_filter {
        Some(bloom_filter) => bloom_filter.remove(value.to_bytes()),
        _ => Ok(false),
    })
}
