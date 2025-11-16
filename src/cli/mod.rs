use clap::Parser;

mod csv_opts;
mod gen_pass_opts;

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
}
