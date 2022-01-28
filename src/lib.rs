pub trait Hasher {
    fn hash(&self, input: Vec<u8>) -> String;
}

pub mod sha2 {
    use super::Hasher;
    use ::sha2::Digest;

    pub struct Sha256;
    
    impl Hasher for Sha256 {
        fn hash(&self, input: Vec<u8>) -> String {
            let mut hasher = ::sha2::Sha256::new();
            hasher.update(input);
            format!("{:x}", hasher.finalize())
        }
    }
    
    pub struct Sha512;
    
    impl Hasher for Sha512 {
        fn hash(&self, input: Vec<u8>) -> String {
            let mut hasher = ::sha2::Sha512::new();
            hasher.update(input);
            format!("{:x}", hasher.finalize())
        }
    }
}

pub mod sha3 {
    use super::Hasher;
    use ::sha3::Digest;

    pub struct Sha3_256;

    impl Hasher for Sha3_256 {
        fn hash(&self, input: Vec<u8>) -> String {
            let mut hasher = ::sha3::Sha3_256::new();
            hasher.update(input);
            format!("{:x}", hasher.finalize())
        }
    }

    pub struct Sha3_512;

    impl Hasher for Sha3_512 {
        fn hash(&self, input: Vec<u8>) -> String {
            let mut hasher = ::sha3::Sha3_512::new();
            hasher.update(input);
            format!("{:x}", hasher.finalize())
        }
    }
}

pub mod md5 {
    use super::Hasher;

    pub struct MD5;

    impl Hasher for MD5 {
        fn hash(&self, input: Vec<u8>) -> String {
            format!("{:x}", ::md5::compute(input))
        }
    }
}
