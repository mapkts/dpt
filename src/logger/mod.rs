use chrono::Local;
use env_logger::Env;
use log::{error, info, warn};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

/// Configures and builds a logger using env_logger.
pub struct Logger {
    write_log: bool,
    log_file: Option<File>,
}

impl Logger {
    pub fn open<P: AsRef<Path>>(path: P) -> Self {
        let file = OpenOptions::new().create(true).append(true).open(path);
        env_logger::Builder::from_env(Env::default().default_filter_or("info"))
            .format_module_path(false)
            .init();

        match file {
            Ok(file) => Logger {
                write_log: true,
                log_file: Some(file),
            },
            Err(_) => {
                warn!("Cannot open log file with write access");
                warn!("File logging is disabled");

                Logger {
                    write_log: false,
                    log_file: None,
                }
            }
        }
    }

    fn write_log(&mut self, ty: &str, msg: &str) {
        if self.write_log {
            let time = Local::now().format("%Y-%m-%d %H:%M:%S");
            let msg = format!("[{} {:<5} npt] {}\n", time, ty.to_uppercase(), msg);

            self.log_file.as_mut().map(|f| {
                if f.write(msg.as_bytes()).is_err() {
                    warn!("Cannot write message to log file");
                }
            });
        }
    }

    pub fn info(&mut self, msg: &str) {
        info!("{}", msg);
        self.write_log("INFO", msg);
    }

    pub fn warn(&mut self, msg: &str) {
        warn!("{}", msg);
        self.write_log("WARN", msg);
    }

    pub fn error(&mut self, msg: &str) {
        error!("{}", msg);
        self.write_log("ERROR", msg);
    }
}
