use std::fs::File;
use std::io::Read;

use anyhow::Context;
use base64::engine::general_purpose::URL_SAFE;
use base64::prelude::*;

use crate::Base64Format;

fn read_buffer_from_input(input: &str) -> anyhow::Result<Vec<u8>> {
    let is_stdin = input == "-";
    let mut reader: Box<dyn Read> = if is_stdin {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input).with_context(|| format!("Open file: {input} failed"))?)
    };
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    // When user input in terminal will keydown `Enter + Ctrl + d`, so trime the end \n
    if is_stdin && buf.ends_with(b"\n") {
        buf.pop();
    }
    Ok(buf)
}

pub fn process_base64_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let buf = read_buffer_from_input(input)?;
    let result = match format {
        Base64Format::Standard => BASE64_STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE.encode(buf),
    };

    print!("{}", result);
    Ok(())
}

pub fn process_base64_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let buf = read_buffer_from_input(input)?;
    let result = match format {
        Base64Format::Standard => BASE64_STANDARD
            .decode(buf)
            .context("Decode input base64 failed")?,
        Base64Format::UrlSafe => URL_SAFE.decode(buf).context("Decode input base64 failed")?,
    };

    // TODO: decode output maybe not string, but for this case assume it is string
    print!("{}", String::from_utf8_lossy(&result));
    Ok(())
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
