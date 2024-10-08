use clap::Parser;

#[derive(Debug,Parser)]
#[command(name="rcli",version,author,about,long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug,Parser)]
struct CsvOpts{
    #[arg(short,long,default_value = "string")]
    input: String,

    #[arg(short,long,default_value = "output.json")]
    output: String,

    #[arg(short,long,default_value_t = ',')]
    delimiter: char,

    #[arg(long,default_value_t = true)]
    header: bool,
}

#[derive(Debug,Parser)]
enum SubCommand {
    #[command(name="csv",about = "Show CSV,or Convert CSV to JSON")]
    Csv(CsvOpts),
}

fn main() {
    match Opts::try_parse() {
        Ok(opts) => {
            println!("{:?}", opts);
        }
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            std::process::exit(1);
        }
    }
}