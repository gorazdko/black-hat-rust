//use crate::model::Subdomain;

use crate::model::{Port, Subdomain};

use rayon::prelude::*;

use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;
use tokio::sync::mpsc;

use tokio::net::TcpStream;
use tokio::time::timeout;

use futures::stream;
use futures::StreamExt;

pub const MOST_COMMON_PORTS_100: &[u16] = &[
    80, 23, 443, 21, 22, 25, 3389, 110, 445, 139, 143, 53, 135, 3306, 8080, 1723, 111, 995, 993,
    5900, 1025, 587, 8888, 199, 1720, 465, 548, 113, 81, 6001, 10000, 514, 5060, 179, 1026, 2000,
    8443, 8000, 32768, 554, 26, 1433, 49152, 2001, 515, 8008, 49154, 1027, 5666, 646, 5000, 5631,
    631, 49153, 8081, 2049, 88, 79, 5800, 106, 2121, 1110, 49155, 6000, 513, 990, 5357, 427, 49156,
    543, 544, 5101, 144, 7, 389, 8009, 3128, 444, 9999, 5009, 7070, 5190, 3000, 5432, 1900, 3986,
    13, 1029, 9, 5051, 6646, 49157, 1028, 873, 1755, 2717, 4899, 9100, 119, 37,
];

pub async fn scan_ports(concurrency: usize, mut subdomain: Subdomain) -> Subdomain {
    let mut ret = subdomain.clone();

    let (tx1, mut rx1) = mpsc::channel(concurrency);
    let (tx2, mut rx2) = mpsc::channel(concurrency);

    tokio::spawn(async move {
        for port in MOST_COMMON_PORTS_100 {
            tx1.send(port).await;
        }
    });

    println!("here");

    let input_rx_stream = tokio_stream::wrappers::ReceiverStream::new(rx1);
    input_rx_stream
        .for_each_concurrent(concurrency, |port| {
            let tx2c = tx2.clone();
            let subdomain_copy = subdomain.clone();
            async move {
                let port = scan_port(&subdomain_copy.name, *port).await;
                if port.is_open {
                    tx2c.send(port).await;
                }
            }
        })
        .await;

    println!("here2");
    // close channel
    drop(tx2);

    let output_rx_stream = tokio_stream::wrappers::ReceiverStream::new(rx2);
    ret.port = output_rx_stream.collect().await;

    println!("here3");
    ret

    /*
        let ports: Vec<Port> = MOST_COMMON_PORTS_100
            .par_iter()
            .map(|p| scan_port(&subdomain.name, *p))
            .filter(|p| p.is_open)
            .collect();

        subdomain.port = ports;
        subdomain
    */
}

async fn scan_port(hostname: &str, port: u16) -> Port {
    // socket address
    // connect timeout

    let delay = Duration::from_secs(1);

    let addr = format!("{}:{}", hostname, port);

    println!("socket addr: {:?}", addr);
    let mut addrs_iter = addr.to_socket_addrs();
    //.expect(&format!("to socket addrs: {:?}", addr));

    if addrs_iter.is_err() {
        return Port {
            port: port,
            is_open: false,
        };
    }

    //let mut addrs_iter = SocketAddr::from_str().unwrap();
    //let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let res = timeout(
        Duration::from_millis(3000),
        tokio::net::TcpStream::connect(&addrs_iter.unwrap().next().expect("connect timeout nack")),
    )
    .await;

    if res.is_err() {
        Port {
            port: port,
            is_open: false,
        }
    } else {
        Port {
            port: port,
            is_open: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ports::*;
    #[tokio::test]
    async fn test_scan_port() {
        let res = scan_port("www.google.com", 80).await;
        assert_eq!(res.is_open, true);

        let res = scan_port("www.google.com", 8080).await;
        assert_eq!(res.is_open, false);
    }

    #[tokio::test]
    async fn test_scan_ports() {
        let mut subdomain = Subdomain {
            name: "www.google.com".to_string(),
            port: Vec::new(),
        };
        let res = scan_ports(100, subdomain).await;
        println!("{:?}", res);
    }
}
