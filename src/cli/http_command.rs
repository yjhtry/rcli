use std::path::PathBuf;

use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{CmdExecutor, cli::verify_path, process_http_serve};

#[derive(Parser, Debug)]
#[enum_dispatch(CmdExecutor)]
pub enum HttpCommand {
    #[command(about = "Start http server")]
    Serve(ServeOpts),
}

#[derive(Debug, Parser)]
pub struct ServeOpts {
    // Static assets directory
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub path: PathBuf,

    #[arg(long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExecutor for ServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let port = self.port;
        process_http_serve(self.path, port)
            .await
            .map_err(|e| anyhow::anyhow!("Http serve {}", e))
    }
}
