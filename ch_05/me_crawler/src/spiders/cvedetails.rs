//pub async fn process(&self, item: Self::Item) -> Result<(), Error> {
//    println!("test process");
//}

use crate::error::Error;
use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;

use select::{
    document::Document,
    predicate::{Attr, Class, Name, Predicate},
};

pub struct CveDetailsSpider {
    http_client: Client,
}

impl CveDetailsSpider {
    fn new() -> Self {
        let builder = Client::builder();
        let tmout = Duration::from_millis(7000);
        let client = builder.timeout(tmout).build().unwrap(); // TODO

        CveDetailsSpider {
            http_client: client,
        }
    }
}

#[async_trait]
impl super::Spider for CveDetailsSpider {
    type Item = Cve;

    fn name(&self) -> String {
        String::from("cvedetails")
    }

    fn start_urls(&self) -> Vec<String> {
        vec!["https://www.cvedetails.com/vulnerability-list/vulnerabilities.html".to_string()]
    }

    async fn scrape(&self, url: String) -> Result<(Vec<Self::Item>, Vec<String>), Error> {
        println!("*** 0[scrape]]");
        let res = self.http_client.get(url).send().await?.text().await?;
        println!("***[scrape]]");

        let next_pages_link = Document::from(res.as_str());

        //println!("++res {:?}", next_pages_link);

        println!("*****************************  ******************************");

        let res = next_pages_link.find(Attr("id", "pagingb").descendant(Name("a")));

        let mut links = Vec::new();
        let mut i = 0;
        for r in res {
            links.push(r.attr("href"));
            i += 1;
            if i > 10 {
                break;
            }
        }

        println!("**strings: {:?}", links);
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

use crate::spiders::Spider;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_crawler() {
        let crawler = CveDetailsSpider::new();

        println!("crawler name: {:?}", crawler.name());

        let strg = crawler
            .scrape(crawler.start_urls()[0].clone())
            .await
            .unwrap();
        println!("crawler respond: {:?}", strg);
    }
}
