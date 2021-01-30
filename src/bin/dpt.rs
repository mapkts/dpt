extern crate dpt;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;

use clap::App;
use dpt::Logger;
use toml::Value;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::sync::Mutex;

macro_rules! error {
    ($msg:tt) => {
        LOGGER.lock().unwrap().error($msg);
        LOGGER
            .lock()
            .unwrap()
            .error("program exited due to previous error");
        process::exit(0);
    };
}

macro_rules! warn {
    ($msg:tt) => {
        LOGGER.lock().unwrap().warn($msg);
    };
}

macro_rules! info {
    ($msg:tt) => {
        LOGGER.lock().unwrap().info($msg);
    };
}

lazy_static! {
    // The path of the executable.
    pub static ref DIR: PathBuf = {
        PathBuf::from(env::args().next().unwrap()).parent().unwrap().to_path_buf()
    };

    // Global logger.
    pub static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::open(DIR.join("../../log.txt")));

    // User configurations.
    pub static ref CONFIG: Value = {
        // FIXME: change inner string to "config.toml"
        let contents = fs::read(DIR.join("../../config.toml"));

        match contents {
            Err(_) => {
                error!("Cannot locate and read config.toml");
            }
            Ok(contents) => {
                let string = String::from_utf8_lossy(&contents);
                let config = string.as_ref().parse::<Value>();
                if config.is_err() {
                    error!("Cannot parse config.toml correctly");
                }
                config.unwrap()
            }
        }
    };
}

fn main() {
    let yaml = load_yaml!("../../cli.yml");
    let _matches = App::from_yaml(yaml).get_matches();

    info!("info");
    warn!("warn");

    use dpt::convert::*;
    use dpt::st::aggregate::*;
    use dpt::st::parse::*;
    use dpt::st::writer::write_mmap;
    use dpt::CsvReader;

    let header = "定单|号,单据|类型,第二项目|号,说明 1|,数量|,价格|计量单位,定单|公司,分部/场所|,请求|日期,定单|日期,售至|,售至地址|名,业务记录|发起人,单|价,总|金额,说明|行 2,次要|数量,定单|类型,辅计量|单位,行|号,计量|单位,暂挂|码,要求的|时间,发运|至,原始|定单号,原始|定单类型,原始|定单公司,第三项目|号,上一|状态,下一|状态,客户采购单|,外币|单价,外币总|金额,父项|号,协议|号,发运|号,提货单|号,交货|号,短|项目号,单据|号,单据|公司,计划|提货,计划|提货时间,原始|承诺,原始|承诺时间,实际|发运,实际|发运时间,发票|日期,取消|日期,总帐|日期,承诺|交货,承诺交|货时间,行|类型,销售|码 1,销售|码 2,销售|码 3,销售|码 4,货币|码,汇|率,本位|币,销售|码 5,定购|数量,发运|数量,延交定单|数量,已取消|数量,价格生效|日期";
    let record = "21022388, ,101012385,泡菜调味料（民福记）500g*28包/件,6,BA,00117,       11751,2021/01/25,2021/01/12,2020001,太二萝岗万达店,YANGPP,9.7885,58.73,500g*28包/件,6,ST,BA,1,BA, ,0,2020001, , , ,101012385,580,620, ,0,,2020001, ,,1785672,,87238871,,,2021/01/12,0,2021/01/12,0,2021/01/24,0,,,,2021/01/12,0,S,1,101, ,10,CNY,,CNY,158,6,6,0,0,2021/01/12,00101,OT,21007453,1,, , , ,20200815141, ,, ";
    let mut rdr = CsvReader::new();
    let hdr = parse_header(header, &CONFIG.clone(), &mut rdr).unwrap();
    println!("{:?}", hdr);
    let range = parse_config_st_headers(&CONFIG.clone());
    // println!("{:?}", range);
    let range = parse_config_store_ranges(&CONFIG.clone());
    // println!("{:?}", range);

    let (mmap, _, _) = aggregate(
        "./202012ST.csv",
        Encoding::GB18030,
        &CONFIG.clone(),
        &mut rdr,
    )
    .unwrap();

    let _ = write_mmap(mmap, ".");
}
