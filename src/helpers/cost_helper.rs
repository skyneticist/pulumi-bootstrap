use reqwest::Client;
use reqwest::Error;
use serde::Deserialize;
use std::error::Error as StdError;

#[derive(Deserialize, Debug)]
struct Item {
    arm_sku_name: String,
    retail_price: f64,
    unit_of_measure: String,
    arm_region_name: String,
    meter_name: String,
    product_name: String,
}

#[derive(Deserialize, Debug)]
struct PricingResponse {
    items: Vec<Item>,
    next_page_link: Option<String>,
}

fn build_pricing_table(json_data: &PricingResponse, table_data: &mut Vec<Vec<String>>) {
    for item in &json_data.items {
        table_data.push(vec![
            item.arm_sku_name.clone(),
            item.retail_price.to_string(),
            item.unit_of_measure.clone(),
            item.arm_region_name.clone(),
            item.meter_name.clone(),
            item.product_name.clone(),
        ]);
    }
}

async fn get_pricing_data(
    client: &Client,
    api_url: &str,
    query: &str,
) -> Result<PricingResponse, Box<dyn StdError>> {
    let response = client
        .get(api_url)
        .query(&[("api-version", "2021-10-01-preview"), ("$filter", query)])
        .send()
        .await?;

    if !response.status().is_success() {
        return Err("Failed to fetch pricing data".into());
    }

    let json_data: PricingResponse = response.json().await?;
    Ok(json_data)
}

pub async fn calculate() -> Result<(), Box<dyn StdError>> {
    let client = Client::new();
    let api_url = "https://prices.azure.com/api/retail/prices";
    let query = "armRegionName eq 'southcentralus' and armSkuName eq 'Standard_NP20s' and priceType eq 'Consumption' and contains(meterName, 'Spot')";

    let mut table_data = vec![vec![
        "SKU".to_string(),
        "Retail Price".to_string(),
        "Unit of Measure".to_string(),
        "Region".to_string(),
        "Meter".to_string(),
        "Product Name".to_string(),
    ]];

    // Fetch the initial pricing data
    let mut json_data = get_pricing_data(&client, api_url, query).await?;
    build_pricing_table(&json_data, &mut table_data);

    // Paginate through the next pages, if any
    while let Some(next_page) = json_data.next_page_link {
        let response: PricingResponse = client.get(&next_page).send().await?.json().await?;
        json_data = response;
        build_pricing_table(&json_data, &mut table_data);
    }

    // Print the result in a table format
    for row in table_data {
        println!("{}", row.join(" | "));
    }

    Ok(())
}
