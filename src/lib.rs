mod cli;
mod process;
mod utils;

pub use cli::*;
use enum_dispatch::enum_dispatch;
pub use process::process_csv;
pub use process::process_gen_pass;
pub use process::{
    check_password_strength, process_base64_decode, process_base64_encode, process_http_serve,
    process_key_generate, process_text_decrypt, process_text_encrypt, process_text_sign,
    process_text_verify,
};
pub use utils::read_buffer_from_input;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
