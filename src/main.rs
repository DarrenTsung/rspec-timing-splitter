use std::collections::HashSet;
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
            let file_timings = load_file_timings(timing_output)?;

            if current_split >= total_splits {
                println!(
                    "Error: current split should be between [0..{}), got {}.",
                    total_splits, current_split
                );
                return Ok(());
            }

            let bucket = {
                let mut bucketed_timings = timings::split_timings(&file_timings, total_splits);
                bucketed_timings.remove(current_split as usize)
            };

            print!(
                "{}",
                bucket
                    .into_iter()
                    .map(|t| t.file_path)
                    .collect::<Vec<_>>()
                    .join(" ")
            );

            if current_split == total_splits - 1 {
                print!(
                    " {}",
                    paths_not_covered_by_timings(&file_timings)?
                        .into_iter()
                        .map(|p| p.to_str().unwrap().to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            }

            println!("");
        }
        Opt::Analyze {
            total_splits,
            timing_file,
        } => {
            let mut non_covered_paths = 0;

            let timing_output = fs::read_to_string(timing_file)?;
            let file_timings = load_file_timings(timing_output)?;
            let bucketed_timings = timings::split_timings(&file_timings, total_splits);

            for (index, bucket) in bucketed_timings.into_iter().enumerate() {
                let bucket_total_time: f64 = bucket.iter().map(|t| t.total_time).sum();
                let mut file_names = bucket
                    .into_iter()
                    .map(|t| {
                        let file_stem = get_file_stem(&PathBuf::from(t.file_path));
                        format!("{}:{:.2}s", file_stem, t.total_time)
                    })
                    .collect::<Vec<_>>();

                if index + 1 == total_splits as usize {
                    let paths_not_covered_by_timings = paths_not_covered_by_timings(&file_timings)?;
                    if !paths_not_covered_by_timings.is_empty() {
                        non_covered_paths = paths_not_covered_by_timings.len();
                    }
                    file_names.append(
                        &mut paths_not_covered_by_timings
                            .into_iter()
                            .map(|p| format!("{}:NA", get_file_stem(&p)))
                            .collect::<Vec<_>>(),
                    );
                }
                println!(
                    "[BUCKET {} - {:.2}s] {}",
                    index + 1,
                    bucket_total_time,
                    file_names.join(", ")
                );
            }

            if non_covered_paths > 0 {
                println!(
                    "WARNING: Found {} non-covered paths, please re-run split timing script to fix!",
                    non_covered_paths
                )
            }
        }
    }

    Ok(())
}

fn load_file_timings(timing_output: String) -> Result<Vec<FileTiming>, failure::Error> {
    let mut file_timings: Vec<FileTiming> = serde_json::from_str(&timing_output)?;
    let spec_paths = read_specs_recursively()?
        .into_iter()
        .collect::<HashSet<_>>();
    file_timings.retain(|t| spec_paths.contains(&PathBuf::from(&t.file_path)));
    Ok(file_timings)
}

fn paths_not_covered_by_timings(timings: &[FileTiming]) -> Result<Vec<PathBuf>, failure::Error> {
    let covered_paths = timings
        .iter()
        .map(|t| PathBuf::from(t.file_path.clone()))
        .collect::<HashSet<_>>();

    let mut not_covered_paths = HashSet::new();
    for spec_path in read_specs_recursively()? {
        if !covered_paths.contains(&spec_path) {
            not_covered_paths.insert(spec_path);
        }
    }

    Ok(not_covered_paths.into_iter().collect())
}

fn read_specs_recursively() -> Result<Vec<PathBuf>, failure::Error> {
    let mut specs = vec![];
    let mut dirs_to_read = vec![fs::read_dir("./spec")?];
    while let Some(dir) = dirs_to_read.pop() {
        for entry in dir {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                dirs_to_read.push(fs::read_dir(path)?);
            } else {
                let file_stem = get_file_stem(&path);
                if file_stem.ends_with("_spec") {
                    specs.push(path);
                }
            }
        }
    }
    Ok(specs)
}

fn get_file_stem(path: &PathBuf) -> String {
    path.file_stem().unwrap().to_str().unwrap().to_string()
}
