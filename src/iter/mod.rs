//! Definitions of various iterators.
use crate::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

/// File path entries.
pub struct Entries {
    entries: Vec<PathBuf>,
}

impl Entries {
    /// Creates an `Entries` from a single file path.
    pub fn from(path: &str) -> Self {
        Entries {
            entries: vec![PathBuf::from(path)],
        }
    }

    /// Creates an `Entries` from a directory of files.
    pub fn from_dir(dir: &str) -> Result<Self> {
        let mut entries = Vec::new();
        for entry in WalkDir::new(dir).max_depth(1) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let path = PathBuf::from(entry.path());
                entries.push(path);
            }
        }
        Ok(Entries { entries })
    }

    /// Creates an `Entries` from a directory of files, excluding files whose extension
    /// does not match one of the given extensions.
    ///
    /// Note that the `.` before extensions should not be given.
    ///
    /// A valid extension vector: `vec!['csv', 'xlsx']`.
    pub fn from_dir_with(dir: &str, exts: Vec<&str>) -> Result<Self> {
        let mut entries = Vec::new();
        for entry in WalkDir::new(dir).max_depth(1) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if exts.contains(&ext.to_str().unwrap()) {
                        entries.push(path.into())
                    }
                }
            }
        }
        Ok(Entries { entries })
    }
}

impl IntoIterator for Entries {
    type Item = PathBuf;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}
