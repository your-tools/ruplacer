extern crate ignore;
extern crate colored;
extern crate difference;
mod errors;
mod line_patcher;
mod file_patcher;
mod directory_patcher;
pub use directory_patcher::DirectoryPatcher;
