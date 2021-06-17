mod ruby;

use std::ffi::CString;

#[no_mangle]
pub unsafe extern "C" fn Init_bloom() {
    let class_name = CString::new("BloomFilter").expect("Invalid class name.");
    let _ = ruby::rb_define_class(class_name.as_ptr(), ruby::rb_cObject);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
