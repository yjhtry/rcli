use clap::Parser;
use std::{fmt::Display, str::FromStr};

use crate::cli::verify_file;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    JSON,
    YAML,
    TOML,
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    /// "Support json, yaml, toml"
    #[arg(short, long, default_value = "json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::JSON => f.write_str("json"),
            OutputFormat::YAML => f.write_str("yaml"),
            OutputFormat::TOML => f.write_str("toml"),
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::JSON),
            "yaml" => Ok(OutputFormat::YAML),
            "toml" => Ok(OutputFormat::TOML),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::JSON => "json",
            OutputFormat::YAML => "yaml",
            OutputFormat::TOML => "toml",
        }
    }
}
