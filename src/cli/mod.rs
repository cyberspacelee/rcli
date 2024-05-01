mod base64;
mod csv;
mod genpass;

use std::path::Path;

pub use base64::{Base64Format, Base64SubCommand};
use clap::Parser;
use csv::CsvOpts;
pub use csv::OutputFormat;
use genpass::GenPassOpts;

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

    #[command(subcommand)]
    BASE64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_input_file("not-exist"), Err("File does not exist"));
    }
}
