use std::{fs::File, io::Write};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{CsvOpts, OutputFormat};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct LangTrend {
    pub name: String,
    pub kind: String,
    pub birthday: String,
    pub trending: String,
}

impl CsvOpts {
    pub fn run(&self) -> anyhow::Result<()> {
        let output_path = self.get_output_path();
        let input_file = File::open(&self.input).context("Open input file failed")?;
        let mut rdr = csv::Reader::from_reader(input_file);
        let records: Vec<LangTrend> = rdr
            .deserialize()
            .collect::<Result<Vec<_>, _>>()
            .with_context(|| format!("Read csv file {} failed", &self.input))?;

        let mut output_file = File::create(&output_path)
            .with_context(|| format!("Open outoput file {} failed", &output_path))?;
        let output_content = match &self.format {
            OutputFormat::JSON => {
                serde_json::to_string_pretty(&records).context("Serialize failed")?
            }
            OutputFormat::YAML => serde_yaml::to_string(&records).context("Serialize failed")?,
        };

        output_file
            .write_all(output_content.as_bytes())
            .with_context(|| format!("Write records to {} failed", &output_path))?;

        Ok(())
    }

    fn get_output_path(&self) -> String {
        match &self.output {
            Some(path) => path.to_string(),
            None => format!("{}.{}", "output", self.format),
        }
    }
}
