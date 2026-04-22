//! Pre-built test scenarios for the Stellar fee tracker harness.

use std::path::Path;

/// Loads a scenario JSON file from the given path and returns its contents.
pub fn load_from_file(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
