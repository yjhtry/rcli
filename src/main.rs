use clap::Parser;
use rcli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Csv(opts) => {
            opts.run()?;
        }
    }

    Ok(())
}
