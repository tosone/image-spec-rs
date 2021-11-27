use serde::{Deserialize, Serialize};
use std::string::String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Digest {
    pub name: String,
    pub digest: String,
}

impl Digest {
    pub fn new(alg: super::algorithm::Algorithm, digest: &str) -> Self {
        let name = alg.name.to_string();
        let digest = format!("{}:{}", name, digest.to_string());
        Self { name, digest }
    }

    pub fn new_from_bytes(alg: super::algorithm::Algorithm, bytes: &[u8]) -> Self {
        let name = alg.name.to_string();
        let digest = format!("{}:{}", name, String::from_utf8(bytes.to_vec()).unwrap());
        Self { name, digest }
    }

    pub fn string(self: Self) -> String {
        self.digest.to_string()
    }

    pub fn algorithm(self: &Self) -> String {
        self.digest[..self.sep_index()].to_string()
    }

    pub fn encoded(self: Self) -> String {
        self.digest[self.sep_index() + 1..].to_string()
    }

    fn sep_index(&self) -> usize {
        self.digest.find(':').unwrap()
    }

    pub fn validate(self: &Self) -> Result<(), std::io::Error> {
        let alg = self.algorithm();
        match alg.as_str() {
            super::algorithm::SHA256 | super::algorithm::SHA384 | super::algorithm::SHA512 => {}
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "invalid checksum digest algorithm",
                ));
            }
        }
        let re = regex::Regex::new(r"^[a-z0-9]+(?:[.+_-][a-z0-9]+)*:[a-zA-Z0-9=_-]+$").unwrap();
        if re.is_match(&self.digest) {
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid checksum digest format",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        let d = Digest {
            name: "sha256".to_string(),
            digest: "sha256:abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
        };
        assert!(d.validate().is_ok());
    }
}
