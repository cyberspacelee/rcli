// rcli csv -i input.csv -o output.csv --header -d ','
use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::CSV(opts) => process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}
