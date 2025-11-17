use clap::Parser;
use rcli::{
    Cli, Commands, process_base64_decode, process_base64_encode, process_csv, process_gen_pass,
};

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
            )?;
            println!("{}", password);
        }
        Commands::Base64(base64_command) => match base64_command {
            rcli::Base64Command::Decode(decode_opts) => {
                process_base64_decode(&decode_opts.input, decode_opts.format)?;
            }
            rcli::Base64Command::Encode(encode_opts) => {
                process_base64_encode(&encode_opts.input, encode_opts.format)?;
            }
        },
    }

    Ok(())
}
