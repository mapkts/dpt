use fantoccini::{error::CmdError, Client, Locator};

use crate::jde::time;

pub struct JdeClient {
    client: Client,
}

impl JdeClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn click(&mut self, locator: &str) -> Result<&mut Self, CmdError> {
        self.client
            .find(Locator::Css(locator))
            .await?
            .click()
            .await?;

        Ok(self)
    }

    pub async fn wait_click(&mut self, locator: &str) -> Result<&mut Self, CmdError> {
        self.delay_ms(20).await;

        self.client
            .wait_for_find(Locator::Css(locator))
            .await?
            .click()
            .await?;

        Ok(self)
    }

    pub async fn wait_delay_click(
        &mut self,
        locator: &str,
        delay_ms: u64,
    ) -> Result<&mut Self, CmdError> {
        let elt = self.client.wait_for_find(Locator::Css(locator)).await?;
        self.delay_ms(delay_ms).await;
        elt.click().await?;

        Ok(self)
    }

    pub async fn click_linktext(
        &mut self,
        locator: &str,
        parent_locator: Option<&str>,
    ) -> Result<&mut Self, CmdError> {
        if let Some(parent) = parent_locator {
            self.client
                .find(Locator::Css(parent))
                .await?
                .find(Locator::LinkText(locator))
                .await?
                .click()
                .await?;
        } else {
            self.client
                .find(Locator::LinkText(locator))
                .await?
                .click()
                .await?;
        }

        Ok(self)
    }

    pub async fn wait_click_linktext(
        &mut self,
        locator: &str,
        parent_locator: Option<&str>,
    ) -> Result<&mut Self, CmdError> {
        if let Some(parent) = parent_locator {
            self.client
                .wait_for_find(Locator::Css(parent))
                .await?
                .find(Locator::LinkText(locator))
                .await?
                .click()
                .await?;
        } else {
            self.client
                .wait_for_find(Locator::LinkText(locator))
                .await?
                .click()
                .await?;
        }

        Ok(self)
    }

    pub async fn sendkeys(&mut self, locator: &str, text: &str) -> Result<&mut Self, CmdError> {
        let mut elt = self.client.find(Locator::Css(locator)).await?;
        elt.clear().await?;
        elt.send_keys(text).await?;

        Ok(self)
    }

    pub async fn wait_sendkeys(
        &mut self,
        locator: &str,
        text: &str,
    ) -> Result<&mut Self, CmdError> {
        let mut elt = self.client.wait_for_find(Locator::Css(locator)).await?;
        elt.clear().await?;
        elt.send_keys(text).await?;

        Ok(self)
    }

    pub async fn enter_frame(self, index: u16) -> Result<Self, CmdError> {
        let client = self.client.enter_frame(Some(index)).await?;
        Ok(JdeClient::new(client))
    }

    pub async fn enter_parent_frame(self) -> Result<Self, CmdError> {
        let client = self.client.enter_parent_frame().await?;
        Ok(JdeClient::new(client))
    }

    pub async fn delay_ms(&mut self, delay: u64) -> &mut Self {
        crate::jde::time::delay_ms(delay).await;
        self
    }

    pub async fn select_by_index(
        &mut self,
        locator: &str,
        index: usize,
        interval_ms: u64,
    ) -> Result<&mut Self, CmdError> {
        self.client
            .wait_for_find(Locator::Css(locator))
            .await?
            .click()
            .await?;

        time::delay_ms(interval_ms).await;

        self.client
            .wait_for_find(Locator::Css(locator))
            .await?
            .select_by_index(index)
            .await?;

        Ok(self)
    }

    pub async fn select_by_value(
        &mut self,
        locator: &str,
        value: &str,
        interval_ms: u64,
    ) -> Result<&mut Self, CmdError> {
        self.client
            .wait_for_find(Locator::Css(locator))
            .await?
            .click()
            .await?;

        time::delay_ms(interval_ms).await;

        self.client
            .wait_for_find(Locator::Css(locator))
            .await?
            .select_by_value(value)
            .await?;

        Ok(self)
    }

    pub async fn click_nth_favor(&mut self, nth: u8) -> Result<&mut Self, CmdError> {
        self.client
            .wait_for_find(Locator::Css("#e1MMenuFav:nth-child(1)"))
            .await?
            .find(Locator::Css(&format!("div:nth-child({})", nth)))
            .await?
            .find(Locator::Css("a"))
            .await?
            .click()
            .await?;

        Ok(self)
    }

    pub fn into_inner(self) -> Client {
        self.client
    }

    pub async fn execute(&mut self, script: &str) -> Result<&mut Self, CmdError> {
        self.client.execute(script, vec![]).await?;
        Ok(self)
    }
}
