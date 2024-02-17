use hex_literal::hex;
use md5::{Digest, Md5};
pub fn md5s(data: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(data);
    let hash = hasher.finalize();
    let d = format!("{:x}", hash);
    d
}
