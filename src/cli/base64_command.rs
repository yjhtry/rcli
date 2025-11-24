use core::fmt;
use std::str::FromStr;

use anyhow::anyhow;
use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{CmdExecutor, cli::verify_file, process_base64_decode, process_base64_encode};

#[derive(Parser, Debug)]
#[enum_dispatch(CmdExecutor)]
pub enum Base64Command {
    #[command(about = "Decode base64 to output")]
    Decode(DecodeOpts),
    #[command(about = "Encode input to base64")]
    Encode(EncodeOpts),
}

#[derive(Debug, Parser)]
pub struct DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(long, value_parser = verify_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExecutor for DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let result = process_base64_decode(&self.input, self.format)?;

        // TODO: decode output maybe not string, but for this case assume it is string
        print!("{}", String::from_utf8_lossy(&result));
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct EncodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(long, value_parser = verify_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExecutor for EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_base64_encode(&self.input, self.format)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "url_safe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow!("Invalid base64 format")),
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Base64Format::Standard => write!(f, "standard"),
            Base64Format::UrlSafe => write!(f, "url_safe"),
        }
    }
}

impl From<Base64Format> for &str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "url_safe",
        }
    }
}

fn verify_base64_format(format: &str) -> Result<Base64Format, String> {
    format.parse().map_err(|e: anyhow::Error| e.to_string())
}
