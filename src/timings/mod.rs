use serde_derive::{Deserialize, Serialize};

mod parse;
mod split;

pub use self::parse::*;
pub use self::split::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct FileTiming {
    pub file_path: String,
    pub total_time: f64,
}
