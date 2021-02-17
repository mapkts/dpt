//! Definitions of various iterators.
use crate::{Error, ErrorKind, Result};
use walkdir::WalkDir;

use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

/// File path entries.
pub struct FilePathEntries {
    entries: Vec<PathBuf>,
}

impl FilePathEntries {
    /// Creates an `Entries` from a single file path.
    pub fn from(path: &str) -> Self {
        FilePathEntries {
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
        Ok(FilePathEntries { entries })
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
        Ok(FilePathEntries { entries })
    }
}

impl IntoIterator for FilePathEntries {
    type Item = PathBuf;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

/// Line reader of mutiply files.
#[derive(Debug)]
pub struct LineReader {
    paths: Vec<File>,
    current: Option<BufReader<File>>,
    head_once: bool,
}

impl LineReader {
    pub fn new<'a, P>(paths: Vec<&'a P>, head_once: bool) -> Result<LineReader>
    where
        P: AsRef<Path> + ?Sized,
    {
        let mut checked = Vec::new();

        for p in paths {
            if !p.as_ref().is_file() {
                return Err(Error::new(ErrorKind::Access(format!(
                    "{}",
                    p.as_ref().display()
                ))));
            }
            checked.push(File::open(p)?)
        }

        Ok(LineReader {
            paths: checked,
            head_once,
            current: None,
        })
    }

    pub fn next_line(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        match self.current.as_mut() {
            Some(reader) => match reader.read_until(b'\n', buf) {
                Ok(0) => match self.paths.pop() {
                    Some(f) => {
                        let mut reader = BufReader::new(f);
                        if self.head_once {
                            let mut buf = Vec::new();
                            reader.read_until(b'\n', &mut buf)?;
                        }
                        self.current = Some(reader);
                        return self.next_line(buf);
                    }
                    None => return Ok(0),
                },
                Ok(bytes_read) => return Ok(bytes_read),
                Err(e) => return Err(e.into()),
            },
            None => {
                if let Some(f) = self.paths.pop() {
                    let reader = BufReader::new(f);
                    self.current = Some(reader);
                    return self.next_line(buf);
                }
                return Ok(0);
            }
        }
    }
}

// fn ends_with_newline<R: Read + Seek>(f: &mut R) -> Result<bool> {
//     // If the length of the given stream is zero, it can't end with a newline.
//     if f.stream_len()? == 0 {
//         return Ok(false);
//     }

//     let mut byte = [0; 1];
//     let current_pos = f.stream_position()?;
//     f.seek(SeekFrom::End(-1))?;
//     f.read_exact(&mut byte)?;
//     // reset the internal cursor to its original position.
//     f.seek(SeekFrom::Start(current_pos))?;

//     match byte {
//         [b'\n'] => return Ok(true),
//         _ => return Ok(false),
//     }
// }
