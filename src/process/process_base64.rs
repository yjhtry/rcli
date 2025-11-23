use anyhow::{Context, Result};
use base64::engine::general_purpose::URL_SAFE;
use base64::prelude::*;

use crate::{Base64Format, read_buffer_from_input};

pub fn process_base64_encode(input: &str, format: Base64Format) -> Result<()> {
    let buf = read_buffer_from_input(input)?;
    let result = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE.encode(buf),
    };

    print!("{}", result);
    Ok(())
}

pub fn process_base64_decode(input: &str, format: Base64Format) -> Result<Vec<u8>> {
    let buf = read_buffer_from_input(input)?;
    let result = match format {
        Base64Format::Standard => BASE64_STANDARD
            .decode(buf)
            .context("Decode input base64 failed")?,
        Base64Format::UrlSafe => URL_SAFE.decode(buf).context("Decode input base64 failed")?,
    };

    Ok(result)
}

#[cfg(test)]
mod test {
    use std::fs;

    use base64::prelude::*;
    #[test]
    fn process_base64_encode_decode() {
        let buf = fs::read("Cargo.toml").unwrap();
        let base64_content = BASE64_STANDARD.encode(&buf);
        assert_eq!(buf, BASE64_STANDARD.decode(base64_content).unwrap());
    }
}
