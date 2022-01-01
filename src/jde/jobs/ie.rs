use crate::jde::client::JdeClient;
use crate::jde::Locators;

pub async fn calculate_ie_report(
    mut client: JdeClient,
    ymd: &str,
    locators: &Locators,
) -> Result<JdeClient, fantoccini::error::CmdError> {
    client
        .wait_click(&locators.fav_btn)
        .await?
        .click_nth_favor(5)
        .await?;

    let mut client = client.enter_frame(locators.main_frame as u16).await?;

    client
        .wait_click(&locators.data_select_btn)
        .await?
        .wait_click(&locators.submit_btn)
        .await?
        .select_by_value("#RightOperand1", "Literal", 0)
        .await?
        .wait_sendkeys("#LITtf", "00117")
        .await?
        .click(&locators.select_btn)
        .await?
        .select_by_value("#RightOperand3", "Literal", 0)
        .await?
        .wait_sendkeys("#LITtf", ymd)
        .await?
        .click(&locators.select_btn)
        .await?
        .select_by_index("#LeftOperand4", 66, 0)
        .await?
        .select_by_index("#Comparison4", 1, 0)
        .await?
        .select_by_value("#RightOperand4", "Literal", 0)
        .await?
        .click_linktext("值清单", Some("#modelessTabHeaders"))
        .await?
        .wait_sendkeys("#LITtfList", "11751\r\n")
        .await?
        .delay_ms(1000)
        .await
        .sendkeys("#LITtfList", "11761\r\n")
        .await?
        .delay_ms(1000)
        .await
        .wait_delay_click(&locators.select_btn, 50)
        .await?
        .wait_click(&locators.select_btn)
        .await?
        .delay_ms(1000)
        .await
        .wait_click(&locators.ok_btn)
        .await?
        .delay_ms(2000)
        .await;

    Ok(client.enter_parent_frame().await?)
}

pub async fn open_report_menu(
    mut client: JdeClient,
    locators: &Locators
) -> Result<JdeClient, fantoccini::error::CmdError> {
    client
        .wait_click(&locators.report_btn)
        .await?
        .wait_click("#recRptsMenuItem")
        .await?;

    let client = client.enter_frame(locators.main_frame as u16).await?;

    Ok(client.enter_parent_frame().await?)
}
