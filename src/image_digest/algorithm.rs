use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

/// sha256 with hex encoding (lower case only)
pub const SHA256: &str = "sha256";
/// sha384 with hex encoding (lower case only)
pub const SHA384: &str = "sha384";
/// sha512 with hex encoding (lower case only)
pub const SHA512: &str = "sha512";

// CANONICAL is the primary digest algorithm used with the distribution
// project. Other digests may be used but this one is the primary storage
// digest.
pub const CANONICAL: &str = SHA256;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Algorithm<'a> {
    pub name: &'a str,
    pub bitsize: isize,
}

impl Algorithm<'_> {
    pub fn new(name: &str, size: isize) -> Algorithm {
        Algorithm {
            name,
            bitsize: size,
        }
    }
}

impl CryptoHash for Algorithm<'static> {
    fn available(self) -> bool {
        true
    }

    fn string(self) -> &'static str {
        self.name
    }

    fn size(self) -> isize {
        self.bitsize
    }

    fn set(self, name: &str) -> Result<Self, Error> {
        match name {
            SHA256 => Ok(Algorithm::new(SHA256, 256)),
            SHA384 => Ok(Algorithm::new(SHA384, 384)),
            SHA512 => Ok(Algorithm::new(SHA512, 512)),
            _ => Err(Error::new(ErrorKind::Other, "Unsupported algorithm")),
        }
    }

    fn digester(&self) -> Box<dyn DynDigest> {
        match self.name {
            SHA256 => Box::new(Sha256::new()),
            SHA384 => Box::new(Sha384::new()),
            SHA512 => Box::new(Sha512::new()),
            _ => panic!("Unsupported algorithm"),
        }
    }

    fn hash(self) -> Box<dyn DynDigest> {
        self.digester()
    }

    fn encode(&self, bytes: &[u8]) -> String {
        let mut digest: Box<dyn DynDigest>;
        match self.name {
            SHA256 => {
                digest = Box::new(Sha256::new());
            }
            SHA384 => {
                digest = Box::new(Sha384::new());
            }
            SHA512 => {
                digest = Box::new(Sha512::new());
            }
            _ => panic!("Unsupported algorithm"),
        };
        digest.update(bytes);
        hex::encode(digest.finalize())
    }

    fn from_reader<R: std::io::Read>(&self, reader: R) -> String {
        let mut digest: Box<dyn DynDigest>;
        match self.name {
            SHA256 => {
                digest = Box::new(Sha256::new());
            }
            SHA384 => {
                digest = Box::new(Sha384::new());
            }
            SHA512 => {
                digest = Box::new(Sha512::new());
            }
            _ => panic!("Unsupported algorithm"),
        };
        let mut reader = reader;
        let mut buffer = [0; 1024];
        loop {
            let len = match reader.read(&mut buffer) {
                Ok(len) => len,
                Err(_) => break,
            };
            if len == 0 {
                break;
            }
            digest.update(&buffer[..len]);
        }
        hex::encode(digest.finalize())
    }
}

use digest::DynDigest;
use sha2::{Digest, Sha256, Sha384, Sha512};

/// CryptoHash is the interface that any hash algorithm must implement
pub trait CryptoHash {
    // Available reports whether the given hash function is usable in the current binary.
    fn available(self) -> bool;
    // Size returns the length, in bytes, of a digest resulting from the given hash function.
    fn size(self) -> isize;
    // String returns the name of the hash function.
    fn string(self) -> &'static str;
    // Set implemented to allow use of Algorithm as a command line flag.
    fn set(self, _: &str) -> Result<Self, Error>
    where
        Self: Sized;
    // Digester returns a new digester for the specified algorithm. If the algorithm
    // does not have a digester implementation, nil will be returned. This can be
    // checked by calling Available before calling Digester.
    fn digester(&self) -> Box<dyn DynDigest>;
    // Hash returns a new hash as used by the algorithm. If not available, the
    // method will panic. Check Algorithm.Available() before calling.
    fn hash(self) -> Box<dyn DynDigest>;
    // Encode encodes the raw bytes of a digest, typically from a hash.Hash, into
    // the encoded portion of the digest.
    fn encode(&self, _: &[u8]) -> String;

    fn from_reader<R: std::io::Read>(&self, reader: R) -> String;
}

/// A digest is a cryptographic hash of a data stream.
pub struct Algorithms<'a> {
    algorithms: HashMap<&'a str, isize>,
}

impl<'a> Algorithms<'a> {
    pub fn new() -> Self {
        let mut algs = Algorithms {
            algorithms: HashMap::new(),
        };
        algs.register_algorithm(SHA256, 256);
        algs.register_algorithm(SHA384, 384);
        algs.register_algorithm(SHA512, 512);
        algs
    }

    // Add an algorithm to the list of available algorithms.
    pub fn register_algorithm(self: &mut Self, name: &'a str, size: isize) -> bool {
        match self.algorithms.get(name) {
            Some(_) => false,
            None => {
                self.algorithms.insert(name, size);
                true
            }
        }
    }

    pub fn get_algorithm(self: &Self, name: &'a str) -> Option<Algorithm<'a>> {
        match self.algorithms.get(name) {
            Some(size) => Some(Algorithm::new(name, *size)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Algorithms, CryptoHash};

    #[test]
    fn encode() {
        let algs = Algorithms::new();
        let alg = algs.get_algorithm(super::SHA256).unwrap();
        assert_eq!(
            alg.encode(b"hello"),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
        assert_eq!(
            alg.encode(b"hello"),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn from_reader() {
        let algs = Algorithms::new();
        let alg = algs.get_algorithm(super::SHA256).unwrap();
        assert_eq!(
            alg.from_reader(b"hello".as_ref()),
            "fca9125f1d755424b55a18877213a746135c1ce0087f9471a6f98cdd4600df83"
        );
    }
}
