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

    let modules = modules::init_modules();

    // vulnerabilities:
    //stream::iter(modules.into_iter()).map

    // create urls aka mergin subdomains with port numbers

    let mut urls: Vec<(Box<dyn modules::HttpModule>, String)> = Vec::new();

    for module in modules {
        urls.push((module, format!("{}:{}", "scans.name".to_string(), 1)));
        for scans in &scan_result {
            /*scans
            .port
            .into_iter()
            .for_each(|p| urls.push((module, format!("{}:{}", scans.name, p.port)))) */
        }
    }

    Ok(())
}
