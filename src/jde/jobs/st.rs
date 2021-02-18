use crate::jde::client::JdeClient;
use crate::jde::Locators;

pub async fn download_st_records(
    mut client: JdeClient,
    date: &str,
    repo: &str,
    locators: &Locators,
) -> Result<JdeClient, fantoccini::error::CmdError> {
    client
        .wait_click(&locators.fav_btn)
        .await?
        .click_nth_favor(2)
        .await?;

    let mut client = client.enter_frame(locators.main_frame as u16).await?;

    client
        .wait_sendkeys(&locators.st_order_type_field, "*")
        .await? // fills table fields
        .wait_sendkeys(&locators.st_company_field, "00117")
        .await?
        .wait_sendkeys(&locators.st_repo_field, repo)
        .await?
        .wait_sendkeys(&locators.st_expected_date_field, date)
        .await?
        .wait_click(&locators.query_btn)
        .await? // start querying
        .delay_ms(500)
        .await
        .wait_delay_click(&locators.grid_down_btn, 500)
        .await?
        .delay_ms(500)
        .await
        .wait_delay_click(&locators.export_data_btn, 500)
        .await? // export data
        .delay_ms(500)
        .await
        .wait_delay_click(&locators.download_btn, 500)
        .await?
        .wait_delay_click(&locators.close_btn, 4000)
        .await?
        .delay_ms(1000)
        .await; // finish download

    Ok(client.enter_parent_frame().await?)
}

pub async fn download_st_records_from(
    mut client: JdeClient,
    date: &str,
    repo: &str,
    locators: &Locators,
) -> Result<JdeClient, fantoccini::error::CmdError> {
    client
        .wait_click(&locators.fav_btn)
        .await?
        .click_nth_favor(2)
        .await?;

    let mut client = client.enter_frame(locators.main_frame as u16).await?;

    client
        .wait_sendkeys(&locators.st_order_type_field, "*")
        .await? // fills table fields
        .wait_sendkeys(&locators.st_company_field, "00117")
        .await?
        .wait_sendkeys(&locators.st_repo_field, repo)
        .await?
        .wait_sendkeys(&locators.st_expected_date_field, &format!(">={}", date))
        .await?
        .wait_click(&locators.query_btn)
        .await? // start querying
        .delay_ms(500)
        .await
        .wait_delay_click(&locators.grid_down_btn, 500)
        .await?
        .delay_ms(500)
        .await
        .wait_delay_click(&locators.export_data_btn, 500)
        .await? // export data
        .delay_ms(500)
        .await
        .wait_delay_click(&locators.download_btn, 500)
        .await?
        .wait_delay_click(&locators.close_btn, 30000)
        .await?
        .delay_ms(1000)
        .await; // finish download

    Ok(client.enter_parent_frame().await?)
}
