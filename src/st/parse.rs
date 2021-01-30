//! Parsing functions, such as [`parse_header`] and [`parse_record`].
use crate::CsvReader;
use crate::{Error, ErrorKind, Result};
use std::ops::RangeInclusive;

use chrono::NaiveDate;
use toml::Value as Config;

/// Represents the indexes of ST record fields.
#[derive(Debug, Clone, Copy, Default)]
pub struct Header {
    pub mid: usize,
    pub sid: usize,
    pub wid: usize,
    pub mname: usize,
    pub sname: usize,
    pub qt: usize,
    pub at: usize,
    pub dt: usize,
}

impl Header {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Represents the indexes of ST record fields.
#[derive(Debug, Clone)]
pub struct STHeader {
    pub mid: String,
    pub sid: String,
    pub wid: String,
    pub mname: String,
    pub sname: String,
    pub qt: String,
    pub at: String,
    pub dt: String,
}

/// Represents a ST record.
#[derive(Debug, Clone, Default)]
pub struct Record {
    pub mid: u32,
    pub sid: u32,
    pub wid: u16,
    pub mname: String,
    pub sname: String,
    pub qt: f64,
    pub at: f64,
    pub dt: Option<NaiveDate>,
}

impl Record {
    pub fn new() -> Self {
        Default::default()
    }
}

/// A list of inclusive range ([`RangeInclusive`]).
#[derive(Debug, Clone)]
pub struct Ranges(pub Vec<RangeInclusive<usize>>);

impl IntoIterator for Ranges {
    type Item = RangeInclusive<usize>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Represents store ranges.
#[derive(Debug, Clone)]
pub struct StoreRange {
    pub jmj: Ranges,
    pub tey: Ranges,
    pub lkd: Ranges,
    pub nws: Ranges,
    pub son: Ranges,
    pub jmj_local: Ranges,
    pub tey_local: Ranges,
    pub lkd_local: Ranges,
    pub nws_local: Ranges,
    pub son_local: Ranges,
    pub dc_outer: Ranges,
}

/// Parses `st` table in `config.toml`.
pub fn parse_config_st_headers(config: &Config) -> Result<STHeader> {
    let st = config
        .get("st")
        .map(|st| st.as_table())
        .flatten()
        .ok_or_else(|| Error::new(ErrorKind::Config("table st".to_owned())))?;

    macro_rules! field {
        ($field:literal) => {
            st.get($field)
                .map(|f| f.as_str())
                .flatten()
                .ok_or_else(|| Error::new(ErrorKind::Config(concat!("st.", $field).to_owned())))?
                .to_string();
        };
    }

    Ok(STHeader {
        mid: field!("field_mid"),
        sid: field!("field_sid"),
        wid: field!("field_wid"),
        mname: field!("field_mname"),
        sname: field!("field_sname"),
        qt: field!("field_qt"),
        at: field!("field_at"),
        dt: field!("field_dt"),
    })
}

/// Parses `range` table in `config.toml`.
pub fn parse_config_store_ranges(config: &Config) -> Result<StoreRange> {
    let range = config
        .get("range")
        .map(|range| range.as_table())
        .flatten()
        .ok_or_else(|| Error::new(ErrorKind::Config("table range".to_owned())))?;

    fn ranges(range: &toml::map::Map<String, Config>, field: &str) -> Result<Ranges> {
        let vec = range
            .get(field)
            .map(|arr| {
                arr.as_array()
                    .map(|arr| {
                        arr.into_iter()
                            .map(|r| {
                                r.as_str().map(|s| s.to_string()).ok_or_else(|| {
                                    Error::new(ErrorKind::Config(format!("range.{}.{}", field, r)))
                                })
                            })
                            .collect::<Result<Vec<String>>>()
                    })
                    .ok_or_else(|| Error::new(ErrorKind::Config(format!("range.{}", field))))
            })
            .ok_or_else(|| Error::new(ErrorKind::Config("range".to_owned())))???;

        let mut ranges = Ranges(Vec::new());
        for range in vec {
            if range.contains("-") {
                let mut split = range.split("-");
                let (left, right) = (split.next().unwrap(), split.next().unwrap());
                let (left, right) = (
                    left.parse::<usize>()
                        .map_err(|_| Error::new(ErrorKind::FromStr(left.to_owned(), "usize")))?,
                    right
                        .parse::<usize>()
                        .map_err(|_| Error::new(ErrorKind::FromStr(right.to_owned(), "usize")))?,
                );
                ranges.0.push(RangeInclusive::new(left, right));
            } else {
                let num = range
                    .parse::<usize>()
                    .map_err(|_| Error::new(ErrorKind::FromStr(range.to_owned(), "usize")))?;
                ranges.0.push(RangeInclusive::new(num, num));
            }
        }

        Ok(ranges)
    }

    Ok(StoreRange {
        jmj: ranges(range, "range_jmj")?,
        tey: ranges(range, "range_tey")?,
        lkd: ranges(range, "range_lkd")?,
        nws: ranges(range, "range_nws")?,
        son: ranges(range, "range_son")?,
        jmj_local: ranges(range, "range_jmj_local")?,
        tey_local: ranges(range, "range_tey_local")?,
        lkd_local: ranges(range, "range_lkd_local")?,
        nws_local: ranges(range, "range_nws_local")?,
        son_local: ranges(range, "range_son_local")?,
        dc_outer: ranges(range, "range_outer_warehouse")?,
    })
}

/// Parses the header of a ST table into a [`Header`].
///
/// # Errors
///
/// Returns an error variant of [`ErrorKind::Config`] if the parsing process failed.
pub fn parse_header(header: &str, config: &Config, reader: &mut CsvReader) -> Result<Header> {
    let st_headers = parse_config_st_headers(&config)?;

    let fields = reader.read_line(header).unwrap();

    let mut header = Header::new();
    for (i, v) in fields.iter().enumerate() {
        macro_rules! assign_index_if_match {
            ($($field:ident),*) => {
                match v {
                    $(_ if v.as_str() == st_headers.$field.as_str() && header.$field == 0 => header.$field = i,)*
                    _ => (),
                }

            }
        }
        assign_index_if_match!(mid, sid, wid, mname, sname, qt, at, dt)
    }

    Ok(header)
}

/// Parses a single record of a ST table into a [`Record`].
///
/// # Errors
///
/// If the parsing failed, an error variant  of [`ErrorKind::FromStr`] will be returned.
pub fn parse_record(data: &str, header: Header, reader: &mut CsvReader) -> Result<Option<Record>> {
    let mut record = Record::new();

    let fields = reader.read_line(data).unwrap();
    // It cannot be parsed into a `Record` if its length is less than 8.
    if fields.len() < 8 {
        return Ok(None);
    }
    for (i, v) in fields.iter().enumerate() {
        match i {
            _ if i == header.mid => {
                if v == "" {
                    return Ok(None);
                }
                record.mid = v
                    .parse::<u32>()
                    .map_err(|_| Error::new(ErrorKind::FromStr(v.clone(), "u32")))?;
            }
            _ if i == header.sid => {
                record.sid = v
                    .parse::<u32>()
                    .map_err(|_| Error::new(ErrorKind::FromStr(v.clone(), "u32")))?;
            }
            _ if i == header.wid => {
                record.wid = v
                    .trim()
                    .parse::<u16>()
                    .map_err(|_| Error::new(ErrorKind::FromStr(v.clone(), "u16")))?;
            }
            _ if i == header.mname => {
                record.mname = v.to_string();
            }
            _ if i == header.sname => {
                record.sname = v.to_string();
            }
            _ if i == header.qt => {
                record.qt = parse_csv_number(v)?;
            }
            _ if i == header.at => {
                record.at = parse_csv_number(v)?;
            }
            _ if i == header.dt => {
                record.dt = Some(
                    NaiveDate::parse_from_str(v.as_str(), "%Y/%m/%d")
                        .map_err(|_| Error::new(ErrorKind::FromStr(v.clone(), "date")))?,
                );
            }
            // Ignore all the fields.
            _ => (),
        }
    }

    Ok(Some(record))
}

/// Parses a single csv value into a [`f64`] float number.
///
/// # Errors
///
/// Returns an error variant of [`ErrorKind::FromStr`] if the parsing process failed.
pub fn parse_csv_number(s: &str) -> Result<f64> {
    if s == "" {
        return Ok(0.0);
    }

    let strnum = if s.starts_with(".") {
        "0".to_owned() + s
    } else {
        s.to_string()
    };

    strnum
        .replace(",", "")
        .parse::<f64>()
        .map_err(|_| Error::new(ErrorKind::FromStr(s.to_owned(), "f64")))
}
