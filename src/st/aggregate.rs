//! Performs preset statistical aggregations for ST records.
use super::parse::*;
use crate::convert::{decode, EncodeType};
use crate::CsvReader;
use crate::{Error, ErrorKind, Result};

use chrono::NaiveDate;
use fxhash::{FxHashMap, FxHashSet};
use toml::Value;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone, Copy, Default)]
pub struct FSlots {
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
pub struct ISlots {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BrandType {
    Jmj,
    Tey,
    Lkd,
    Son,
    Nws,
    Os,
    Dc,
    Oth,
}

impl From<(StoreType, StoreLoc)> for BrandType {
    fn from(src: (StoreType, StoreLoc)) -> Self {
        use StoreLoc::*;
        use StoreType::*;
        match src {
            (Jmj, Local) => Self::Jmj,
            (Tey, Local) => Self::Tey,
            (Lkd, Local) => Self::Lkd,
            (Son, Local) => Self::Son,
            (Nws, Local) => Self::Nws,
            (Dc, Outer) => Self::Dc,
            (_, Outer) => Self::Os,
            _ => Self::Oth,
        }
    }
}

impl Default for StoreType {
    fn default() -> Self {
        StoreType::Oth
    }
}

impl Default for StoreLoc {
    fn default() -> Self {
        StoreLoc::Unknown
    }
}

impl Default for BrandType {
    fn default() -> Self {
        BrandType::Oth
    }
}

impl StoreType {
    pub fn to_string(self) -> String {
        match self {
            StoreType::Jmj => "九毛九".to_string(),
            StoreType::Tey => "太二".to_string(),
            StoreType::Lkd => "两颗鸡蛋".to_string(),
            StoreType::Son => "怂".to_string(),
            StoreType::Nws => "那未大叔".to_string(),
            StoreType::Dc => "外区".to_string(),
            StoreType::Oth => "其他".to_string(),
        }
    }
}

impl StoreLoc {
    pub fn to_string(self) -> String {
        match self {
            StoreLoc::Local => "广深".to_string(),
            StoreLoc::Outer => "外区".to_string(),
            StoreLoc::Unknown => "未知".to_string(),
        }
    }
}

impl BrandType {
    pub fn to_string(self) -> String {
        match self {
            BrandType::Jmj => "九毛九".to_string(),
            BrandType::Tey => "太二".to_string(),
            BrandType::Lkd => "两颗鸡蛋".to_string(),
            BrandType::Son => "怂".to_string(),
            BrandType::Nws => "那未大叔".to_string(),
            BrandType::Os => "外区门店".to_string(),
            BrandType::Dc => "外区".to_string(),
            BrandType::Oth => "其他".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Material {
    pub mid: u32,
    pub wid: u16,
    pub mname: String,
    pub store: ISlots,
    pub req_times: ISlots,
    pub quantity: FSlots,
    pub amount: FSlots,
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
    pub max_req_amount: f64,
    pub min_req_amount: f64,
    pub max_req_date: Option<NaiveDate>,
    pub min_req_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Default)]
pub struct Brand {
    pub brand: BrandType,
    pub req_amount: f64,
    pub sku_in_use: u16,
    pub req_amount_11751: f64,
    pub req_amount_11752: f64,
    pub req_amount_11753: f64,
    pub req_amount_11754: f64,
    pub req_amount_11755: f64,
    pub req_amount_11759: f64,
    pub req_amount_11795: f64,
    pub req_amount_other: f64,
    pub sku_in_use_11751: u16,
    pub sku_in_use_11752: u16,
    pub sku_in_use_11753: u16,
    pub sku_in_use_11754: u16,
    pub sku_in_use_11755: u16,
    pub sku_in_use_11759: u16,
    pub sku_in_use_11795: u16,
    pub sku_in_use_other: u16,
    pub sku_in_use_alone_11751: u16,
    pub sku_in_use_alone_11752: u16,
    pub sku_in_use_alone_11753: u16,
    pub sku_in_use_alone_11754: u16,
    pub sku_in_use_alone_11755: u16,
    pub sku_in_use_alone_11759: u16,
    pub sku_in_use_alone_11795: u16,
    pub sku_in_use_alone_other: u16,
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

/// A type alias for `FxhashMap<u32, Material>`.
pub type MMap = FxHashMap<u32, Material>;
/// A type alias for `FxHashMap<u32, Store>`.
pub type SMap = FxHashMap<u32, Store>;
/// A type alias for `FxHashMap<u32, Brand>`.
pub type BMap = FxHashMap<BrandType, Brand>;

/// Aggregates ST records from a single file.
pub fn aggregate(
    file: File,
    encoding: EncodeType,
    config: &Value,
    reader: &mut CsvReader,
    strict: bool,
) -> Result<(MMap, SMap, BMap)> {
    let mut mmap = MMap::default();
    let mut smap = SMap::default();
    let mut bmap = BMap::default();
    let mut jmj_req_set: FxHashSet<(u32, u32)> = FxHashSet::default();
    let mut tey_req_set: FxHashSet<(u32, u32)> = FxHashSet::default();
    let mut lkd_req_set: FxHashSet<(u32, u32)> = FxHashSet::default();
    let mut son_req_set: FxHashSet<(u32, u32)> = FxHashSet::default();
    let mut nws_req_set: FxHashSet<(u32, u32)> = FxHashSet::default();
    let mut oth_req_set: FxHashSet<(u32, u32)> = FxHashSet::default();
    let mut os_req_set: FxHashSet<(u32, u32)> = FxHashSet::default();
    let mut dc_req_set: FxHashSet<(u32, u32)> = FxHashSet::default();
    let mut daily_req_qt: FxHashMap<u32, FxHashMap<NaiveDate, f64>> = FxHashMap::default();
    let mut store_map: FxHashMap<u32, (FxHashSet<u32>, FxHashMap<NaiveDate, f64>)> =
        FxHashMap::default();
    let mut brand_set: FxHashSet<(u32, BrandType)> = FxHashSet::default();

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
        buf.clear();
        match rdr.read_until(b'\n', &mut buf) {
            Err(e) => return Err(Error::new(ErrorKind::Io(e))),
            Ok(0) => {
                // All contents have been read when reaching here.
                // Just break here.
                break;
            }
            Ok(_) => {
                decode(&buf, encoding, &mut line)?;
                line_number += 1;
                let record = match parse_record(&line, header, reader) {
                    Ok(option_record) => option_record,
                    Err(e) => {
                        if !strict {
                            continue;
                        } else {
                            return Err(Error::new(ErrorKind::MalformedData(
                                e.to_string(),
                                line_number,
                            )));
                        }
                    }
                };
                let record = if record.is_none() {
                    // All records have been aggregated when reaching here.
                    // Break this loop.
                    break;
                } else {
                    record.unwrap()
                };

                // Only insert a `Material` into `mmap` if quantity is not zero.
                if record.qt != 0.0 {
                    // Insert a new `Material` into `mmap` if `record.mid` does not exist yet.
                    let mmap_entry = mmap.entry(record.mid).or_insert(Material {
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

                    let mut smap_entry = smap.entry(record.sid).or_insert(Store {
                        sid: record.sid,
                        sname: record.sname,
                        store_type: Default::default(),
                        store_loc: Default::default(),
                        sku_in_use: 0,
                        amount: 0.0,
                        first_req_date: None,
                        last_req_date: None,
                        max_req_interval: 0,
                        min_req_interval: 0,
                        max_req_date: None,
                        min_req_date: None,
                        max_req_amount: 0.0,
                        min_req_amount: 0.0,
                    });

                    use StoreLoc::*;
                    use StoreType::*;
                    macro_rules! update_maps_stage1 {
                    ($($type:ident, $loc:ident, $set:ident, $field:ident)*) => {
                        match get_store_type(record.sid, &ranges) {
                            $(($type, $loc) => {
                                // Update `Material`.
                                if record.qt > 0.0 && $set.insert((record.mid, record.sid)) {
                                    (*mmap_entry).store.$field += 1;
                                }
                                (*mmap_entry).quantity.$field += record.qt;
                                {
                                    let entry = daily_req_qt.entry(record.mid).or_default();
                                    let entry_inner = entry.entry(record.dt.unwrap()).or_default();
                                    *entry_inner += record.qt;
                                }
                                (*mmap_entry).amount.$field += record.at;
                                if record.qt > 0.0 {
                                    (*mmap_entry).req_times.$field += 1;
                                }

                                // Update `Store`.
                                (*smap_entry).store_type = $type;
                                (*smap_entry).store_loc = $loc;
                                (*smap_entry).amount += record.at;
                                let entry = store_map.entry(record.sid).or_default();
                                if (*entry).0.insert(record.mid) {
                                    (*smap_entry).sku_in_use += 1;
                                }
                                let entry_inner = (*entry).1.entry(record.dt.unwrap()).or_default();
                                *entry_inner += record.at;

                                // Update `Brand`.
                                let brand_type = ($type, $loc).into();
                                let bmap_entry = bmap.entry(brand_type).or_default();
                                (*bmap_entry).brand = brand_type;
                                (*bmap_entry).req_amount += record.at;
                                match record.wid {
                                    11751 => (*bmap_entry).req_amount_11751 += record.at,
                                    11752 => (*bmap_entry).req_amount_11752 += record.at,
                                    11753 => (*bmap_entry).req_amount_11753 += record.at,
                                    11754 => (*bmap_entry).req_amount_11754 += record.at,
                                    11755 => (*bmap_entry).req_amount_11755 += record.at,
                                    11759 => (*bmap_entry).req_amount_11759 += record.at,
                                    11795 => (*bmap_entry).req_amount_11795 += record.at,
                                    _ => (*bmap_entry).req_amount_other += record.at,
                                }
                                if brand_set.insert((record.mid, brand_type)) {
                                    (*bmap_entry).sku_in_use += 1;
                                    match record.wid {
                                        11751 => (*bmap_entry).sku_in_use_11751 += 1,
                                        11752 => (*bmap_entry).sku_in_use_11752 += 1,
                                        11753 => (*bmap_entry).sku_in_use_11753 += 1,
                                        11754 => (*bmap_entry).sku_in_use_11754 += 1,
                                        11755 => (*bmap_entry).sku_in_use_11755 += 1,
                                        11759 => (*bmap_entry).sku_in_use_11759 += 1,
                                        11795 => (*bmap_entry).sku_in_use_11795 += 1,
                                        _ => (*bmap_entry).sku_in_use_other += 1,
                                    }
                                }
                            })*
                            _ => (),
                        }
                    }
                }

                    update_maps_stage1!(
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

    // Stage 2 `mmap` generation process.
    for (mid, map) in daily_req_qt {
        // Transmute `map` into a `vec` for easy in-place sorting.
        let mut vec = map.into_iter().collect::<Vec<_>>();
        vec.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        // SAFETY: `unwrap`s here is safe because when current `(mid, map)` pair is present in
        // `daily_req_qt`, there must be at least one entry in `map`.
        let &(dt, qt) = vec.first().unwrap();
        let last_dt = vec.last().unwrap().0;
        let mut min_qt = qt;
        let mut max_qt = qt;
        let mut min_dt = dt;
        let mut max_dt = dt;
        let mut min_gap = 0;
        let mut max_gap = 0;
        let mut gap_prev = 0;
        vec.iter()
            .zip(vec.iter().skip(1))
            .for_each(|(&(d1, _q1), &(d2, q2))| {
                if q2 > max_qt {
                    max_qt = q2;
                    max_dt = d2;
                } else if q2 < min_qt {
                    min_qt = q2;
                    min_dt = d2;
                }

                let gap = (d2 - d1).num_days().abs();
                if gap_prev == 0 {
                    max_gap = gap;
                    min_gap = gap;
                } else if gap > gap_prev {
                    max_gap = gap;
                } else if gap < gap_prev {
                    min_gap = gap;
                }
                gap_prev = gap;
            });

        // Update `mmap` entries.
        mmap.entry(mid).and_modify(|e| {
            (*e).min_req_interval = min_gap as u16;
            (*e).max_req_interval = max_gap as u16;
            (*e).min_req_quantity = min_qt;
            (*e).max_req_quantity = max_qt;
            (*e).min_req_date = Some(min_dt);
            (*e).max_req_date = Some(max_dt);
            (*e).first_req_date = Some(dt);
            (*e).last_req_date = Some(last_dt);
        });
    }

    // Stage 2 `smap` generation process.
    for (sid, (_, map)) in store_map {
        // Transmute `map` into a `vec` for easy in-place sorting.
        let mut vec = map.into_iter().collect::<Vec<_>>();
        vec.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        // SAFETY: `unwrap`s here is safe because when current `(sid, map)` pair is present in
        // `store_map`, there must be at least one entry in `map`.
        let &(dt, at) = vec.first().unwrap();
        let last_dt = vec.last().unwrap().0;
        let mut min_at = at;
        let mut max_at = at;
        let mut min_dt = dt;
        let mut max_dt = dt;
        let mut min_gap = 0;
        let mut max_gap = 0;
        let mut gap_prev = 0;
        vec.iter()
            .zip(vec.iter().skip(1))
            .for_each(|(&(d1, _a1), &(d2, a2))| {
                if a2 > max_at {
                    max_at = a2;
                    max_dt = d2;
                } else if a2 < min_at {
                    min_at = a2;
                    min_dt = d2;
                }

                let gap = (d2 - d1).num_days().abs();
                if gap_prev == 0 {
                    max_gap = gap;
                    min_gap = gap;
                } else if gap > gap_prev {
                    max_gap = gap;
                } else if gap < gap_prev {
                    min_gap = gap;
                }
                gap_prev = gap;
            });

        // Update `smap` entries.
        smap.entry(sid).and_modify(|e| {
            (*e).min_req_interval = min_gap as u16;
            (*e).max_req_interval = max_gap as u16;
            (*e).min_req_amount = min_at;
            (*e).max_req_amount = max_at;
            (*e).min_req_date = Some(min_dt);
            (*e).max_req_date = Some(max_dt);
            (*e).first_req_date = Some(dt);
            (*e).last_req_date = Some(last_dt);
        });
    }

    // Return three maps.
    Ok((mmap, smap, bmap))
}

pub fn get_store_type(sid: u32, ranges: &StoreRange) -> (StoreType, StoreLoc) {
    // TODO: `StoreRange` should implement Iterator trait.
    macro_rules! check_store_type {
        ($($range:ident, $ty:ident, $loc:ident)*) => {
            $(if ranges
                .$range
                .clone()
                .into_iter()
                .any(|r| r.contains(&(sid as usize)))
            {
                return (StoreType::$ty, StoreLoc::$loc);
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
