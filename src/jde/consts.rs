#![doc(hidden)]

pub const DRIVER_PORT: u32 = 8888;

// login infos
pub const URL: &'static str = "http://192.168.0.232:8003/jde/E1Menu.maf";
pub const USERNAME: &'static str = "tanxl";
pub const PASSWORD: &'static str = "TANX-1996";

// Frame ids
pub const MAIN_FRAME: u16 = 8;

// CSS selectors
pub const USERNAME_FIELD: &'static str = "#User";
pub const PASSWORD_FIELD: &'static str = "#Password";
pub const LOGIN_BTN: &'static str = ".buttonstylenormal.margin-top5";
pub const FAV_BTN: &'static str = "#drop_fav_menus";
pub const REPORT_BTN: &'static str = "#drop_reportmenu";
pub const EXPORT_DATA_BTN: &'static str = "#jdehtmlExportData";
pub const QUERY_BTN: &'static str = "#hc_Find";
pub const CLOSE_BTN: &'static str = "#hc_Close";
pub const OK_BTN: &'static str = "#hc_OK";
pub const SELECT_BTN: &'static str = "#hc_Select";
pub const GRID_DOWN_BTN: &'static str = "#GOTOLAST0_1";
pub const DOWNLOAD_BTN: &'static str = "#hc1";
pub const QUERY_LIST_SELECTOR: &'static str = "#AQFormQueryList";
pub const DATA_SELECT_BTN: &'static str = "#C0_23";
pub const SUBMIT_BTN: &'static str = "#hc0";

pub const ST_ORDER_TYPE_FIELD: &'static str = "#C0_13";
pub const ST_COMPANY_FIELD: &'static str = "input[name='qbe0_1.9']";
pub const ST_REPO_FIELD: &'static str = "input[name='qbe0_1.10']";
pub const ST_EXPECTED_DATE_FIELD: &'static str = "input[name='qbe0_1.11']";

pub const IOS_COMPANY_FROM_FIELD: &'static str = "#PO6T0";
pub const IOS_COMPANY_TO_FIELD: &'static str = "#PO7T0";
pub const IOS_MONTH_FROM_FIELD: &'static str = "#PO8T0";
pub const IOS_MONTH_TO_FIELD: &'static str = "#PO9T0";
pub const IOS_COMPANY_FIELD: &'static str = "input[name='qbe0_1.0']";
pub const IOS_REPO_FIELD: &'static str = "input[name='qbe0_1.2']";
pub const IOS_CENTURY_FIELD: &'static str = "input[name='qbe0_1.4']";
pub const IOS_YEAR_FIELD: &'static str = "input[name='qbe0_1.5']";
pub const IOS_MONTH_FIELD: &'static str = "input[name='qbe0_1.6']";
