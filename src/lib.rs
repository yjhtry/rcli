mod cli;
mod process;
mod utils;

pub use cli::{
    Base64Command, Base64Format, Cli, Commands, CsvOpts, OutputFormat, TextCommand, TextSignFormat,
};
pub use process::process_csv;
pub use process::process_gen_pass;
pub use process::{
    process_base64_decode, process_base64_encode, process_text_sign, process_text_verify,
};
pub use utils::read_buffer_from_input;
