mod bloom_filter;
mod errors;
mod ruby;

pub use bloom_filter::BloomFilter;
use std::ffi::CString;

#[no_mangle]
pub unsafe extern "C" fn Init_bloom() {
    let class_name = CString::new("BloomFilter").expect("Invalid class name.");
    let _ = ruby::rb_define_class(class_name.as_ptr(), ruby::rb_cObject);
}
