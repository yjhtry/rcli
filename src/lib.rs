mod opts;
mod process;

pub use opts::{Cli, Commands, CsvOpts, OutputFormat};
pub use process::LangTrend;

#[cfg(test)]
mod test {
    #[test]
    fn name() {
        // noop
    }
}
