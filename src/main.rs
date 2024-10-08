use std::{fs, path::Path};

use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[command(name="rcli",version,author,about,long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Deserialize, Serialize)]

struct Iris {
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

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Show CSV,or Convert CSV to JSON")]
    Csv(CsvOpts),
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist".into())
    }
}

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;
            let mut ret = Vec::with_capacity(300);
            for result in reader.deserialize::<Iris>() {
                let record: Iris = result?;
                ret.push(record);
            }

            let json = serde_json::to_string_pretty(&ret)?;
            fs::write(opts.output, json)?;
        }
    }
    Ok(())
}
