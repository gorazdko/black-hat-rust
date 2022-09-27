mod model;
mod ports;
mod subdomains;

use reqwest::blocking;
use reqwest::blocking::Client;
use subdomains::enumerate;

fn main() -> Result<(), anyhow::Error> {
    println!("Hello World!");

    let c = Client::new(); // https://crt.sh/?q=%25.l-tek.com&output=json
    let target = &format!("https://crt.sh/?q=%25.kerkour.com&output=json");
    //let err = enumerate(c, target);
    //assert!(err.is_ok());
    //println!("{:?}", err.unwrap());

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()
        .unwrap();

    let n = pool.install(|| {
        let x = enumerate(c, target);
        println!("{:?}", x.unwrap());
    });

    Ok(())
}
