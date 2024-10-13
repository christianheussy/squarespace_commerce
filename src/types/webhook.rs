use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderCreate {
    pub id: String,
    pub website_id: String,
    pub subscription_id: String,
    pub topic: String,
    pub data: Data,
    pub created_on: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub order_id: String,
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use rstest::rstest;

    use super::super::tests::test_responses_path;
    use super::*;

    #[rstest]
    fn deserialize_order_create(mut test_responses_path: PathBuf) -> anyhow::Result<()> {
        test_responses_path.push("webhook_order.json");
        let json_str = fs::read_to_string(test_responses_path)?;
        let _my_struct: OrderCreate = serde_json::from_str(&json_str)?;
        Ok(())
    }
}
