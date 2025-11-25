use crate::{CmdExecutor, read_buffer_from_input, verify_file};
use anyhow::Context;
use clap::Parser;
use core::fmt;
use enum_dispatch::enum_dispatch;
use humantime::parse_duration;
use jwt_simple::{
    claims::Claims,
    prelude::{
        Ed25519KeyPair, Ed25519PublicKey, EdDSAKeyPairLike, EdDSAPublicKeyLike, HS256Key, HS512Key,
        MACLike,
    },
};
use std::{collections::HashSet, str::FromStr, time::Duration};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum JwtCommand {
    #[command(about = "Generate json web token")]
    Sign(JwtSignOpts),

    #[command(about = "Verify json web token")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    /// hs256, hs512, ed25519
    #[arg(long, value_parser = verify_format, default_value = "ed25519")]
    format: JwtSignFormat,

    #[arg(long)]
    sub: Option<String>,

    #[arg(long)]
    aud: Vec<String>,

    /// humantime such as 1d, 1w, 10s...
    #[arg(long, value_parser = verify_exp)]
    exp: Duration,

    /// Private key for sign token
    #[arg(long, value_parser = verify_file)]
    key: String,

    /// json string example: {\"name\": \"John\"}
    #[arg(long, value_parser = verify_json_string)]
    json: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy)]
pub enum JwtSignFormat {
    Ed25519,
    HS256,
    HS512,
}

impl CmdExecutor for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = read_buffer_from_input(&self.key)?;
        let custom_claims = self.json.unwrap_or(serde_json::Value::Null);
        let mut claims = Claims::with_custom_claims(custom_claims, self.exp.into());

        if self.sub.is_some() {
            claims = claims.with_subject(self.sub.unwrap());
        }
        if !self.aud.is_empty() {
            claims = claims.with_audiences(HashSet::from_iter(self.aud));
        }
        let token = match &self.format {
            JwtSignFormat::Ed25519 => {
                let key = Ed25519KeyPair::from_bytes(&key)?;
                key.sign(claims)?
            }
            JwtSignFormat::HS256 => {
                let key = HS256Key::from_bytes(&key);
                key.authenticate(claims)?
            }
            JwtSignFormat::HS512 => {
                let key = HS512Key::from_bytes(&key);
                key.authenticate(claims)?
            }
        };

        println!("{}", token);

        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(long, value_parser = verify_format, default_value = "ed25519")]
    format: JwtSignFormat,

    /// Verify key for parse token
    #[arg(long, value_parser = verify_file)]
    key: String,

    #[arg(long)]
    token: String,
}

impl CmdExecutor for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = read_buffer_from_input(&self.key)?;
        let claims = match &self.format {
            JwtSignFormat::HS256 => {
                let key = HS256Key::from_bytes(&key);
                key.verify_token::<serde_json::Value>(&self.token, None)?
            }
            JwtSignFormat::HS512 => {
                let key = HS512Key::from_bytes(&key);
                key.verify_token::<serde_json::Value>(&self.token, None)?
            }
            JwtSignFormat::Ed25519 => {
                let pk = Ed25519PublicKey::from_bytes(&key)?;
                pk.verify_token::<serde_json::Value>(&self.token, None)?
            }
        };

        println!("{:?}", claims);

        Ok(())
    }
}

impl FromStr for JwtSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ed25519" => Ok(JwtSignFormat::Ed25519),
            "hs256" => Ok(JwtSignFormat::HS256),
            "hs512" => Ok(JwtSignFormat::HS512),
            _ => Err(anyhow::anyhow!("Invalid jwt sign format")),
        }
    }
}

impl fmt::Display for JwtSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JwtSignFormat::Ed25519 => write!(f, "ed25519"),
            JwtSignFormat::HS256 => write!(f, "hs256"),
            JwtSignFormat::HS512 => write!(f, "hs512"),
        }
    }
}

impl From<JwtSignFormat> for &'static str {
    fn from(value: JwtSignFormat) -> Self {
        match value {
            JwtSignFormat::Ed25519 => "ed25519",
            JwtSignFormat::HS256 => "hs256",
            JwtSignFormat::HS512 => "hs512",
        }
    }
}

fn verify_format(format: &str) -> anyhow::Result<JwtSignFormat> {
    format.parse()
}

fn verify_exp(duration: &str) -> anyhow::Result<Duration> {
    Ok(parse_duration(duration)?)
}

fn verify_json_string(data: &str) -> anyhow::Result<serde_json::Value> {
    serde_json::from_str::<serde_json::Value>(data).context("Please input invalid json string")
}
