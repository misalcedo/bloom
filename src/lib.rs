mod atomic_bloom_filter;
mod bloom_filter;
mod errors;
mod hash;

pub use atomic_bloom_filter::AtomicBloomFilter;
pub use bloom_filter::BloomFilter;
use libc::{c_char, size_t};
use std::ffi::CStr;
use std::slice;

/// Type used for returning and propagating errors.
#[repr(C)]
pub enum BloomResult<O> {
    Ok(O),
    Err(BloomFilterError),
}

impl<O> From<Result<O, errors::BloomFilterError>> for BloomResult<O> {
    fn from(result: Result<O, errors::BloomFilterError>) -> Self {
        match result {
            Ok(value) => BloomResult::Ok(value),
            Err(error) => BloomResult::Err(error.into()),
        }
    }
}

#[repr(C)]
pub struct BloomFilterError {
    pub kind: ErrorKind,
}

#[repr(C)]
pub enum ErrorKind {
    NotSupported,
    AsyncRuntime,
}

impl From<errors::BloomFilterError> for BloomFilterError {
    fn from(error: errors::BloomFilterError) -> Self {
        BloomFilterError {
            kind: error.kind.into(),
        }
    }
}

impl From<errors::ErrorKind> for ErrorKind {
    fn from(kind: errors::ErrorKind) -> Self {
        match kind {
            errors::ErrorKind::NotSupported => ErrorKind::NotSupported,
            errors::ErrorKind::AsyncRuntime => ErrorKind::AsyncRuntime,
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
pub unsafe extern "C" fn bloom_contains_all(
    bloom_filter: Option<&BloomFilter>,
    values: *const *const c_char,
    value_count: size_t,
) -> BloomResult<bool> {
    if values.is_null() {
        return BloomResult::Ok(false);
    }

    let values: &[*const c_char] = slice::from_raw_parts(values, value_count as usize);

    match bloom_filter {
        Some(bloom_filter) => {
            let bytes: Vec<&[u8]> = values
                .into_iter()
                .map(|&v| CStr::from_ptr(v))
                .map(CStr::to_bytes)
                .collect();

            match bloom_filter.contains_multiple(&bytes) {
                Ok(results) => BloomResult::Ok(results.iter().all(|&value| value)),
                Err(error) => BloomResult::Err(error.into()),
            }
        }
        _ => BloomResult::Ok(false),
    }
}

#[no_mangle]
pub unsafe extern "C" fn bloom_remove(
    bloom_filter: Option<&mut BloomFilter>,
    value: *const c_char,
) -> BloomResult<bool> {
    if value.is_null() {
        return Ok(false).into();
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
pub unsafe extern "C" fn atomic_bloom_contains_all(
    bloom_filter: Option<&AtomicBloomFilter>,
    values: *const *const c_char,
    value_count: size_t,
) -> BloomResult<bool> {
    if values.is_null() {
        return BloomResult::Ok(false);
    }

    let values: &[*const c_char] = slice::from_raw_parts(values, value_count as usize);

    match bloom_filter {
        Some(bloom_filter) => {
            let bytes: Vec<&[u8]> = values
                .into_iter()
                .map(|&v| CStr::from_ptr(v))
                .map(CStr::to_bytes)
                .collect();

            match bloom_filter.contains_multiple(&bytes) {
                Ok(results) => BloomResult::Ok(results.iter().all(|&value| value)),
                Err(error) => BloomResult::Err(error.into()),
            }
        }
        _ => BloomResult::Ok(false),
    }
}

#[no_mangle]
pub unsafe extern "C" fn atomic_bloom_remove(
    bloom_filter: Option<&mut AtomicBloomFilter>,
    value: *const c_char,
) -> BloomResult<bool> {
    if value.is_null() {
        return Ok(false).into();
    }

    let value = CStr::from_ptr(value);

    BloomResult::from(match bloom_filter {
        Some(bloom_filter) => bloom_filter.remove(value.to_bytes()),
        _ => Ok(false),
    })
}
