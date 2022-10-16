//use reqwest::blocking;
use reqwest::Client;

use serde::{Deserialize, Serialize};

use crate::model::Subdomain;

#[derive(Deserialize, Debug)]
struct Names {
    name_value: String,
}

use std::error::Error;

use std::collections::HashSet;

use trust_dns_resolver::AsyncResolver;

use trust_dns_resolver::name_server::GenericConnection;
use trust_dns_resolver::name_server::GenericConnectionProvider;
use trust_dns_resolver::name_server::TokioRuntime;
type DnsRsolver = AsyncResolver<GenericConnection, GenericConnectionProvider<TokioRuntime>>;

//use tokio_stream::{self as stream, StreamExt};

async fn resolves(resolver: DnsRsolver, hostname: String) -> bool {
    let res = resolver.lookup_ip(hostname).await.is_ok();
    res
}

use futures::stream;
use futures::StreamExt;

pub async fn enumerate(
    http_client: Client,
    target: &str,
) -> Result<Vec<Subdomain>, Box<dyn Error>> {
    //println!("target: {:?}", target);

    //println!("++++++++++++++++");

    let dns_resolver = AsyncResolver::tokio_from_system_conf().unwrap();

    let target = &format!("https://crt.sh/?q=%25.{}&output=json", target);

    let name_value: Vec<Names> = http_client.get(target).send().await?.json().await?;
    //println!("name_value {:?}", name_value);

    //println!("***********");

    let mut x: HashSet<&str> = name_value
        .iter()
        .flat_map(|name| name.name_value.split("\n"))
        .filter(|x| x != &target)
        .filter(|x| !x.contains('*'))
        .collect();

    x.insert(&target);

    // check if subdominas actually resolve correctly:
    // todo resolves() chain subdomains through resolve function
    //resolves(dns_resolver)
    // turn subdomains into stream:
    let x: Vec<&str> = stream::iter(x.into_iter())
        .filter_map(|x| {
            let dns_resolver = dns_resolver.clone();
            async move {
                if resolves(dns_resolver, x.to_string()).await {
                    Some(x)
                } else {
                    None
                }
            }
        })
        .collect()
        .await;

    //println!("evo: {:?}", x);

    let x: Vec<Subdomain> = x
        .iter()
        .map(|x| Subdomain {
            name: x.to_string(),
            port: Vec::new(),
        })
        .collect();

    let v: Vec<Subdomain> = Vec::new();
    Ok(x)
}

#[cfg(test)]
mod tests {
    use crate::subdomains::*;
    //#[ignore]
    #[tokio::test]
    async fn test_enumerate() {
        let c = Client::new(); // https://crt.sh/?q=%25.l-tek.com&output=json
        let target = "kerkour.com"; //&format!("https://crt.sh/?q=%25.kerkour.com&output=json");
        let err = enumerate(c, target);
        //assert!(err.is_ok());

        async {
            println!("result: {:?}", err.await.unwrap());
        }
        .await;
    }
}
