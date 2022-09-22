use reqwest::blocking;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use std::error::Error;

#[derive(Debug)]
pub struct Subdomain {
    name: String,
    port: Vec<u8>,
}

#[derive(Deserialize, Debug)]
struct Names {
    name_value: String,
}

pub fn enumerate(http_client: Client, target: &str) -> Result<Vec<Subdomain>, Box<dyn Error>> {
    let name_value: Vec<Names> = http_client.get(target).send()?.json()?;
    //println!("{:?}", name_value);

    let x: Vec<&str> = name_value
        .iter()
        .flat_map(|name| name.name_value.split("\n"))
        .filter(|x| x != &target)
        .filter(|x| !x.contains('*'))
        .collect();

    //println!("evo: {:?}", x);

    let x: Vec<Subdomain> = x
        .iter()
        .map(|x| Subdomain {
            name: x.to_string(),
            port: Vec::new(),
        })
        .collect();

    //let v: Vec<Subdomain> = Vec::new();
    Ok(x)
}

#[cfg(test)]
mod tests {
    use crate::subdomains::*;
    #[test]
    fn test_enumerate() {
        let c = Client::new(); // https://crt.sh/?q=%25.l-tek.com&output=json
        let target = &format!("https://crt.sh/?q=%25.kerkour.com&output=json");
        let err = enumerate(c, target);
        assert!(err.is_ok());
        println!("{:?}", err.unwrap());
    }
}
