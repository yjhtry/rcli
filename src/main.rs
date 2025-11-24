use clap::Parser;
use rcli::{Cli, CmdExecutor};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    tracing_subscriber::fmt().init();

    cli.command.execute().await
}
