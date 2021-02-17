//! Writers for writing out aggregated data.
use super::aggregate::{BMap, MMap, SMap};
use crate::Result;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::LineWriter;

/// Write aggregation result to files.
pub fn write_aggregation_result(maps: (MMap, SMap, BMap), out_dir: &str) -> Result<()> {
    write_mmap(maps.0, out_dir)?;
    write_smap(maps.1, out_dir)?;
    write_bmap(maps.2, out_dir)?;
    Ok(())
}

/// `MMap` writer.
pub fn write_mmap(mmap: MMap, out_dir: &str) -> Result<()> {
    // Set up open options.
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!("{}/sku.csv", out_dir))?;

    // Write UTF-8 BOM.
    file.write("\u{feff}".as_bytes())?;

    // Write header to file
    let mut file = LineWriter::new(file);
    let header = concat!(
        "物料编码,",
        "物料名称,",
        "仓位编码,",
        "单日最大领用量,",
        "单日最小领用量,",
        "最大用量日,",
        "最小用量日,",
        "最大领用间隔天数,",
        "最小领用间隔天数,",
        "最早领用日期,",
        "最晚领用日期,",
        "领用门店数-九毛九,",
        "领用门店数-太二,",
        "领用门店数-两颗鸡蛋,",
        "领用门店数-怂,",
        "领用门店数-那未大叔,",
        "领用门店数-外区门店,",
        "领用外区数,",
        "领用-其他,",
        "领用次数-九毛九,",
        "领用次数-太二,",
        "领用次数-两颗鸡蛋,",
        "领用次数-怂,",
        "领用次数-那未大叔,",
        "领用次数-外区门店,",
        "领用次数-外区,",
        "领用次数-其他,",
        "用量-九毛九,",
        "用量-太二,",
        "用量-两颗鸡蛋,",
        "用量-怂,",
        "用量-那未大叔,",
        "用量-外区门店,",
        "用量-外区,",
        "用量-其他,",
        "金额-九毛九,",
        "金额-太二,",
        "金额-两颗鸡蛋,",
        "金额-怂,",
        "金额-那未大叔,",
        "金额-外区门店,",
        "金额-外区,",
        "金额-其他,",
    );
    file.write_all(format!("{}\r\n", header).as_bytes())?;

    // Write records.
    for v in mmap.values() {
        let record = format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\r\n",
            v.mid,
            v.mname,
            v.wid,
            v.max_req_quantity,
            v.min_req_quantity,
            v.max_req_date.unwrap(),
            v.min_req_date.unwrap(),
            v.max_req_interval,
            v.min_req_interval,
            v.first_req_date.unwrap(),
            v.last_req_date.unwrap(),
            v.store.local_jmj,
            v.store.local_tey,
            v.store.local_lkd,
            v.store.local_son,
            v.store.local_nws,
            v.store.outer_store,
            v.store.outer_dc,
            v.store.other,
            v.req_times.local_jmj,
            v.req_times.local_tey,
            v.req_times.local_lkd,
            v.req_times.local_son,
            v.req_times.local_nws,
            v.req_times.outer_store,
            v.req_times.outer_dc,
            v.req_times.other,
            v.quantity.local_jmj,
            v.quantity.local_tey,
            v.quantity.local_lkd,
            v.quantity.local_son,
            v.quantity.local_nws,
            v.quantity.outer_store,
            v.quantity.outer_dc,
            v.quantity.other,
            v.amount.local_jmj,
            v.amount.local_tey,
            v.amount.local_lkd,
            v.amount.local_son,
            v.amount.local_nws,
            v.amount.outer_store,
            v.amount.outer_dc,
            v.amount.other,
        );
        file.write_all(record.as_bytes())?;
    }

    file.flush()?;

    Ok(())
}

/// `SMap` writer.
pub fn write_smap(smap: SMap, out_dir: &str) -> Result<()> {
    // Set up open options.
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!("{}/store.csv", out_dir))?;

    // Write UTF-8 BOM.
    file.write("\u{feff}".as_bytes())?;

    // Write header to file
    let mut file = LineWriter::new(file);
    let header = concat!(
        "门店编码,",
        "门店名称,",
        "品牌,",
        "市场,",
        "使用SKU数,",
        "要货金额,",
        "单日最大要货金额,",
        "单日最小要货金额,",
        "最大要货金额日,",
        "最小要货金额日,",
        "最大要货间隔天数,",
        "最小要货间隔天数,",
        "最早要货日期,",
        "最晚要货日期,",
    );
    file.write_all(format!("{}\r\n", header).as_bytes())?;

    // Write records.
    for v in smap.values() {
        let record = format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\r\n",
            v.sid,
            v.sname,
            v.store_type.to_string(),
            v.store_loc.to_string(),
            v.sku_in_use,
            v.amount,
            v.max_req_amount,
            v.min_req_amount,
            v.max_req_date.unwrap(),
            v.min_req_date.unwrap(),
            v.max_req_interval,
            v.min_req_interval,
            v.first_req_date.unwrap(),
            v.last_req_date.unwrap(),
        );
        file.write_all(record.as_bytes())?;
    }

    file.flush()?;

    Ok(())
}

/// `BMap` writer.
pub fn write_bmap(bmap: BMap, out_dir: &str) -> Result<()> {
    // Set up open options.
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!("{}/brand.csv", out_dir))?;

    // Write UTF-8 BOM.
    file.write("\u{feff}".as_bytes())?;

    // Write header to file
    let mut file = LineWriter::new(file);
    let header = concat!(
        "品牌,",
        "要货金额,",
        "要货金额-11751,",
        "要货金额-11752,",
        "要货金额-11753,",
        "要货金额-11754,",
        "要货金额-11755,",
        "要货金额-11759,",
        "要货金额-11795,",
        "要货金额-其他,",
        "使用SKU数,",
        "使用SKU数-11751,",
        "使用SKU数-11752,",
        "使用SKU数-11753,",
        "使用SKU数-11754,",
        "使用SKU数-11755,",
        "使用SKU数-11759,",
        "使用SKU数-11795,",
        "使用SKU数-其他,",
    );
    file.write_all(format!("{}\r\n", header).as_bytes())?;

    // Write records.
    // Transform `bmap` into a sorted vector.
    let mut vec = bmap.into_iter().map(|x| x.1).collect::<Vec<_>>();
    vec.sort_unstable_by(|a, b| a.brand.cmp(&b.brand));
    for v in vec {
        let record = format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\r\n",
            v.brand.to_string(),
            v.req_amount,
            v.req_amount_11751,
            v.req_amount_11752,
            v.req_amount_11753,
            v.req_amount_11754,
            v.req_amount_11755,
            v.req_amount_11759,
            v.req_amount_11795,
            v.req_amount_other,
            v.sku_in_use,
            v.sku_in_use_11751,
            v.sku_in_use_11752,
            v.sku_in_use_11753,
            v.sku_in_use_11754,
            v.sku_in_use_11755,
            v.sku_in_use_11759,
            v.sku_in_use_11795,
            v.sku_in_use_other,
        );
        file.write_all(record.as_bytes())?;
    }

    file.flush()?;

    Ok(())
}
