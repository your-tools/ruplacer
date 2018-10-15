extern crate ignore;
extern crate inflector;
extern crate colored;
extern crate difference;
extern crate regex;
mod errors;
pub use errors::Error;
mod stats;
pub mod query;
mod line_patcher;
mod file_patcher;
mod directory_patcher;
pub use directory_patcher::DirectoryPatcher;
pub use stats::Stats;

