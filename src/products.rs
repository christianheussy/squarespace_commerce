use anyhow::Ok;

use crate::{
    client::Client,
    types::products::{Product, ProductResponse},
};

impl Client {
    const PRODUCT_BASE_URL: &'static str = "https://api.squarespace.com/1.1/commerce/products?";
    pub async fn retrieve_all_products(&self) -> anyhow::Result<Vec<Product>> {
        let url = format!("{}?&type=DIGITAL", Self::PRODUCT_BASE_URL);

        let response = self.get_default_get_request(url).await?;

        // Deserialize the response
        let product_response: ProductResponse = response.json().await?;

        Ok(product_response.products)
    }
}
