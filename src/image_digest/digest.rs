#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Digest {
    pub str: std::string::String,
}

impl Digest {
    pub fn new(s: &str) -> Digest {
        Digest { str: s.to_string() }
    }

    pub fn validate(d: Digest) -> Result<(), std::io::Error> {
        let s = d.str.as_str();
        if s.len() != 64 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "digest length is not 64",
            ));
        }
        match s.find(':') {
            Some(i) => {
                if i <= 0 || i + 1 == s.len() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "invalid checksum digest format",
                    ));
                }
            }
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "invalid checksum digest format",
                ));
            }
        }
        Ok(())
    }
}
