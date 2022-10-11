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

fn main() -> Result<(), anyhow::Error> {
    println!("Hello World!");

    //let c = Client::new(); // https://crt.sh/?q=%25.l-tek.com&output=json

    let http_timeout = Duration::from_secs(5);
    let c = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;

    //let target = "https://crt.sh/?q=%25.kerkour.com&output=json";

    //let c = Client::new(); // https://crt.sh/?q=%25.l-tek.com&output=json
    let target = &format!("https://crt.sh/?q=%25.kerkour.com&output=json");

    //let err = enumerate(c, target);
    //assert!(err.is_ok());
    //println!("{:?}", err.unwrap());

    /*
        let x: Vec<Subdomain> = enumerate(c, target)
            .unwrap()
            .into_par_iter()
            .map(|subdomain| scan_ports(subdomain))
            .collect();
        println!("result: {:?}", x);
    */

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()
        .unwrap();

    let n = pool.install(|| {
        let x: Vec<Subdomain> = enumerate(c, target)
            .unwrap()
            .into_par_iter()
            .map(|subdomain| scan_ports(subdomain))
            .collect();
        println!("result: {:?}", x);
    });

    Ok(())
}
