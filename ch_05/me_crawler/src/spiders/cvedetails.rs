//pub async fn process(&self, item: Self::Item) -> Result<(), Error> {
//    println!("test process");
//}

use crate::error::Error;
use async_trait::async_trait;
use reqwest::Client;

pub struct CveDetailsSpider {
    http_client: Client,
}

#[async_trait]
impl super::Spider for CveDetailsSpider {
    type Item = Cve;

    fn name(&self) -> String {
        return "ss".to_string();
    }
    fn start_urls(&self) -> Vec<String> {
        Vec::from(["dd".to_string()])
    }
    async fn scrape(&self, url: String) -> Result<(Vec<Self::Item>, Vec<String>), Error> {
        Err(Error::Reqwest(String::from("Hello, world!")))
    }
    async fn process(&self, item: Self::Item) -> Result<(), Error> {
        Err(Error::Reqwest(String::from("Hello, world!")))
    }
}

#[derive(Debug, Clone)]
pub struct Cve {
    name: String,
    url: String,
    cwe_id: Option<String>,
    cwe_url: Option<String>,
    vulnerability_type: String,
    publish_date: String,
    update_date: String,
    score: f32,
    access: String,
    complexity: String,
    authentication: String,
    confidentiality: String,
    integrity: String,
    availability: String,
}
