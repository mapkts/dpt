use crate::jde::{Jde, Locators};
use crate::Result;

use std::time::Duration;

use fantoccini::{elements::Element, Client, ClientBuilder, Locator};
use serde_json::{json, Map};
use tokio::time::sleep;

/// Logins to JDE and returns a client handle.
pub async fn login(driver_port: u32, jde: &Jde, locators: &Locators) -> Result<Client> {
    let webdriver = format!("http://localhost:{}", driver_port);

    let mut cap = Map::new();
    cap.insert(
        "moz:firefoxOptions".to_string(),
        json!({
            "prefs": {
                "browser.downLoad.folderList": 1,
                "browser.helperApps.neverAsk.saveToDisk":
                    "application/vnd.ms-excel,text/csv,text/html,application/octet-stream,image/png",
                "browser.download.manager.showWhenStarting": false,
                "pdfjs.disabled": true,
            },
        }),
    );

    let mut client = ClientBuilder::native()
        .capabilities(cap)
        .connect(&webdriver)
        .await?;

    client.goto(&jde.address).await?;

    // Fills login infos and logins.
    client
        .find(Locator::Css(&locators.username_field))
        .await?
        .send_keys(&jde.username)
        .await?;
    client
        .find(Locator::Css(&locators.password_field))
        .await?
        .send_keys(&jde.password)
        .await?;
    client
        .find(Locator::Css(&locators.login_btn))
        .await?
        .click()
        .await?;

    // Refreshes current tab if DOM is loaded correctly.
    loop {
        match wait_timeout_ms(&mut client, &locators.fav_btn, 3000).await {
            Err(_) => {
                client.refresh().await?;
                // Fills login infos and logins.
                client
                    .find(Locator::Css(&locators.username_field))
                    .await?
                    .send_keys(&jde.username)
                    .await?;
                client
                    .find(Locator::Css(&locators.password_field))
                    .await?
                    .send_keys(&jde.password)
                    .await?;
                client
                    .find(Locator::Css(&locators.login_btn))
                    .await?
                    .click()
                    .await?;
            }
            Ok(_) => break,
        }
    }

    Ok(client)
}

async fn wait_timeout_ms(c: &mut Client, selector: &str, timeout: u64) -> Result<Element> {
    match c.find(Locator::Css(selector)).await {
        Ok(elt) => return Ok(elt),
        Err(_) => {
            sleep(Duration::from_millis(timeout)).await;
            match c.find(Locator::Css(selector)).await {
                Ok(elt) => return Ok(elt),
                Err(e) => return Err(e.into()),
            }
        }
    }
}
