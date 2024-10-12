use serde::{Deserialize, Serialize};

use super::pagination::Pagination;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub pagination: Pagination,
    pub result: Vec<Order>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCreated {
    pub id: String,
    pub website_id: String,
    pub subscription_id: String,
    pub topic: String,
    pub data: Data,
    pub created_on: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub order_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub id: String,
    pub order_number: String,
    pub created_on: String,
    pub modified_on: String,
    pub channel: String,
    pub testmode: bool,
    pub customer_email: String,
    pub billing_address: Address,
    pub shipping_address: Option<Address>,
    pub fulfillment_status: String,
    pub line_items: Vec<LineItem>,
    pub internal_notes: Vec<InternalNote>,
    pub shipping_lines: Vec<ShippingLine>,
    pub discount_lines: Vec<DiscountLine>,
    pub form_submission: Option<Vec<FormSubmission>>,
    pub fulfillments: Vec<Fulfillment>,
    pub subtotal: Money,
    pub shipping_total: Money,
    pub discount_total: Money,
    pub tax_total: Money,
    pub refunded_total: Money,
    pub grand_total: Money,
    pub channel_name: Option<String>,
    pub external_order_reference: Option<String>,
    pub fulfilled_on: Option<String>,
    pub price_tax_interpretation: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub first_name: String,
    pub last_name: String,
    pub address1: String,
    pub address2: Option<String>,
    pub city: String,
    pub state: String,
    pub country_code: String,
    pub postal_code: String,
    pub phone: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LineItem {
    pub id: String,
    pub variant_id: Option<String>,
    pub sku: Option<String>,
    pub weight: f64,
    pub width: f64,
    pub length: f64,
    pub height: f64,
    pub product_id: Option<String>,
    pub product_name: String,
    pub quantity: u32,
    pub unit_price_paid: Money,
    pub variant_options: Option<Vec<VariantOption>>,
    pub customizations: Option<Vec<Customization>>,
    pub image_url: Option<String>,
    pub line_item_type: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Money {
    pub value: String,
    pub currency: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VariantOption {
    pub value: String,
    pub option_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Customization {
    pub label: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InternalNote {
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShippingLine {
    pub method: String,
    pub amount: Money,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiscountLine {
    pub description: Option<String>, // Deprecated, use `name` instead
    pub name: String,
    pub amount: Money,
    pub promo_code: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FormSubmission {
    pub label: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Fulfillment {
    pub ship_date: String,
    pub carrier_name: String,
    pub service: String,
    pub tracking_number: String,
    pub tracking_url: String,
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use rstest::rstest;

    use super::super::tests::test_responses_path;
    use super::*;

    #[rstest]
    fn test_deser(mut test_responses_path: PathBuf) -> anyhow::Result<()> {
        test_responses_path.push("orders.json");
        let json_str = fs::read_to_string(test_responses_path)?;
        let _my_struct: OrderResponse = serde_json::from_str(&json_str)?;
        Ok(())
    }
}
