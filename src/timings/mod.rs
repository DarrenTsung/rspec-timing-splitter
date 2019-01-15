mod parse;

pub use self::parse::*;

#[derive(Debug, PartialEq)]
pub struct FileTiming {
    pub file_path: String,
    pub total_time: f64,
}
