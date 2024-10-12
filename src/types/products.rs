use serde::{Deserialize, Serialize};

use super::pagination::Pagination;

#[derive(Debug, Deserialize, Serialize)]
pub struct Product {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductResponse {
    pub pagination: Pagination,
    pub products: Vec<Product>,
}
