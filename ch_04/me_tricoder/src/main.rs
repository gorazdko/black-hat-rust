mod model;
mod ports;
mod subdomains;

use model::Subdomain;
use ports::scan_ports;
use rayon::prelude::*;
//use reqwest::blocking;
use reqwest::Client;
use subdomains::enumerate;

use reqwest::redirect;
use std::{env, time::Duration};

use futures::stream;
use futures::StreamExt;

mod modules;
//use gitlab_open_registration;

mod error;
pub use error::Error;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let http_timeout = Duration::from_secs(5);
    let http_client = Client::builder().timeout(http_timeout).build()?;
    let ports_concurrency = 100;
    let subdomains_concurrency = 100;
    //let scan_start = Instant::now();
    let target = "kerkour.com";
    let subdomains = subdomains::enumerate(http_client, target).await.unwrap();

    let scan_result: Vec<Subdomain> = stream::iter(subdomains.into_iter())
        .map(|s| ports::scan_ports(ports_concurrency, s))
        .buffer_unordered(subdomains_concurrency)
        .collect()
        .await;

    println!("result: {:?}", scan_result);

    // vulnerabilities:
    //stream::iter(modules.into_iter()).map

    // create urls aka mergin subdomains with port numbers

    let mut urls: Vec<(Box<dyn modules::HttpModule>, String)> = Vec::new();

    for scan in scan_result {
        for port in scan.port {
            let modules = modules::init_modules();
            for module in modules {
                urls.push((module, format!("{}:{}", scan.name.to_string(), port.port)));
            }
        }
    }

    Ok(())
}
