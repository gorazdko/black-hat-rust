use crate::error::Error;
use crate::modules::{HttpFinding, HttpModule, Module};
use async_trait::async_trait;
use reqwest::Client;

pub struct DotEnvDisclosure {}

#[async_trait]
impl HttpModule for DotEnvDisclosure {
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error> {
        let url = format!("{}/.env", endpoint);
        let res = http_client.get(url.clone()).send().await?;

        if res.status() == reqwest::StatusCode::OK {
            return Ok(Some(HttpFinding::DotEnvDisclosure(url)));
        }
        Ok(None)
    }
}

impl Module for DotEnvDisclosure {
    fn name(&self) -> String {
        return "DotEnvDisclosure".to_string();
    }
    fn description(&self) -> String {
        return "DotEnvDisclosure Description".to_string();
    }
}
