use aes_gcm::{Aes256Gcm, Key, KeyInit, aead::Aead, aes::Aes256};
use anyhow::{Result, anyhow};
use base64::{Engine, prelude::BASE64_URL_SAFE};
use clap::{Parser, arg, command};
use encoding::Encoding;

mod encoding;

/// Cli tool for quick prisma-field-encryption column decryption
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// aka PRISMA_FIELD_ENCRYPTION_KEY \n (k1.<cipher>.<key>) \n IF OMITTED, USES PRISMA_FIELD_ENCRYPTION_KEY ENV VAR
    #[arg(short, long)]
    #[arg(help = "Prisma AES-GCM 256-bit key (aka `PRISMA_FIELD_ENCRYPTION_KEY`)")]
    #[arg(
        long_help = "Prisma AES-GCM 256-bit key (aka `PRISMA_FIELD_ENCRYPTION_KEY`) \nIf omitted, uses `PRISMA_FIELD_ENCRYPTION_KEY` environment variable"
    )]
    #[arg(value_name = "k1.cipher.key")]
    prisma_key: Option<String>,

    /// Specify output encoding (e.g. `utf-8`, `base64`, `hex`)
    #[arg(long, default_value_t = Encoding::Utf8)]
    encoding: Encoding,

    /// Encrypted column data (v1.<nonce>.<data>.<auth_tag>)
    #[arg(value_name = "v1.nonce.data.auth_tag")]
    data: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let aes_gcm_prisma_key = {
        if let Some(key) = args.prisma_key {
            key
        } else {
            std::env::var("PRISMA_FIELD_ENCRYPTION_KEY")?
        }
    };
    let encrypted_data = args.data;

    let b64_key = aes_gcm_prisma_key
        .split(".")
        .last()
        .ok_or(anyhow!("invalid key {}", aes_gcm_prisma_key))?;

    let decoded_key: Vec<u8> = BASE64_URL_SAFE.decode(b64_key)?;
    let aes_gcm_prisma_key = Key::<Aes256>::from_slice(&decoded_key);

    let mut parts: [Vec<u8>; 2] = [vec![], vec![]];
    for (i, p) in encrypted_data.split(".").skip(3).enumerate() {
        parts[i] = BASE64_URL_SAFE.decode(p)?;
    }

    let [nonce, ciphertext] = parts;
    let cipher = Aes256Gcm::new(&aes_gcm_prisma_key);

    let decrypted = cipher
        .decrypt(nonce.as_slice().into(), ciphertext.as_slice())
        .map_err(|e| anyhow!("failed to decrypt data with aead err: {}", e.to_string()))?;

    let decoded = args.encoding.encode(&decrypted)?;
    println!("{}", decoded);

    Ok(())
}
