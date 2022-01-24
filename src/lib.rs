use sha2::Digest;

pub trait Hasher {
    fn hash(&self, input: Vec<u8>) -> String;
}

pub struct Sha256;

impl Hasher for Sha256 {
    fn hash(&self, input: Vec<u8>) -> String {
        let mut hasher = sha2::Sha256::new();
        hasher.update(input);
        return hex::encode(hasher.finalize());
    }
}

pub struct Sha512;

impl Hasher for Sha512 {
    fn hash(&self, input: Vec<u8>) -> String {
        let mut hasher = sha2::Sha512::new();
        hasher.update(input);
        return hex::encode(hasher.finalize());
    }
}
