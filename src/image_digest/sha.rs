use sha2::Digest;

/// sha256 with hex encoding (lower case only)
pub const SHA256: &str = "sha256";
/// sha384 with hex encoding (lower case only)
pub const SHA384: &str = "sha384";
/// sha512 with hex encoding (lower case only)
pub const SHA512: &str = "sha512";

/// Sha256 ...
#[derive(Debug, Clone)]
pub struct Sha256 {
    pub hasher: sha2::Sha256,
}

impl super::algorithm::CryptoHash for Sha256 {
    fn size(self: &Self) -> usize {
        256 / 8
    }

    fn available(&self) -> bool {
        true
    }
}

impl Sha256 {
    // New returns a new hash.Hash calculating the given hash function. If the hash function is not
    // available, it may panic.
    pub fn new() -> Self {
        Self {
            hasher: sha2::Sha256::new(),
        }
    }
}

/// Sha384 ...
pub struct Sha384 {
    pub hasher: sha2::Sha384,
}

impl super::algorithm::CryptoHash for Sha384 {
    fn size(self: &Self) -> usize {
        384 / 8
    }

    fn available(&self) -> bool {
        true
    }
}

impl Sha384 {
    // New returns a new hash.Hash calculating the given hash function. If the hash function is not
    // available, it may panic.
    pub fn new() -> Self {
        Self {
            hasher: sha2::Sha384::new(),
        }
    }
}

/// Sha512 ...
pub struct Sha512 {
    pub hasher: sha2::Sha512,
}

impl super::algorithm::CryptoHash for Sha512 {
    fn size(self: &Self) -> usize {
        512 / 64
    }

    fn available(&self) -> bool {
        true
    }
}

impl Sha512 {
    // New returns a new hash.Hash calculating the given hash function. If the hash function is not
    // available, it may panic.
    pub fn new() -> Self {
        Self {
            hasher: sha2::Sha512::new(),
        }
    }
}
