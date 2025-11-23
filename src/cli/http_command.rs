use std::path::PathBuf;

use clap::Parser;

use crate::cli::verify_path;

#[derive(Parser, Debug)]
pub enum HttpCommand {
    #[command(about = "Start http server")]
    Serve(ServeOpts),
}

#[derive(Debug, Parser)]
pub struct ServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub path: PathBuf,

    #[arg(long, default_value_t = 8080)]
    pub port: u16,
}
