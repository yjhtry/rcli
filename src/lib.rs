mod cli;
mod process;

pub use cli::{Cli, Commands, CsvOpts, OutputFormat};
pub use process::process_csv;
pub use process::process_gen_pass;

#[cfg(test)]
mod test {
    #[test]
    fn name() {
        // noop
    }
}
