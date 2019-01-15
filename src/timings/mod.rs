use serde_derive::{Deserialize, Serialize};

mod parse;

pub use self::parse::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FileTiming {
    pub file_path: String,
    pub total_time: f64,
}
