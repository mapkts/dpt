use crate::jde::client::JdeClient;
use crate::jde::Locators;

pub async fn calculate_ios_report(
    mut client: JdeClient,
    yyyymm_from: &str,
    yyyymm_to: &str,
    locators: &Locators,
) -> Result<JdeClient, fantoccini::error::CmdError> {
    client
        .wait_click(&locators.fav_btn)
        .await?
        .click_nth_favor(3)
        .await?;

    // switch to main frame
    let mut client = client.enter_frame(locators.main_frame as u16).await?;

    client
        .wait_click(&locators.data_select_btn)
        .await?
        .wait_click(&locators.submit_btn)
        .await?
        .wait_click(&locators.select_btn)
        .await?
        .wait_sendkeys(&locators.ios_company_from_field, "00117")
        .await?
        .wait_sendkeys(&locators.ios_company_to_field, "00117")
        .await?
        .wait_sendkeys(&locators.ios_month_from_field, yyyymm_from)
        .await?
        .wait_sendkeys(&locators.ios_month_to_field, yyyymm_to)
        .await?
        .wait_click(&locators.select_btn)
        .await?
        .wait_click(&locators.ok_btn)
        .await?
        .delay_ms(1000)
        .await;

    Ok(client.enter_parent_frame().await?)
}

pub async fn download_ios_report(
    mut client: JdeClient,
    century: &str,
    year: &str,
    month: &str,
    locators: &Locators,
) -> Result<JdeClient, fantoccini::error::CmdError> {
    client
        .wait_click(&locators.fav_btn)
        .await?
        .click_nth_favor(4)
        .await?;

    // switch to main frame
    let mut client = client.enter_frame(locators.main_frame as u16).await?;

    client
        .select_by_index(&locators.query_list_selector, 0, 100)
        .await?
        .wait_sendkeys(&locators.ios_company_field, "00117")
        .await?
        .sendkeys(&locators.ios_repo_field, "<=11759")
        .await?
        .sendkeys(&locators.ios_century_field, century)
        .await?
        .sendkeys(&locators.ios_year_field, year)
        .await?
        .sendkeys(&locators.ios_month_field, month)
        .await?
        .click(&locators.query_btn)
        .await?
        .wait_click(&locators.grid_down_btn)
        .await?
        .delay_ms(500)
        .await
        .wait_click(&locators.export_data_btn)
        .await?
        .delay_ms(500)
        .await
        .wait_click(&locators.download_btn)
        .await?
        .delay_ms(2000)
        .await
        .wait_click(&locators.close_btn)
        .await?
        .delay_ms(1000)
        .await;

    Ok(client.enter_parent_frame().await?)
}
