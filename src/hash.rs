use crate::errors::{BloomFilterError, ErrorKind};
use futures::future::join_all;
use openssl::sha;
use tokio::runtime::Runtime;
use tokio::task::{JoinError, JoinHandle};

pub fn sha512(value: &[u8]) -> [u8; 64] {
    let mut hasher = sha::Sha512::new();

    hasher.update(value);

    hasher.finish()
}

pub fn sha512_multiple(values: &[&[u8]]) -> Result<Vec<[u8; 64]>, BloomFilterError> {
    let runtime = Runtime::new().map_err(|_| BloomFilterError::from(ErrorKind::AsyncRuntime))?;

    runtime.block_on(async {
        let tasks: Vec<JoinHandle<[u8; 64]>> = values
            .iter()
            .map(|&value| value.to_vec())
            .map(|value| tokio::spawn(async move { sha512(&value) }))
            .collect();

        let values: Result<Vec<[u8; 64]>, JoinError> = join_all(tasks).await.into_iter().collect();

        values.map_err(|_| ErrorKind::AsyncRuntime.into())
    })
}

pub fn indices(hash: &[u8]) -> &[usize] {
    let (_prefix, indices, _suffix) = unsafe { hash.align_to::<usize>() };

    indices
}

#[cfg(test)]
mod tests {
    use super::{indices, sha512};
    use crate::hash::sha512_multiple;

    #[test]
    fn single() {
        let value = "Hello, World!".as_bytes();
        let output = sha512(value);
        let indices = indices(&output);

        assert_eq!(
            indices,
            &[
                15622931673454824759,
                11789279641285646771,
                15002322529494823272,
                7835799490558798219,
                63103149339153094,
                16003977717793378922,
                17307022467165316852,
                9728842927814993318
            ]
        );
    }

    #[test]
    fn multiple() {
        let value = "Hello, World!".as_bytes();
        let values = vec![value; 100];

        let output = sha512(value);
        let result = sha512_multiple(&values);

        assert!(result.is_ok());
        for value in result.unwrap() {
            assert_eq!(value, output)
        }
    }
}
