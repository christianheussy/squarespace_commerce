use log::info;

use crate::{
    client::Client,
    types::orders::{Order, OrderResponse},
};

impl Client {
    /// Retrieves all orders from the Squarespace Commerce API.
    pub async fn retrieve_all_orders(&self) -> anyhow::Result<Vec<Order>> {
        info!("Retrieving all orders");
        let mut url_builder = Self::base_url_builder();
        url_builder.add_route("orders");
        let response = self.get_default_get_request(url_builder.build()).await?;

        let order_response: OrderResponse = response.json().await?;
        Ok(order_response.result)
    }

    /// Retrieves a specific order by its `order_id`.
    pub async fn retrieve_order(&self, order_id: impl AsRef<str>) -> anyhow::Result<Order> {
        info!("Retrieving order with ID: {}", order_id.as_ref()); // More specific log message

        let mut url_builder = Self::base_url_builder();
        url_builder.add_route("orders").add_route(order_id.as_ref());

        let response = self.get_default_get_request(url_builder.build()).await?;

        Ok(response.json().await?)
    }
}
