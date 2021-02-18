use std::process::{Child, Command, Stdio};

pub fn startup_driver(port: u32) -> std::io::Result<Child> {
    Command::new("geckodriver.exe")
        .arg("-p")
        .arg(format!("{}", port))
        .arg("-b")
        .arg(format!(
            "{}",
            r#"C:/Users/mapkts/AppData/Local/Mozilla Firefox/firefox.exe"#
        ))
        .stdout(Stdio::null())
        .spawn()

    // Command::new("chromedriver.exe")
    //     .arg(format!("--port={}", port))
    //     .spawn()
}
