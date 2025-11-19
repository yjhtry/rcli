use core::fmt;
use std::{path::PathBuf, str::FromStr};

use clap::Parser;

use crate::cli::{verify_file, verify_path};

#[derive(Parser, Debug)]
pub enum TextCommand {
    #[command(about = "Sign text with private/shared key")]
    Sign(SignTextOpts),

    #[command(about = "Verify text with sign")]
    Verify(VerifyTextOpts),

    #[command(about = "Generate sign/verify key or encrypt/decrypt key")]
    Generate(GenerateOpts),

    #[command(about = "Encrypt text")]
    Encrypt(EncryptOpts),

    #[command(about = "Decrypt encrypted text")]
    Decrypt(DecryptOpts),
}

#[derive(Debug, Parser)]
pub struct SignTextOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file)]
    pub key: String,

    #[arg(long, value_parser = verify_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct VerifyTextOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file)]
    pub key: String,

    #[arg(short, long)]
    pub sign: String,

    #[arg(long, value_parser = verify_format, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct GenerateOpts {
    // Generate sign/verify or encrypt/decrypt key
    #[arg(long, value_parser = verify_format, default_value = "blake3")]
    pub format: TextSignFormat,

    #[arg(short, long, value_parser = verify_path, default_value = "fixtures")]
    pub output: PathBuf,
}

#[derive(Debug, Parser)]
pub struct EncryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    // Cipher content output file
    #[arg(short, long)]
    pub output: String,

    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct DecryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    ED25519,
    ChaCha20Poly1305,
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::ED25519),
            "chacha20poly1305" => Ok(TextSignFormat::ChaCha20Poly1305),
            _ => Err(anyhow::anyhow!("Invalid text sign format")),
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextSignFormat::Blake3 => write!(f, "blake3"),
            TextSignFormat::ED25519 => write!(f, "ed25519"),
            TextSignFormat::ChaCha20Poly1305 => write!(f, "chacha20poly1305"),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::ED25519 => "ed25519",
            TextSignFormat::ChaCha20Poly1305 => "chacha20poly1305",
        }
    }
}

fn verify_format(format: &str) -> Result<TextSignFormat, String> {
    format.parse().map_err(|e: anyhow::Error| e.to_string())
}
