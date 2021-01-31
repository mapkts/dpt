//! Performs preset statistical aggregations for ST records.
use super::parse::*;
use crate::convert::{decode, EncodeType};
use crate::CsvReader;
use crate::{Error, ErrorKind, Result};

use chrono::NaiveDate;
use fxhash::{FxHashMap, FxHashSet};
use std::path::Path;
use toml::Value;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone, Copy, Default)]
pub struct FltStore {
    pub local_jmj: f64,
    pub local_tey: f64,
    pub local_lkd: f64,
    pub local_son: f64,
    pub local_nws: f64,
    pub outer_store: f64,
    pub outer_dc: f64,
    pub other: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct IntStore {
    pub local_jmj: u32,
    pub local_tey: u32,
    pub local_lkd: u32,
    pub local_son: u32,
    pub local_nws: u32,
    pub outer_store: u32,
    pub outer_dc: u32,
    pub other: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StoreType {
    Jmj,
    Tey,
    Lkd,
    Son,
    Nws,
    Oth,
    Dc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StoreLoc {
    Local,
    Outer,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BrandType {
    Jmj,
    Tey,
    Lkd,
    Son,
    Nws,
    OS,
    DC,
    Oth,
}

impl Default for StoreType {
    fn default() -> Self {
        StoreType::Oth
    }
}

impl Default for StoreLoc {
    fn default() -> Self {
        StoreLoc::Local
    }
}

impl Default for BrandType {
    fn default() -> Self {
        BrandType::Oth
    }
}

#[derive(Debug, Clone, Default)]
pub struct Material {
    pub mid: u32,
    pub wid: u16,
    pub mname: String,
    pub store: IntStore,
    pub req_times: IntStore,
    pub quantity: FltStore,
    pub amount: FltStore,
    pub first_req_date: Option<NaiveDate>,
    pub last_req_date: Option<NaiveDate>,
    pub max_req_interval: u16,
    pub min_req_interval: u16,
    pub max_req_quantity: f64,
    pub min_req_quantity: f64,
    pub max_req_date: Option<NaiveDate>,
    pub min_req_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Default)]
pub struct Store {
    pub sid: u32,
    pub sname: String,
    pub store_type: StoreType,
    pub store_loc: StoreLoc,
    pub sku_in_use: u32,
    pub amount: f64,
    pub first_req_date: Option<NaiveDate>,
    pub last_req_date: Option<NaiveDate>,
    pub max_req_interval: u16,
    pub min_req_interval: u16,
}

#[derive(Debug, Clone, Default)]
pub struct Brand {
    pub brand: BrandType,
    pub sku_in_use: u32,
    pub amount: f64,
}

impl Material {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Store {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Brand {
    pub fn new() -> Self {
        Default::default()
    }
}

pub type MMap = FxHashMap<u32, Material>;
pub type SMap = FxHashMap<u32, Store>;
pub type BMap = FxHashMap<u32, Brand>;

/// Aggregates ST records from a single file.
pub fn aggregate<P: AsRef<Path>>(
    path: P,
    encoding: EncodeType,
    config: &Value,
    reader: &mut CsvReader,
) -> Result<(MMap, SMap, BMap)> {
    let mut mmap = MMap::default();
    let smap = SMap::default();
    let bmap = BMap::default();
    let mut jmj_req_set: FxHashSet<u32> = FxHashSet::default();
    let mut tey_req_set: FxHashSet<u32> = FxHashSet::default();
    let mut lkd_req_set: FxHashSet<u32> = FxHashSet::default();
    let mut son_req_set: FxHashSet<u32> = FxHashSet::default();
    let mut nws_req_set: FxHashSet<u32> = FxHashSet::default();
    let mut oth_req_set: FxHashSet<u32> = FxHashSet::default();
    let mut os_req_set: FxHashSet<u32> = FxHashSet::default();
    let mut dc_req_set: FxHashSet<u32> = FxHashSet::default();
    let mut daily_req_qt: FxHashMap<(u32, NaiveDate), f64> = FxHashMap::default();

    // Open the given file for reading.
    let file = File::open(path)?;
    let mut rdr = BufReader::new(file);
    let mut buf = Vec::new();
    let mut line = String::new();

    // Get store ranges from config file.
    let ranges = parse_config_store_ranges(config)?;

    // Read header row and parse it into a `Header`.
    rdr.read_until(b'\n', &mut buf)?;
    decode(&buf, encoding, &mut line)?;
    let header = parse_header(&line, config, reader)?;

    let mut line_number = 1;
    // Analyse records
    loop {
        // Must clear buffer first.
        buf.clear();
        match rdr.read_until(b'\n', &mut buf) {
            Err(e) => return Err(Error::new(ErrorKind::Io(e))),
            Ok(0) => {
                // All contents have been read when reaching here.
                // Just return the final result.
                return Ok((mmap, smap, bmap));
            }
            Ok(_) => {
                decode(&buf, encoding, &mut line)?;
                line_number += 1;
                let record = match parse_record(&line, header, reader) {
                    Ok(option_record) => option_record,
                    Err(e) => {
                        return Err(Error::new(ErrorKind::MalformedData(
                            e.to_string(),
                            line_number,
                        )));
                    }
                };
                let record = if record.is_none() {
                    // All records have been aggregated when reaching here.
                    // Just return the final result.
                    return Ok((mmap, smap, bmap));
                } else {
                    record.unwrap()
                };

                // Inserts a `Material` into `mmap` if `record.mid` does not exist yet.
                let entry = mmap.entry(record.mid).or_insert(Material {
                    mid: record.mid,
                    wid: record.wid,
                    mname: record.mname,
                    store: Default::default(),
                    req_times: Default::default(),
                    quantity: Default::default(),
                    amount: Default::default(),
                    first_req_date: record.dt,
                    last_req_date: record.dt,
                    max_req_interval: 0,
                    min_req_interval: 0,
                    max_req_quantity: 0.0,
                    min_req_quantity: 0.0,
                    max_req_date: None,
                    min_req_date: None,
                });

                use StoreLoc::*;
                use StoreType::*;
                macro_rules! update_material_op1 {
                    ($($type:ident, $loc:ident, $set:ident, $field:ident)*) => {
                        match get_store_type(record.sid, &ranges) {
                            $(($type, $loc) => {
                                // update `store`
                                if record.qt > 0.0 && $set.insert(record.sid) {
                                    (*entry).store.$field += 1;
                                }
                                // update `quantity`
                                (*entry).quantity.$field += record.qt;
                                {
                                    let entry = daily_req_qt.entry((record.mid, record.dt.unwrap())).or_insert(0.0);
                                    *entry += record.qt;
                                }
                                // update `amount`
                                (*entry).amount.$field += record.at;
                                // update `req_times`
                                if record.qt > 0.0 {
                                    (*entry).req_times.$field += 1;
                                }
                                // update `first_req_date`, `last_req_date`
                                if record.dt < (*entry).first_req_date {
                                    (*entry).first_req_date = record.dt;
                                }
                                if record.dt > (*entry).last_req_date {
                                    (*entry).last_req_date = record.dt;
                                }
                            })*
                            _ => (),
                        }
                    }
                }
                update_material_op1!(
                    Jmj, Local, jmj_req_set, local_jmj
                    Tey, Local, tey_req_set, local_tey
                    Lkd, Local, lkd_req_set, local_lkd
                    Son, Local, son_req_set, local_son
                    Nws, Local, nws_req_set, local_nws
                    Jmj, Outer, os_req_set, outer_store
                    Tey, Outer, os_req_set, outer_store
                    Lkd, Outer, os_req_set, outer_store
                    Son, Outer, os_req_set, outer_store
                    Nws, Outer, os_req_set, outer_store
                    Dc, Outer, dc_req_set, outer_dc
                    Oth, Unknown, oth_req_set, other
                );
            }
        }
    }
}

pub fn get_store_type(sid: u32, ranges: &StoreRange) -> (StoreType, StoreLoc) {
    // TODO: `StoreRange` should implement Iterator trait.
    macro_rules! check_store_type {
        ($($range:ident, $type:ident, $loc:ident)*) => {
            $(if ranges
                .$range
                .clone()
                .into_iter()
                .any(|r| r.contains(&(sid as usize)))
            {
                return (StoreType::$type, StoreLoc::$loc);
            })*
        };
    }
    check_store_type! {
        jmj_local, Jmj, Local
        tey_local, Tey, Local
        lkd_local, Lkd, Local
        son_local, Son, Local
        nws_local, Nws, Local
        jmj, Jmj, Outer
        tey, Tey, Outer
        lkd, Lkd, Outer
        son, Son, Outer
        nws, Nws, Outer
        dc_outer, Dc, Outer
    };

    (StoreType::Oth, StoreLoc::Unknown)
}
