use std::str::FromStr;

use anyhow::{Result, bail};

use base64::{Engine, prelude::BASE64_STANDARD};
use hex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Base64,
    Utf8,
    Hex,
}

impl FromStr for Encoding {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "base64" => Encoding::Base64,
            "utf-8" => Encoding::Utf8,
            "hex" => Encoding::Hex,
            _ => bail!("invalid encoding {}", s),
        })
    }
}

impl ToString for Encoding {
    fn to_string(&self) -> String {
        match self {
            Encoding::Base64 => "base64".to_string(),
            Encoding::Utf8 => "utf-8".to_string(),
            Encoding::Hex => "hex".to_string(),
        }
    }
}

impl Encoding {
    pub fn encode(&self, data: &[u8]) -> Result<String> {
        match self {
            Encoding::Utf8 => Ok(String::from_utf8(data.to_vec())?),
            Encoding::Base64 => Ok(BASE64_STANDARD.encode(data)),
            Encoding::Hex => Ok(hex::encode(data)),
        }
    }
}
