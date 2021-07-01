use openssl::sha;

pub fn sha512(value: &[u8]) -> [u8; 64] {
    let mut hasher = sha::Sha512::new();

    hasher.update(value);

    hasher.finish()
}

pub fn indices(hash: &[u8]) -> &[usize] {
    let (_prefix, indices, _suffix) = unsafe { hash.align_to::<usize>() };

    indices
}
