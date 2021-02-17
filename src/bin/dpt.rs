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
        LOGGER.lock().unwrap().error(&format!("{}", $msg));
        LOGGER
            .lock()
            .unwrap()
            .error("process exits due to previous error");
        process::exit(0);
    };
}

macro_rules! warn {
    ($msg:tt) => {
        LOGGER.lock().unwrap().warn(&format!("{}", $msg));
    };
}

macro_rules! info {
    ($msg:tt) => {
        LOGGER.lock().unwrap().info(&format!("{}", $msg));
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
        // FIXME: change inner string to "config.toml" when it's time to ship production.
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
    let matches = App::from_yaml(yaml).get_matches();

    // run subcommand `st`.
    if let Some(sub_m) = matches.subcommand_matches("st") {
        if sub_m.is_present("input") {}
    }

    // use dpt::convert::*;
    // use dpt::st::aggregate::*;
    // use dpt::st::writer::*;
    // use dpt::CsvReader;

    // let mut rdr = CsvReader::new();

    // match aggregate(
    //     "./202012ST.csv",
    //     EncodeType::GB18030,
    //     &CONFIG.clone(),
    //     &mut rdr,
    //     false,
    // ) {
    //     Ok(maps) => write_aggregation_result(maps, "./").unwrap(),
    //     Err(e) => {
    //         error!(e);
    //     }
    // }

    use dpt::iter::*;

    let mut buf = Vec::new();
    let mut reader = LineReader::new(vec!["1.test.txt", "2.test.txt", "3.test.txt"], true).unwrap();
    loop {
        buf.clear();
        match reader.next_line(&mut buf) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                println!("{:?}", std::str::from_utf8(&buf).unwrap());
            }
            Err(e) => {
                warn!(e);
            }
        }
    }
}
