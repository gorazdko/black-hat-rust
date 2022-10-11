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

use tokio::sync::mpsc;
use tokio::sync::oneshot;

use tokio::sync::broadcast;

async fn some_computation() -> String {
    "represents the result of the computation".to_string()
}

use std::sync::Arc;
use tokio::sync::Mutex;

use futures::{stream, StreamExt};
use rand::{thread_rng, Rng};

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    stream::iter(0..200u64)
        .for_each_concurrent(20, |number| async move {
            let mut rng = thread_rng();
            let sleep_ms: u64 = rng.gen_range(0..20);
            tokio::time::sleep(Duration::from_millis(sleep_ms)).await;
            println!("{}", number);
        })
        .await;

    use futures::channel::oneshot;
    use futures::stream::{self, StreamExt};

    let (tx1, rx1) = oneshot::channel::<String>();
    let (tx2, rx2) = oneshot::channel::<String>();
    let (tx3, rx3) = oneshot::channel::<String>();

    let fut = stream::iter(vec![rx1, rx2, rx3]).for_each_concurrent(
        /* limit */ 3,
        |rx| async move {
            let res = rx.await.unwrap();
            println!("res: {:?}", res);
        },
    );
    tx1.send(
        "en222222222222222222222222222222222222222222222222222gggggggggggggg222222222222222"
            .to_string(),
    )
    .unwrap();
    tx2.send(
        "endddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd2".to_string(),
    )
    .unwrap();
    tx3.send("3".to_string()).unwrap();
    fut.await;
}

/*
//#[tokio::main]
async fn main2() {











    let port: Result<String, std::env::VarError> =
        std::env::var("PORT").or(Ok(String::from("8080")));

    println!("port: {:?}", port.unwrap());

    let mut var: u32 = 0;
    let data1 = Arc::new(Mutex::new(var));
    let data2 = Arc::clone(&data1);
    let data3 = Arc::clone(&data1);

    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        let res = some_computation().await;
        tx.send(res).unwrap();
    });
    // Do other work while the computation is happening in the background
    println!("Doing other work");

    let (tx2, mut rx2) = mpsc::channel(100);

    let join_handle1 = tokio::spawn(async move {
        let mut lock1 = data1.lock().await;
        for i in 1..10 {
            *lock1 += 1;
            let res = some_computation().await;
            tx2.send(res).await;
        }
    });

    let join_handle = tokio::spawn(async move {
        for i in 1..10 {
            let res = rx2.recv().await.unwrap();
            println!("pp::res2: {:?}", res);
        }
    });

    let join_handle2 = tokio::spawn(async move {
        // Wait for the computation result
        let mut lock1 = data2.lock().await;
        tokio::time::sleep(Duration::from_millis(2000)).await;
        let res = rx.await.unwrap();
        *lock1 += 1;

        println!("++hello world_: {:?}", res);
    });

    //tokio::time::sleep(Duration::from_millis(3000)).await;

    let res = join_handle.await.unwrap();
    let res = join_handle2.await.unwrap();
    let res = join_handle1.await.unwrap();

    println!("data2: {:?}", data3);
}
*/
