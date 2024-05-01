mod csv;
mod genpass;

use clap::Parser;
use csv::CsvOpts;
use genpass::GenPassOpts;

pub use csv::OutputFormat;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert CSV To Other Format")]
    CSV(CsvOpts),

    #[command(name = "genpass", about = "Generate a random password")]
    GENPASS(GenPassOpts),
}

fn verify_input_file(file_name: &str) -> Result<String, String> {
    if std::path::Path::new(file_name).exists() {
        Ok(file_name.to_string())
    } else {
        Err(format!("File not found: {}", file_name))
    }
}
