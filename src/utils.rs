use std::{fs::File, io::Read};

use anyhow::Context;

pub fn read_buffer_from_input(input: &str) -> anyhow::Result<Vec<u8>> {
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
