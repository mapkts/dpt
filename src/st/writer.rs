//! Writers for writing out aggregated data.
use super::aggregate::MMap;
use crate::Result;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::LineWriter;

/// `MMap` writer.
pub fn write_mmap(mmap: MMap, out_dir: &str) -> Result<()> {
    // Set up open options.
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
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
