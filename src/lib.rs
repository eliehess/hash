pub mod sha2 {
    use ::sha2::Digest;

    pub fn sha256(input: Vec<u8>) -> String {
        let mut hasher = ::sha2::Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }

    pub fn sha512(input: Vec<u8>) -> String {
        let mut hasher = ::sha2::Sha512::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }
}

pub mod sha3 {
    use ::sha3::Digest;

    pub fn sha3_256(input: Vec<u8>) -> String {
        let mut hasher = ::sha3::Sha3_256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }

    pub fn sha3_512(input: Vec<u8>) -> String {
        let mut hasher = ::sha3::Sha3_512::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }
}

pub mod md5 {
    pub fn md5(input: Vec<u8>) -> String {
        format!("{:x}", ::md5::compute(input))
    }
}
