use core::fmt;
use std::str::FromStr;

use clap::Parser;

use crate::cli::verify_file;

#[derive(Parser, Debug)]
pub enum TextCommand {
    #[command(about = "Sign text with private/shared key")]
    Sign(SignTextOpts),
    #[command(about = "Verify text with sign")]
    Verify(VerifyTextOpts),
}

#[derive(Debug, Parser)]
pub struct SignTextOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file)]
    pub key: String,

    #[arg(long, value_parser = verify_format, default_value = "Blake3")]
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

    #[arg(long, value_parser = verify_format, default_value = "Blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Clone)]
pub enum TextSignFormat {
    Blake3,
    ED25519,
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::ED25519),
            _ => Err(anyhow::anyhow!("Invalid text sign format")),
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextSignFormat::Blake3 => write!(f, "Blake3"),
            TextSignFormat::ED25519 => write!(f, "ed25519"),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Blake3 => "Blake3",
            TextSignFormat::ED25519 => "ed25519",
        }
    }
}

fn verify_format(format: &str) -> Result<TextSignFormat, String> {
    format.parse().map_err(|e: anyhow::Error| e.to_string())
}
