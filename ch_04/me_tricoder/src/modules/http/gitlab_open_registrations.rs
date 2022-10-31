use async_trait::async_trait;

use crate::{
    error::Error,
    modules::{HttpFinding, HttpModule, Module},
};
use reqwest::Client;

pub struct GitlabOpenRegistrations {}

impl GitlabOpenRegistrations {
    pub fn new() -> Self {
        return GitlabOpenRegistrations {};
    }
}

#[async_trait]
impl HttpModule for GitlabOpenRegistrations {
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error> {
        let body = http_client.get(endpoint).send().await?.text().await?;
        if body.contains("Gitlab open registrations") {
            return Ok(Some(HttpFinding::GitlabOpenRegistrations(
                endpoint.to_string(),
            )));
        }

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

#[cfg(test)]
mod tests {
    use crate::modules::http::gitlab_open_registrations as go;
    use crate::modules::HttpModule;
    // super::gitlab_open_registrations the keyword is super
    #[tokio::test]
    async fn test_GitlabOpenRegistrations() {
        let client = go::Client::new(); // TODO:: connect timeout
        let p = go::GitlabOpenRegistrations {};
        let res = p.scan(&client, "http://www.l-tek.com:443").await;

        println!("res: {:?}", res);
    }
}
