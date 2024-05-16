use fastcrypto::hash::{EllipticCurveMultisetHash, MultisetHash};

fn main() {
    let mut hash1 = EllipticCurveMultisetHash::default();
    hash1.insert(b"World");
    hash1.insert(b"Hello");

    println!("Hash1: {:?}", hash1.digest());
}
