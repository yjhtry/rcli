use std::{fs::File, io::Write};

use anyhow::Context;
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::OutputFormat;

// No use, only show serde lib deserialize and seridelize
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
struct LangTrend {
    pub name: String,
    pub kind: String,
    pub birthday: String,
    pub trending: String,
}

pub fn process_csv(
    input: &str,
    output: Option<&str>,
    format: OutputFormat,
    delimiter: char,
) -> anyhow::Result<()> {
    let input_file = File::open(input).context("Open input file failed")?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .from_reader(&input_file);
    let headers = rdr.headers()?.clone();
    let records = rdr.records().collect::<Result<Vec<_>, _>>()?;
    let json_list = records
        .iter()
        .map(|record| headers.iter().zip(record.iter()).collect::<Value>())
        .collect::<Vec<_>>();

    let output_path = match output {
        Some(path) => path.to_string(),
        None => format!("{}.{}", "output", format),
    };

    let mut output_file = File::create(&output_path)
        .with_context(|| format!("Open outoput file {} failed", &output_path))?;
    let output_content = match format {
        OutputFormat::JSON => {
            serde_json::to_string_pretty(&json_list).context("Serialize failed")?
        }
        OutputFormat::YAML => serde_yaml::to_string(&json_list).context("Serialize failed")?,
        OutputFormat::TOML => {
            toml::to_string(&json!({"data": &json_list})).context("Serialize failed")?
        }
    };

    output_file
        .write_all(output_content.as_bytes())
        .with_context(|| format!("Write records to {} failed", &output_path))?;

    Ok(())
}
