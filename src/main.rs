use clap::Parser;
use rcli::{Cli, Commands, process_csv, process_gen_pass};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Csv(opts) => {
            process_csv(
                &opts.input,
                opts.output.as_deref(),
                opts.format,
                opts.delimiter,
            )?;
        }
        Commands::GenPass(opts) => {
            let password = process_gen_pass(
                opts.length,
                opts.number,
                opts.lower,
                opts.upper,
                opts.symbol,
            );

            println!("{}", password);
        }
    }

    Ok(())
}
