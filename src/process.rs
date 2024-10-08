use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Iris {
    #[serde(rename = "Sepal.Length")]
    sepal_length: String,
    #[serde(rename = "Sepal.Width")]
    sepal_width: String,
    #[serde(rename = "Petal.Length")]
    setal_length: String,
    #[serde(rename = "Petal.Width")]
    setal_width: String,
    #[serde(rename = "Species")]
    species: String,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(300);
    for result in reader.deserialize::<Iris>() {
        let record: Iris = result?;
        ret.push(record);
    }

    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}
