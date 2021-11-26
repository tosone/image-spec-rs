use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Result;
use std::string::String;
use std::sync::{Arc, Mutex};

// CANONICAL is the primary digest algorithm used with the distribution
// project. Other digests may be used but this one is the primary storage
// digest.
pub const CANONICAL: &str = super::sha::SHA256;

// BLAKE3 is the blake3 algorithm with the default 256-bit output size
// github.com/opencontainers/go-digest/blake3 should be imported to make it available
pub const BLAKE3: &str = "blake3";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Algorithm {
    pub name: String,
    // #[serde(skip)]
    // pub alg: dyn CryptoHash,
}

impl Algorithm {
    pub fn new(name: &str) -> Algorithm {
        Algorithm {
            name: name.to_string(),
            // alg: match name {
            //     CANONICAL => Box::new(super::sha::Sha256::new()),
            //     _ => Box::new(super::sha::Sha256::new()),
            // },
        }
    }

    pub fn available(algorithm: &Algorithm) -> bool {
        return match algorithm.name.as_str() {
            super::sha::SHA256 => true,
            super::sha::SHA384 => true,
            super::sha::SHA512 => true,
            BLAKE3 => true,
            _ => false,
        };
    }

    pub fn string(self: &Self) -> String {
        self.name.clone()
    }

    pub fn size(self: &Self) -> isize {
        0
    }
}

/// CryptoHash is the interface that any hash algorithm must implement
pub trait CryptoHash {
    // Available reports whether the given hash function is usable in the current binary.
    fn available(&self) -> bool;
    // Size returns the length, in bytes, of a digest resulting from the given hash function.
    fn size(&self) -> usize;
}

/// A digest is a cryptographic hash of a data stream.
pub struct Algorithms {
    algorithms: Arc<Mutex<HashMap<String, Box<dyn CryptoHash + 'static>>>>,
    regex: Arc<Mutex<HashMap<String, regex::Regex>>>,
}

impl Algorithms {
    pub fn new() -> Self {
        let algs = Algorithms {
            algorithms: Arc::new(Mutex::new(HashMap::new())),
            regex: Arc::new(Mutex::new(HashMap::new())),
        };

        let sha256 = super::sha::Sha256::new();
        let sha256_size = sha256.size() * 2;
        algs.register_algorithm(String::from(super::sha::SHA256), sha256);
        algs.register_algorithm_regex(
            String::from(super::sha::SHA256),
            format!("^[a-f0-9]{}$", sha256_size),
        );

        let sha384 = super::sha::Sha384::new();
        let sha384_size = sha384.size() * 2;
        algs.register_algorithm(String::from(super::sha::SHA384), sha384);
        algs.register_algorithm_regex(
            String::from(super::sha::SHA384),
            format!("^[a-f0-9]{}$", sha384_size),
        );

        let sha512 = super::sha::Sha512::new();
        let sha512_size = sha512.size() * 2;
        algs.register_algorithm(String::from(super::sha::SHA512), sha512);
        algs.register_algorithm_regex(
            String::from(super::sha::SHA512),
            format!("^[a-f0-9]{}$", sha512_size),
        );
        algs
    }

    pub fn get(self: &Self, name: &str) -> &Box<dyn CryptoHash> {
        let algorithms = Arc::clone(&self.algorithms);
        let algorithms_lock = algorithms.lock().unwrap();
        let algorithm = algorithms_lock.get(name);
        let alg = algorithm.map(|alg| alg.clone());
        alg.unwrap()
    }

    /// Add an algorithm to the list of available algorithms.
    pub fn register_algorithm<T: 'static>(self: &Self, name: String, alg: T) -> bool
    where
        T: CryptoHash,
    {
        let mut algorithms = self.algorithms.lock().unwrap();
        return match algorithms.get(&name) {
            Some(_) => false,
            None => {
                algorithms.insert(name, Box::new(alg));
                true
            }
        };
    }

    pub fn register_algorithm_regex(self: &Self, name: String, regex: String) -> bool {
        let mut regexes = self.regex.lock().unwrap();
        return match regexes.get(&name) {
            Some(_) => false,
            None => {
                let re = Regex::new(&regex).unwrap();
                regexes.insert(name, re);
                true
            }
        };
    }
}
