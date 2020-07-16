//! SHA3-256 示例
extern crate crypto;
extern crate rustc_hex;

use self::crypto::digest::Digest;
use self::crypto::sha3::Sha3;
use self::crypto::md5::Md5;

#[test]
fn test_rustc() {
    // create a SHA3-256 object
    let mut hasher = Md5::new();
    // write input message
    hasher.input_str("hello world");
    // read hash digest
    let hex = hasher.result_str();
    println!("{}", hex)
}