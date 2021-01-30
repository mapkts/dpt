use calamine::{open_workbook_auto, Reader};
use encoding::{all::GB18030, EncoderTrap, Encoding};

use std::fs::File;
use std::io::{self, prelude::*, LineWriter};
use std::path::Path;

/// Converts a rich format Excel file to CSV file that only contains data.
pub fn xlsx2csv(file: &str, out_dir: &str) -> io::Result<()> {
  let path = Path::new(file);
  let filename = path.file_stem().unwrap().to_str().unwrap();

  let mut excel = open_workbook_auto(path).expect("cannot open file");

  if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
    let file = File::create(format!("{}{}.csv", out_dir, filename))?;
    let mut file = LineWriter::new(file);

    for row in r.rows() {
      let mut line = String::new();

      for cell in row.into_iter() {
        line.push_str(&cell.to_string());
        line.push_str(",");
      }
      line.pop();
      line.push_str("\r\n");

      let line_u8: Vec<u8> = GB18030.encode(&line, EncoderTrap::Strict).unwrap();
      file.write_all(&line_u8)?;
    }

    file.flush()?;
  } else {
    eprintln!("ERROR: Worksheet `Sheet1` not found in the .xlsx file.");
  }

  Ok(())
}
