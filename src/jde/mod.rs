//! JDE client automations.
pub mod client;
pub mod consts;
pub mod jobs;
pub mod time;

mod driver;
mod login;

pub use client::JdeClient;
pub use driver::*;
pub use login::*;

use crate::{Error, ErrorKind, Result};
use toml::Value;

pub struct Jde {
    pub username: String,
    pub password: String,
    pub address: String,
    pub browser_path: String,
}

pub struct Locators {
    pub close_btn: String,
    pub data_select_btn: String,
    pub download_btn: String,
    pub export_data_btn: String,
    pub fav_btn: String,
    pub grid_down_btn: String,
    pub login_btn: String,
    pub main_frame: i64,
    pub ok_btn: String,
    pub password_field: String,
    pub query_btn: String,
    pub query_list_selector: String,
    pub report_btn: String,
    pub select_btn: String,
    pub submit_btn: String,
    pub username_field: String,
    pub st_company_field: String,
    pub st_expected_date_field: String,
    pub st_order_type_field: String,
    pub st_repo_field: String,
    pub ios_century_field: String,
    pub ios_company_field: String,
    pub ios_company_from_field: String,
    pub ios_company_to_field: String,
    pub ios_month_field: String,
    pub ios_month_from_field: String,
    pub ios_month_to_field: String,
    pub ios_repo_field: String,
    pub ios_year_field: String,
    pub adsearch_btn: String,
    pub st_repo_add_btn: String,
    pub repo_select: String,
    pub repo_add_index0: String,
    pub repo_add_index1: String,
    pub aq_add_value_more: String,
}

/// Parses `locator` table in `config.toml`.
pub fn parse_config_locator_table(config: &Value) -> Result<Locators> {
    let locator = config
        .get("locator")
        .map(|x| x.as_table())
        .flatten()
        .ok_or_else(|| Error::new(ErrorKind::Config("table locator".to_owned())))?;

    macro_rules! field {
        ($field:literal) => {
            locator
                .get($field)
                .map(|f| f.as_str())
                .flatten()
                .ok_or_else(|| {
                    Error::new(ErrorKind::Config(concat!("locator.", $field).to_owned()))
                })?
                .to_string()
        };
    }

    macro_rules! field_int {
        ($field:literal) => {
            locator
                .get($field)
                .map(|f| f.as_integer())
                .flatten()
                .ok_or_else(|| {
                    Error::new(ErrorKind::Config(concat!("locator.", $field).to_owned()))
                })?
        };
    }

    Ok(Locators {
        close_btn: field!("close_btn"),
        data_select_btn: field!("data_select_btn"),
        download_btn: field!("download_btn"),
        export_data_btn: field!("export_data_btn"),
        fav_btn: field!("fav_btn"),
        grid_down_btn: field!("grid_down_btn"),
        login_btn: field!("login_btn"),
        main_frame: field_int!("main_frame"),
        ok_btn: field!("ok_btn"),
        password_field: field!("password_field"),
        query_btn: field!("query_btn"),
        query_list_selector: field!("query_list_selector"),
        report_btn: field!("report_btn"),
        select_btn: field!("select_btn"),
        submit_btn: field!("submit_btn"),
        username_field: field!("username_field"),
        st_company_field: field!("st_company_field"),
        st_expected_date_field: field!("st_expected_date_field"),
        st_order_type_field: field!("st_order_type_field"),
        st_repo_field: field!("st_repo_field"),
        ios_century_field: field!("ios_century_field"),
        ios_company_field: field!("ios_company_field"),
        ios_company_from_field: field!("ios_company_from_field"),
        ios_company_to_field: field!("ios_company_to_field"),
        ios_month_field: field!("ios_month_field"),
        ios_month_from_field: field!("ios_month_from_field"),
        ios_month_to_field: field!("ios_month_to_field"),
        ios_repo_field: field!("ios_repo_field"),
        ios_year_field: field!("ios_year_field"),
        adsearch_btn: field!("adsearch_btn"),
        st_repo_add_btn: field!("st_repo_add_btn"),
        repo_select: field!("repo_select"),
        repo_add_index0: field!("repo_add_index0"),
        repo_add_index1: field!("repo_add_index1"),
        aq_add_value_more: field!("aq_add_value_more"),
    })
}

/// Parses `jde` table in `config.toml`.
pub fn parse_config_jde_table(config: &Value) -> Result<Jde> {
    let locator = config
        .get("jde")
        .map(|x| x.as_table())
        .flatten()
        .ok_or_else(|| Error::new(ErrorKind::Config("table jde".to_owned())))?;

    macro_rules! field {
        ($field:literal) => {
            locator
                .get($field)
                .map(|f| f.as_str())
                .flatten()
                .ok_or_else(|| Error::new(ErrorKind::Config(concat!("jde.", $field).to_owned())))?
                .to_string()
        };
    }

    Ok(Jde {
        username: field!("username"),
        password: field!("password"),
        address: field!("address"),
        browser_path: field!("browser_path"),
    })
}
