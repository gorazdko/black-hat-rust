mod model;
mod ports;
mod subdomains;

use model::Subdomain;
use ports::scan_ports;
use rayon::prelude::*;
use reqwest::blocking;
use reqwest::blocking::Client;
use subdomains::enumerate;

use reqwest::redirect;
use std::{env, time::Duration};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    /*
        let http_timeout = Duration::from_secs(10);
        let http_client = Client::builder().timeout(http_timeout).build()?;
        let ports_concurrency = 200;
        let subdomains_concurrency = 100;
        let scan_start = Instant::now();
        let subdomains = subdomains::enumerate(&http_client, target).await?;
        // ...
    */
    Ok(())
}
