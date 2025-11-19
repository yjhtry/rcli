use clap::Parser;
use rcli::{
    Cli, Commands, TextCommand, check_password_strength, process_base64_decode,
    process_base64_encode, process_csv, process_gen_pass, process_generate, process_text_sign,
    process_text_verify,
};
use std::fs::{self};

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

            check_password_strength(&password);
        }
        Commands::Base64(base64_command) => match base64_command {
            rcli::Base64Command::Decode(opts) => {
                process_base64_decode(&opts.input, opts.format)?;
            }
            rcli::Base64Command::Encode(opts) => {
                process_base64_encode(&opts.input, opts.format)?;
            }
        },
        Commands::Text(text_command) => match text_command {
            TextCommand::Sign(opts) => {
                let sign = process_text_sign(&opts.input, &opts.key, opts.format)?;
                print!("{}", sign);
            }
            TextCommand::Verify(opts) => {
                let result = process_text_verify(&opts.input, &opts.key, opts.format, opts.sign)?;
                println!("{}", result)
            }
            TextCommand::Generate(opts) => {
                let result = process_generate(opts.format)?;
                match opts.format {
                    rcli::TextSignFormat::Blake3 => {
                        assert_eq!(result.len(), 1, "Generate Blake3 key failed");
                        let path = opts.output.join("blake3.txt");
                        fs::write(&path, &result[0])?;
                    }
                    rcli::TextSignFormat::ED25519 => {
                        assert_eq!(result.len(), 2, "Generate ED25519 key failed");
                        let pk_path = opts.output.join("ed25519.pk");
                        let sk_path = opts.output.join("ed25519.sk");
                        fs::write(sk_path, &result[0])?;
                        fs::write(pk_path, &result[1])?;
                    }
                };
            }
        },
    }

    Ok(())
}
