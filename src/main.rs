use clap::Parser;
use rcli::{
    Cli, Commands, HttpCommand, TextCommand, TextSignFormat, check_password_strength,
    process_base64_decode, process_base64_encode, process_csv, process_gen_pass,
    process_http_serve, process_key_generate, process_text_decrypt, process_text_encrypt,
    process_text_sign, process_text_verify,
};
use std::fs::{self};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    tracing_subscriber::fmt().init();

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
                let result = process_base64_decode(&opts.input, opts.format)?;

                // TODO: decode output maybe not string, but for this case assume it is string
                print!("{}", String::from_utf8_lossy(&result));
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
                let result = process_key_generate(opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        assert_eq!(result.len(), 1, "Generate Blake3 key failed");
                        let path = opts.output.join("blake3.txt");
                        fs::write(&path, &result[0])?;
                    }
                    TextSignFormat::ED25519 => {
                        assert_eq!(result.len(), 2, "Generate ED25519 key failed");
                        let pk_path = opts.output.join("ed25519.pk");
                        let sk_path = opts.output.join("ed25519.sk");
                        fs::write(sk_path, &result[0])?;
                        fs::write(pk_path, &result[1])?;
                    }
                    TextSignFormat::ChaCha20Poly1305 => {
                        assert_eq!(result.len(), 1, "Generate ChaCha20Poly1305 key failed");
                        let path = opts.output.join("chacha20poly1305.txt");
                        fs::write(&path, &result[0])?;
                    }
                };
            }
            TextCommand::Encrypt(opts) => {
                let ciphertext = process_text_encrypt(&opts.input, &opts.key)?;
                fs::write(opts.output, ciphertext)?;
            }
            TextCommand::Decrypt(opts) => {
                let plaintext = process_text_decrypt(&opts.input, &opts.key)?;
                print!("{}", String::from_utf8(plaintext)?);
            }
        },

        Commands::Http(http_command) => match http_command {
            HttpCommand::Serve(opts) => {
                let port = opts.port;
                process_http_serve(opts.path, port).await?;
            }
        },
    }

    Ok(())
}
