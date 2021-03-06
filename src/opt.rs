use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rspec-timing-tool",
    about = "A tool to help split rspec files by timings."
)]
pub enum Opt {
    #[structopt(name = "parse")]
    Parse {
        /// Input file of rspec timing information
        /// Ex. 'bundle exec rspec --format RspecJunitFormatter -o rspec-results.xml'
        /// Then rspec_file is: rspec-results.xml
        #[structopt(parse(from_os_str))]
        rspec_file: PathBuf,

        /// Output file of timing information
        #[structopt(parse(from_os_str), short = "o", long = "output")]
        output_file: PathBuf,
    },
    #[structopt(name = "split-pre-bucketed")]
    SplitPreBucketed {
        /// Number of total splits the timing data is split into
        #[structopt(short = "s", long = "total-splits")]
        total_splits: u32,

        /// Current split needed to output files
        #[structopt(short = "c", long = "current-split")]
        current_split: u32,

        /// Read splits from pre-bucketed file, useful for
        /// manual balancing.
        #[structopt(parse(from_os_str))]
        pre_bucketed_file: PathBuf,
    },
    #[structopt(name = "split")]
    Split {
        /// Number of total splits the timing data is split into
        #[structopt(short = "s", long = "total-splits")]
        total_splits: u32,

        /// Current split needed to output files
        #[structopt(short = "c", long = "current-split")]
        current_split: u32,

        /// Input file of parsed timing information
        #[structopt(parse(from_os_str))]
        timing_file: PathBuf,
    },
    #[structopt(name = "analyze")]
    Analyze {
        /// Number of total splits the timing data is split into
        #[structopt(short = "s", long = "total-splits")]
        total_splits: u32,

        /// Dump buckets into an output file
        #[structopt(parse(from_os_str), short = "o", long = "output")]
        output_file: Option<PathBuf>,

        /// Input file of parsed timing information
        #[structopt(parse(from_os_str))]
        timing_file: PathBuf,
    },
    #[structopt(name = "output-missing")]
    OutputMissing {
        /// Input file of parsed timing information
        #[structopt(parse(from_os_str))]
        timing_file: PathBuf,
    },
}
