mod cli;
mod process;

pub use cli::{Base64Command, Base64Format, Cli, Commands, CsvOpts, OutputFormat};
pub use process::process_csv;
pub use process::process_gen_pass;
pub use process::{process_base64_decode, process_base64_encode};
