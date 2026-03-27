use sha2::{Digest, Sha256};

pub fn hash_password(value: impl Into<String>, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update((value.into() + salt).as_bytes());

    let result = hex::encode(hasher.finalize_reset());
    format!("{}", result)
}
