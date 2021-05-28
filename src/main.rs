extern crate dpt;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;

// lib
use dpt::convert::EncodeType;
use dpt::iter::FilePathEntries;
use dpt::Logger;
use dpt::{Error, ErrorKind, Result};

// third-party
use admerge::{FileMerger, Newline, Skip};
use clap::{App, ArgMatches};
use toml::Value;

// std
use std::borrow::Cow;
use std::env;
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

macro_rules! error {
    ($msg:tt) => {
        LOGGER.lock().unwrap().error(&format!("{}", $msg));
        LOGGER
            .lock()
            .unwrap()
            .error("process didn't exit successfully");
        process::exit(0);
    };
    ($format_str:literal, $($msg:tt)*) => {
        LOGGER.lock().unwrap().error(&format!($format_str, $($msg)*));
        LOGGER
            .lock()
            .unwrap()
            .error("process didn't exit successfully");
        process::exit(0);
    };
}

// macro_rules! warn {
//     ($msg:tt) => {
//         LOGGER.lock().unwrap().warn(&format!("{}", $msg));
//     };
//     ($format_str:literal, $($msg:tt)*) => {
//         LOGGER.lock().unwrap().warn(&format!($format_str, $($msg)*));
//     };
// }

macro_rules! info {
    ($msg:tt) => {
        LOGGER.lock().unwrap().info(&format!("{}", $msg));
    };
    ($format_str:literal, $($msg:tt)*) => {
        LOGGER.lock().unwrap().info(&format!($format_str, $($msg)*));
    };
}

lazy_static! {
    // The path of the executable.
    pub static ref DIR: PathBuf = {
        PathBuf::from(env::args().next().unwrap()).parent().unwrap().to_path_buf()
    };

    // Global logger.
    pub static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::open(DIR.join("log.txt")));

    // User configurations.
    pub static ref CONFIG: Value = {
        // FIXME: change inner string to "config.toml" when it's time to ship production.
        let contents = fs::read(DIR.join("config.toml"));

        match contents {
            Err(_) => {
                error!("cannot locate and read config.toml");
            }
            Ok(contents) => {
                let string = String::from_utf8_lossy(&contents);
                let config = string.as_ref().parse::<Value>();
                if config.is_err() {
                    error!("cannot parse config.toml correctly");
                }
                config.unwrap()
            }
        }
    };
}

#[tokio::main]
async fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Err(e) = run(&matches).await {
        error!(e);
    }
}

