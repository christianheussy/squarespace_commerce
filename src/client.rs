use std::env;

use anyhow::{Context, Result};
use url_builder::URLBuilder;

/// A client for interacting with the Squarespace Commerce API.
pub struct Client {
    /// The API key used for authentication.
    pub(crate) api_key: String,
    /// The user agent string to include in requests.
    pub(crate) user_agent: String,
}

impl Client {
    const API_VERSION: &'static str = "1.0";

    /// Creates a new client with the provided `api_key` and `user_agent`.
    pub fn new(api_key: impl AsRef<str>, user_agent: impl AsRef<str>) -> Self {
        Self {
            api_key: api_key.as_ref().to_owned(),
            user_agent: user_agent.as_ref().to_owned(),
        }
    }

    /// Creates a new client using the `SS_API_KEY` and `SS_USER_AGENT`
    /// environment variables.
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            api_key: env::var("SS_API_KEY").context("SS_API_KEY is not in the environment")?,
            user_agent: env::var("SS_USER_AGENT")
                .context("SS_USER_AGENT is not in the environment")?,
        })
    }

    pub(crate) async fn get_default_get_request(&self, url: String) -> Result<reqwest::Response> {
        // TODO: Create and store a `reqwest::Client` in the `Client` struct for reuse.
        let client = reqwest::Client::new();
        client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("User-Agent", &self.user_agent)
            .send()
            .await
            .map_err(Into::into)
    }

    pub(crate) fn base_url_builder() -> URLBuilder {
        let mut url = URLBuilder::new();
        url.set_protocol("https")
            .set_host("api.squarespace.com")
            .add_route(Self::API_VERSION)
            .add_route("commerce");
        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_url() {
        let url = Client::base_url_builder().build();
        let test_url = format!(
            "https://api.squarespace.com/{}/commerce",
            Client::API_VERSION
        );
        assert_eq!(test_url, url);
    }
}
