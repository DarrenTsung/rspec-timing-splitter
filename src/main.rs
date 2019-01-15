use std::fs::{self, File};
use std::io::Write;
use structopt::StructOpt;

mod opt;
mod timings;

use crate::opt::Opt;

fn main() -> Result<(), failure::Error> {
    let opt = Opt::from_args();
    match opt {
        Opt::Parse {
            rspec_file,
            output_file,
        } => {
            let rspec_output = fs::read_to_string(rspec_file)?;

            let file_timings = timings::parse_rspec_output(rspec_output)?;
            let timings_json = serde_json::to_string(&file_timings)?;

            let mut output_file = File::create(output_file)?;
            output_file.write_all(timings_json.as_bytes())?;
        }
        Opt::Split {
            total_splits: _,
            current_split: _,
            timing_file: _,
        } => {}
    }

    Ok(())
}