async fn run(matches: &ArgMatches<'_>) -> Result<()> {
    // run subcommand `st`.
    if let Some(m) = matches.subcommand_matches("st") {
        use dpt::st::aggregate::aggregate;
        use dpt::st::writer::write_aggregation_result;
        use dpt::CsvReader;

        let paths: Vec<OsString> = if m.is_present("input") {
            m.values_of_os("input")
                .unwrap()
                .map(|s| s.to_os_string())
                .collect()
        } else if m.is_present("directory") {
            let dir = m.value_of("directory").unwrap();
            FilePathEntries::from_dir_with(dir, vec!["csv"])?
                .into_iter()
                .map(|p| p.into_os_string())
                .collect()
        } else {
            Default::default()
        };

        let mut should_cleanup = false;
        let file = if paths.len() > 1 {
            let mut temp = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open("./_dpt_temp_.csv")?;
            let mut merger = FileMerger::new();
            merger.skip_head(Skip::LinesOnce(1));
            merger.skip_tail(Skip::Lines(1));
            merger.with_paths(paths, &mut temp)?;

            let file = File::open("./_dpt_temp_.csv")?;
            should_cleanup = true;
            file
        } else if paths.len() == 1 {
            let file = File::open(&paths[0])?;
            file
        } else {
            unreachable!()
        };

        let encoding = if m.is_present("encoding") {
            match m.value_of("encoding").unwrap() {
                "GB18030" => EncodeType::GB18030,
                "GBK" => EncodeType::GBK,
                "UTF8" => EncodeType::UTF8,
                _ => unreachable!(),
            }
        } else {
            EncodeType::GB18030
        };

        let out_dir = if m.is_present("output") {
            let dir = m.value_of_os("output").unwrap();
            let path: PathBuf = dir.into();
            if !path.is_dir() {
                fs::create_dir(&dir)?;
            }
            dir.to_os_string()
        } else {
            let mut dir = DIR.clone();
            dir.push("st");

            if !dir.is_dir() {
                fs::create_dir(&dir)?;
            }
            dir.into_os_string()
        };

        let strict = m.is_present("strict");

        info!("start aggregating data");

        let mut rdr = CsvReader::new();
        let maps = aggregate(file, encoding, &CONFIG.clone(), &mut rdr, strict)?;

        // remove temp file
        if should_cleanup {
            fs::remove_file("./_dpt_temp_.csv")?;
        }

        write_aggregation_result(maps, out_dir.to_str().unwrap())?;

        info!("aggregation process has finished");
        info!({
            format!(
                "result files can be found in path `{}`",
                fs::canonicalize(out_dir)?.display()
            )
        });
    };

    // run subcommand `jde`.
    if let Some(m) = matches.subcommand_matches("jde") {
        use dpt::jde::{self, *};
        use jde::jobs::*;
        use jde::time::*;

        let jde = jde::parse_config_jde_table(&CONFIG.clone())?;
        let locators = jde::parse_config_locator_table(&CONFIG.clone())?;
        let driver_port = m.value_of("port").unwrap().parse::<u32>().unwrap();

        // Spawns a server thread that runs the webdriver.
        let _driver_handle = startup_driver(driver_port)?;
        info!("webdriver listens at port {}", driver_port);

        // Logins to JDE.
        let client = login(driver_port, &jde, &locators).await?;
        info!("login success");

        // Creates a JDE client.
        let c = JdeClient::new(client);

        // Time related
        let yyyymm = yyyymm();
        let today = today();
        let yestoday = yestoday();
        let nextday = nextday();
        let century = century();
        let short_year = short_year();
        let month = month();

        // NOTE: We must keep a handle to the client otherwise the browser will be closed when
        // jobs were finished.
        let _c = if matches.is_present("pm") {
            info!("start calculating IOS report");
            let c = calculate_ios_report(c, &yyyymm, &yyyymm, &locators).await?;

            info!("start downloading ST records (request date: {})", nextday);
            let c = download_st_records(c, &nextday, "*", &locators).await?;
            info!("finish downloading ST records");

            info!("start downloading IOS report");
            let c = download_ios_report(c, &century, &short_year, &month, &locators).await?;
            info!("finish downloading IOS report");

            c
        } else if matches.is_present("am") {
            info!("start downloading ST records (request date: {})", nextday);
            let c = download_st_records(c, &nextday, "*", &locators).await?;
            info!("finish downloading ST records");

            info!(
                "start downloading ST records (request date: {}, repository: 11751)",
                today
            );
            let c = download_st_records(c, &today, "11751", &locators).await?;
            info!("finish downloading ST records");

            info!(
                "start calculating IE report (request date: {}, repository: 11751)",
                yestoday
            );
            let c = calculate_ie_report(c, &yestoday, &locators).await?;

            let c = open_report_menu(c, &locators).await?;

            c
        } else {
            info!("start calculating IOS report");
            let c = calculate_ios_report(c, &yyyymm, &yyyymm, &locators).await?;

            info!(
                "start calculating IE report (request date: {}, repository: 11751, 11759)",
                yestoday
            );
            let c = calculate_ie_report(c, &yestoday, &locators).await?;

            info!("start downloading ST records (request date: {})", nextday);
            let c = download_st_records(c, &nextday, "*", &locators).await?;
            info!("finish downloading ST records");

            info!("start downloading ST records (request date: {})", today);
            let c = download_st_records(c, &today, "*", &locators).await?;
            info!("finish downloading ST records");

            info!(
                "start downloading ST records (request date: {}, repository: 11752)",
                today
            );
            let c = download_st_records(c, &today, "11752", &locators).await?;
            info!("finish downloading ST records");

            info!(
                "start downloading ST records (request date: {}, repository: 11751)",
                today
            );
            let c = download_st_records(c, &today, "11751", &locators).await?;
            info!("finish downloading ST records");

            info!("start downloading IOS report");
            let c = download_ios_report(c, &century, &short_year, &month, &locators).await?;
            info!("finish downloading IOS report");

            // Downloads last week's ST records if today is Sunday.
            let c = if time::today_is_monday() {
                info!(
                    "start downloading ST records (request date: >={}, repository: 11751)",
                    &time::today_pred(6)
                );
                let c = download_st_records_from(c, &time::today_pred(6), "*", &locators).await?;
                info!("finish downloading ST records");
                c
            } else {
                c
            };

            let c = open_report_menu(c, &locators).await?;

            c
        };

        // let user manually exit this program when running command `jde`.
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })
        .expect("error setting ctrl-c handler");

        info!("job done! use ctrl-c to exit process");
        while running.load(Ordering::SeqCst) {}
        info!("process exited successfully");

        // driver_handle.kill()?;
    }

    // run subcommand `concat`.
    if let Some(m) = matches.subcommand_matches("concat") {
        let input: Vec<PathBuf> = if let Some(file_paths) = m.values_of("input") {
            file_paths.map(|s| PathBuf::from(s)).collect()
        } else if let Some(dir) = m.value_of("directory") {
            FilePathEntries::from_dir(dir)?.into_iter().collect()
        } else {
            vec![]
        };

        let out_dir = m.value_of("output").unwrap();
        let skip_start = if let Some(n) = m.value_of("skip-start") {
            n.parse::<usize>().map_err(|_| {
                Error::new(ErrorKind::Other(Cow::Owned(format!(
                    "skip-start only accepts number, but found `{}`",
                    n
                ))))
            })?
        } else {
            0
        };
        let skip_end = if let Some(n) = m.value_of("skip-end") {
            n.parse::<usize>().map_err(|_| {
                Error::new(ErrorKind::Other(Cow::Owned(format!(
                    "skip-end only accepts number, but found `{}`",
                    n
                ))))
            })?
        } else {
            0
        };
        let headless = m.is_present("headless");
        let headonce = m.is_present("headonce");
        let newline = m.is_present("newline");

        let mut merger = FileMerger::new();
        let mut writer = OpenOptions::new().write(true).create(true).open(out_dir)?;
        if skip_start > 0 {
            merger.skip_head(Skip::Lines(skip_start));
        } else if headless {
            merger.skip_head(Skip::Lines(1));
        } else if headonce {
            merger.skip_head(Skip::LinesOnce(1));
        }
        if skip_end > 0 {
            merger.skip_tail(Skip::Lines(skip_end));
        }
        if newline {
            merger.force_ending_newline(Newline::Crlf);
        }
        merger.with_paths(input, &mut writer)?;

        info!("The given files have been successfully merged");
        info!({
            format!(
                "result file can be found in path `{}`",
                fs::canonicalize(out_dir)?.display()
            )
        });
    }

    // run subcommand `convert`.
    if let Some(m) = matches.subcommand_matches("convert") {
        use dpt::convert::*;

        if let Some(path) = m.value_of("file") {
            let out_dir = m.value_of("output").unwrap();
            xlsx2csv(path, out_dir)?;
        }
    }

    Ok(())
}
