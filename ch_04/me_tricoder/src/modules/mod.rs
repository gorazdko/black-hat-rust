//use futures::stream;
//use futures::StreamExt;

use crate::Error;

use async_trait::async_trait;

use reqwest::Client;

mod http;

pub trait Module {
    fn name(&self) -> String;
    fn description(&self) -> String;
}

#[async_trait]
pub trait SubdomainModule: Module {
    async fn enumerate(&self, domain: &str) -> Result<Vec<String>, Error>;
}

#[async_trait]
pub trait HttpModule: Module {
    async fn scan(
        &self,
        http_client: &Client,
        endpoint: &str,
    ) -> Result<Option<HttpFinding>, Error>;
}

#[derive(Debug, Clone)]
pub enum HttpFinding {
    GitlabOpenRegistrations(String),
    DsStoreFileDisclosure(String),
    DotEnvDisclosure(String),
}

pub fn init_modules() -> Vec<Box<dyn HttpModule>> {
    vec![
        Box::new(http::GitlabOpenRegistrations::new()),
        Box::new(http::DotEnvDisclosure::new()),
    ]
}
