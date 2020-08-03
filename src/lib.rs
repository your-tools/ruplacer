mod directory_patcher;
mod file_patcher;
mod line_patcher;
pub mod query;
mod settings;
pub use settings::Settings;
mod stats;
pub use crate::directory_patcher::DirectoryPatcher;
pub use crate::stats::Stats;
