use std::mem;

static STRING_INITIAL_CAPACITY: usize = 64usize;

enum ParseState {
  Neutral,
  InField,
  InQuotedField,
  EncounteredQuoteInQuotedField,
  EndOfRow,
}

/// A simple CSV reader.
pub struct CsvReader {
  state: ParseState,
  row_data: Vec<String>,
  column_buffer: String,
  options: CsvReaderOptions,
}

/// CSV reader options.
#[derive(Clone, Copy)]
pub struct CsvReaderOptions {
  pub delimiter: char,
  pub text_enclosure: char,
}

impl Default for CsvReaderOptions {
  fn default() -> CsvReaderOptions {
    CsvReaderOptions {
      delimiter: ',',
      text_enclosure: '"',
    }
  }
}

impl CsvReader {
  pub fn new() -> CsvReader {
    CsvReader::with_options(Default::default())
  }

  pub fn with_options(options: CsvReaderOptions) -> CsvReader {
    CsvReader {
      state: ParseState::Neutral,
      row_data: Vec::new(),
      column_buffer: String::with_capacity(STRING_INITIAL_CAPACITY),
      options: options,
    }
  }

  fn new_column(&mut self) {
    let column_data = mem::replace(
      &mut self.column_buffer,
      String::with_capacity(STRING_INITIAL_CAPACITY),
    );
    self.row_data.push(column_data);
    self.state = ParseState::Neutral;
  }

  pub fn read_line(&mut self, str: &str) -> Option<Vec<String>> {
    let line = str.to_owned();
    let delimiter = self.options.delimiter;
    let text_enclosure = self.options.text_enclosure;

    for c in line.chars() {
      match self.state {
        ParseState::Neutral => {
          match c {
            _ if c == text_enclosure => {
              // Start of quoted field
              self.state = ParseState::InQuotedField;
            }
            _ if c == delimiter => {
              // Empty field
              self.row_data.push(String::new());
            }
            '\n' => {
              // Newline outside of quoted field
              self.new_column();
              self.state = ParseState::EndOfRow;
            }
            '\r' => { // Return outside of quoted field. Eat it and keep going
            }
            _ => {
              // Anything else is unquoted data
              self.column_buffer.push(c);
              self.state = ParseState::InField;
            }
          }
        }
        ParseState::InQuotedField => {
          match c {
            _ if c == text_enclosure => {
              self.state = ParseState::EncounteredQuoteInQuotedField;
            }
            _ => {
              // Anything else is data
              self.column_buffer.push(c);
            }
          }
        }
        ParseState::InField => {
          match c {
            _ if c == delimiter => {
              self.new_column();
            }
            '\n' => {
              self.new_column();
              self.state = ParseState::EndOfRow;
            }
            '\r' => {}
            _ => {
              // Anything else is data
              self.column_buffer.push(c);
            }
          }
        }
        ParseState::EncounteredQuoteInQuotedField => {
          match c {
            _ if c == text_enclosure => {
              // 2nd " in a row inside quoted field - escaped quote
              self.column_buffer.push(c);
              self.state = ParseState::InQuotedField;
            }
            _ if c == delimiter => {
              // Field separator, end of quoted field
              self.new_column();
            }
            '\n' => {
              self.new_column();
              self.state = ParseState::EndOfRow;
            }
            '\r' => {}
            _ => {
              // Data after quoted field, treat it as data and add to existing data
              self.column_buffer.push(c);
              self.state = ParseState::InField;
            }
          }
        }
        ParseState::EndOfRow => {
          assert!(false, "Should never reach match of EndOfRow");
        }
      }
    } // end for

    // The parser might have left some data in the column_buffer if it never encountered a newline
    // Add whatever was collected to the current row
    if !self.column_buffer.is_empty() {
      self.new_column();
    }

    // Move row_data out of CsvReader
    let row_data = mem::take(&mut self.row_data);

    // Reset state
    self.state = ParseState::Neutral;

    if row_data.is_empty() {
      return None;
    } else {
      return Some(row_data);
    }
  }
}
