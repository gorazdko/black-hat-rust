use crate::error::Error;
use crate::modules::{HttpFinding, HttpModule, Module};
use async_trait::async_trait;
use reqwest::Client;

pub struct DotEnvDisclosure {}

impl DotEnvDisclosure {
    pub fn new() -> Self {
        return DotEnvDisclosure {};
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_DotEnvDisclosure() {
        let client = Client::new(); // TODO:: connect timeout
        let p = DotEnvDisclosure {};
        let res = p.scan(&client, "http://www.l-tek.com:443").await;

        println!("res: {:?}", res);
    }
}
