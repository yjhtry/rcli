use clap::Parser;

use crate::{CmdExecutor, check_password_strength, process_gen_pass};

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: usize,

    /// Include numbers
    #[arg(short, long = "number")]
    #[arg(long = "no-number", overrides_with = "number", action = clap::ArgAction::SetFalse)]
    pub number: bool,

    /// Include lowercase letters
    #[arg(long = "lower")]
    #[arg(long = "no-lower", overrides_with = "lower", action = clap::ArgAction::SetFalse)]
    pub lower: bool,

    /// Include uppercase letters
    #[arg(short, long = "upper")]
    #[arg(long = "no-upper", overrides_with = "upper", action = clap::ArgAction::SetFalse)]
    pub upper: bool,

    /// Include symbols
    #[arg(short, long = "symbol")]
    #[arg(long = "no-symbol", overrides_with = "symbol", action = clap::ArgAction::SetFalse)]
    pub symbol: bool,
}

impl CmdExecutor for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password = process_gen_pass(
            self.length,
            self.number,
            self.lower,
            self.upper,
            self.symbol,
        )?;
        check_password_strength(&password);
        Ok(())
    }
}
