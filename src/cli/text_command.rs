use core::fmt;
use std::{fs, path::PathBuf, str::FromStr};

use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{
    CmdExecutor,
    cli::{verify_file, verify_path},
    process_key_generate, process_text_decrypt, process_text_encrypt, process_text_sign,
    process_text_verify,
};

#[derive(Parser, Debug)]
#[enum_dispatch(CmdExecutor)]
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

impl CmdExecutor for SignTextOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let sign = process_text_sign(&self.input, &self.key, self.format)?;
        print!("{}", sign);

        Ok(())
    }
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

impl CmdExecutor for VerifyTextOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let result = process_text_verify(&self.input, &self.key, self.format, self.sign)?;
        println!("{}", result);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct GenerateOpts {
    // Generate sign/verify or encrypt/decrypt key
    #[arg(long, value_parser = verify_format, default_value = "blake3")]
    pub format: TextSignFormat,

    #[arg(short, long, value_parser = verify_path, default_value = "fixtures")]
    pub output: PathBuf,
}

impl CmdExecutor for GenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let result = process_key_generate(self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                assert_eq!(result.len(), 1, "Generate Blake3 key failed");
                let path = self.output.join("blake3.txt");
                fs::write(&path, &result[0])?;
            }
            TextSignFormat::ED25519 => {
                assert_eq!(result.len(), 2, "Generate ED25519 key failed");
                let pk_path = self.output.join("ed25519.pk");
                let sk_path = self.output.join("ed25519.sk");
                fs::write(sk_path, &result[0])?;
                fs::write(pk_path, &result[1])?;
            }
            TextSignFormat::ChaCha20Poly1305 => {
                assert_eq!(result.len(), 1, "Generate ChaCha20Poly1305 key failed");
                let path = self.output.join("chacha20poly1305.txt");
                fs::write(&path, &result[0])?;
            }
        };

        Ok(())
    }
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

impl CmdExecutor for EncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ciphertext = process_text_encrypt(&self.input, &self.key)?;
        fs::write(self.output, ciphertext)?;
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct DecryptOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
}

impl CmdExecutor for DecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let plaintext = process_text_decrypt(&self.input, &self.key)?;
        print!("{}", String::from_utf8(plaintext)?);
        Ok(())
    }
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
