use async_trait::async_trait;

use crate::{
    error::Error,
    modules::{HttpFinding, HttpModule, Module},
};
use reqwest::Client;

pub struct GitlabOpenRegistrations {}

#[async_trait]
impl HttpModule for GitlabOpenRegistrations {
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error> {
        return Ok(None);
    }
}

impl Module for GitlabOpenRegistrations {
    fn name(&self) -> String {
        return "GitlabOpenRegistrations".to_string();
    }
    fn description(&self) -> String {
        return "GitlabOpenRegistrations Description".to_string();
    }
}
