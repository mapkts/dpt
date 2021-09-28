#![allow(unused)]

extern crate jde;
use jde::consts::*;
use jde::jobs::*;
use jde::time::*;
use jde::*;

use std::io::Write;
use std::mem::ManuallyDrop;
use std::thread;
use std::time::Duration;

use clap::{App, Arg, SubCommand};
use env_logger::Env;
use log::{error, info, warn};
use webdriver::error::ErrorStatus;

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    if let Err(e) = run().await {
        error!("{}", e);
    }

    loop {}

    Ok(())
}

async fn run() -> Result<(), fantoccini::error::CmdError> {
    let crate_name = env!("CARGO_PKG_NAME");
    let crate_description = env!("CARGO_PKG_DESCRIPTION");
    let crate_author = env!("CARGO_PKG_AUTHORS");
    let crate_version = env!("CARGO_PKG_VERSION");

    let matches = App::new(crate_name)
        .about(crate_description)
        .author(crate_author)
        .version(crate_version)
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs the JDE client server")
                .arg(
                    Arg::with_name("am")
                        .long("am")
                        .short("a")
                        .help("Downloads morning-ready data"),
                )
                .arg(
                    Arg::with_name("pm")
                        .long("pm")
                        .short("p")
                        .help("Downloads afternoon-ready data"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        // Spawns a server thread that runs the webdriver.
        let mut driver_handle = startup_driver(DRIVER_PORT)?;
        info!("webdriver listens at port {}", DRIVER_PORT);

        // Logins to JDE.
        let mut client = login(URL, USERNAME, PASSWORD, DRIVER_PORT).await?;
        info!("login success");

        // Creates a JDE client.
        let mut c = JdeClient::new(client);

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
        let c = if matches.is_present("pm") {
            info!("start calculating IOS report");
            let c = calculate_ios_report(c, &yyyymm, &yyyymm).await?;

            info!("start downloading ST records (request date: {})", &nextday);
            let c = download_st_records(c, &nextday, "*").await?;
            info!("finish downloading ST records");

            info!("start downloading IOS report");
            let c = download_ios_report(c, &century, &short_year, &month).await?;
            info!("finish downloading IOS report");

            c
        } else if matches.is_present("am") {
            info!("start downloading ST records (request date: {})", &nextday);
            let c = download_st_records(c, &nextday, "*").await?;
            info!("finish downloading ST records");

            info!(
                "start downloading ST records (request date: {}, repository: 11751)",
                &today
            );
            let c = download_st_records(c, &today, "11751").await?;
            info!("finish downloading ST records");

            info!(
                "start calculating IE report (request date: {}, repository: 11751)",
                &yestoday
            );
            let c = calculate_ie_report(c, &yestoday).await?;

            let c = open_report_menu(c).await?;

            c
        } else {
            info!("start calculating IOS report");
            let c = calculate_ios_report(c, &yyyymm, &yyyymm).await?;

            info!(
                "start calculating IE report (request date: {}, repository: 11751, 11759)",
                &yestoday
            );
            let c = calculate_ie_report(c, &yestoday).await?;

            info!("start downloading ST records (request date: {})", &nextday);
            let c = download_st_records(c, &nextday, "*").await?;
            info!("finish downloading ST records");

            info!("start downloading ST records (request date: {})", &today);
            let c = download_st_records(c, &today, "*").await?;
            info!("finish downloading ST records");

            info!("start downloading IOS report");
            let c = download_ios_report(c, &century, &short_year, &month).await?;
            info!("finish downloading IOS report");

            info!(
                "start downloading ST records (request date: {}, repository: 11751)",
                &today
            );
            let c = download_st_records(c, &today, "11751").await?;
            info!("finish downloading ST records");

            // Downloads last week's ST records if today is Sunday.

            // let c = if time::today_is_monday() {
            //     info!(
            //         "start downloading ST records (request date: >={}, repository: 11751)",
            //         &time::today_pred(6)
            //     );
            //     let c = download_st_records_from(c, &time::today_pred(6), "*").await?;
            //     info!("finish downloading ST records");
            //     c
            // } else {
            //     c
            // };

            let c = open_report_menu(c).await?;

            c
        };

        driver_handle.kill()?;

        loop {
            thread::sleep(Duration::from_secs(10));
        }
    };

    Ok(())
}
