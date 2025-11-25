use std::path::{Path, PathBuf};

use clap::Parser;

mod base64_command;
mod csv_opts;
mod gen_pass_opts;
mod http_command;
mod jwt_command;
mod text_command;

pub use base64_command::*;
pub use csv_opts::*;
use enum_dispatch::enum_dispatch;
pub use gen_pass_opts::*;
pub use http_command::*;
pub use jwt_command::*;
pub use text_command::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
#[enum_dispatch(CmdExecutor)]
pub enum Commands {
    #[command(name = "csv", about = "Convert csv to json, yaml, toml format")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),

    #[command(subcommand, about = "Encode/Decode base64")]
    Base64(Base64Command),

    #[command(subcommand, about = "Text encrypt/decrypt/sign/verify")]
    Text(TextCommand),

    #[command(subcommand, about = "Generate a random password")]
    Http(HttpCommand),

    #[command(subcommand, about = "Jwt sign and verify")]
    Jwt(JwtCommand),
}

pub fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("Input file not exist")
    }
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let path = Path::new(path);
    if path.exists() && path.is_dir() {
        Ok(path.into())
    } else {
        Err("Input file not exist")
    }
}

#[cfg(test)]
mod test {
    use crate::cli::verify_file;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("File not exist"), Err("Input file not exist"));
    }
}
