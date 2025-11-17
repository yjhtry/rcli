use std::path::Path;

use clap::Parser;

mod base64_command;
mod csv_opts;
mod gen_pass_opts;

pub use base64_command::{Base64Command, Base64Format};
pub use csv_opts::{CsvOpts, OutputFormat};
pub use gen_pass_opts::GenPassOpts;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    #[command(name = "csv", about = "Convert csv to json, yaml, toml format")]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),

    #[command(subcommand)]
    Base64(Base64Command),
}

pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("Input file not exist")
    }
}

#[cfg(test)]
mod test {
    use crate::cli::verify_input_file;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(
            verify_input_file("File not exist"),
            Err("Input file not exist")
        );
    }
}
