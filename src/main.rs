use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

mod opt;
mod timings;

use crate::opt::Opt;
use crate::timings::FileTiming;

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
            total_splits,
            current_split,
            timing_file,
        } => {
            let timing_output = fs::read_to_string(timing_file)?;
            let file_timings: Vec<FileTiming> = serde_json::from_str(&timing_output)?;

            if current_split == 0 || current_split > total_splits {
                println!(
                    "Error: current split should be between 1 and total_splits, got {}.",
                    current_split
                );
                return Ok(());
            }

            let bucket = {
                let mut bucketed_timings = timings::split_timings(file_timings, total_splits);
                bucketed_timings.remove(current_split as usize)
            };
            println!(
                "{}",
                bucket
                    .into_iter()
                    .map(|timing| timing.file_path)
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
        Opt::Analyze {
            total_splits,
            timing_file,
        } => {
            let timing_output = fs::read_to_string(timing_file)?;
            let file_timings: Vec<FileTiming> = serde_json::from_str(&timing_output)?;
            let bucketed_timings = timings::split_timings(file_timings, total_splits);

            for (index, bucket) in bucketed_timings.into_iter().enumerate() {
                let bucket_total_time: f64 = bucket.iter().map(|t| t.total_time).sum();
                let file_names = bucket
                    .into_iter()
                    .map(|t| {
                        let file_stem = PathBuf::from(t.file_path)
                            .file_stem()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string();
                        format!("{}:{:.2}s", file_stem, t.total_time)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                println!(
                    "[BUCKET {} - {:.2}s] {}",
                    index + 1,
                    bucket_total_time,
                    file_names
                );
            }
        }
    }

    Ok(())
}
