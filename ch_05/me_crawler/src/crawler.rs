use crate::spiders::Spider;
use futures::stream::StreamExt;
use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{
    sync::{mpsc, Barrier},
    time::sleep,
};

pub struct Crawler {
    delay: Duration,
    crawling_concurrency: usize,
    processing_concurrency: usize,
}

impl Crawler {
    pub fn new(
        delay: Duration,
        crawling_concurrency: usize,
        processing_concurrency: usize,
    ) -> Self {
        Crawler {
            delay,
            crawling_concurrency,
            processing_concurrency,
        }
    }

    pub async fn run<T: Send + 'static>(&self, spider: Arc<dyn Spider<Item = T>>) {
        let mut visited_urls = HashSet::<String>::new();
        let crawling_concurrency = self.crawling_concurrency;
        let crawling_queue_capacity = crawling_concurrency * 400;
        let processing_concurrency = self.processing_concurrency;
        let processing_queue_capacity = processing_concurrency * 10;
        let active_spiders = Arc::new(AtomicUsize::new(0));

        let (urls_to_visit_tx, urls_to_visit_rx) = mpsc::channel(crawling_queue_capacity);
        let (items_tx, items_rx) = mpsc::channel(processing_queue_capacity);
        let (new_urls_tx, mut new_urls_rx) = mpsc::channel(crawling_queue_capacity);
        let barrier = Arc::new(Barrier::new(3));

        for url in spider.start_urls() {
            visited_urls.insert(url.clone());
            let _ = urls_to_visit_tx.send(url).await;
        }

        self.launch_processors(
            processing_concurrency,
            spider.clone(),
            items_rx,
            barrier.clone(),
        );

        self.launch_scrapers(
            crawling_concurrency,
            spider.clone(),
            urls_to_visit_rx,
            new_urls_tx.clone(),
            items_tx,
            active_spiders.clone(),
            self.delay,
            barrier.clone(),
        );

        loop {
            if let Some((visited_url, new_urls)) = new_urls_rx.try_recv().ok() {
                visited_urls.insert(visited_url);

                for url in new_urls {
                    if !visited_urls.contains(&url) {
                        visited_urls.insert(url.clone());
                        log::debug!("queueing: {}", url);
                        let _ = urls_to_visit_tx.send(url).await;
                    }
                }
            }

            if new_urls_tx.capacity() == crawling_queue_capacity // new_urls channel is empty
            && urls_to_visit_tx.capacity() == crawling_queue_capacity // urls_to_visit channel is empty
            && active_spiders.load(Ordering::SeqCst) == 0
            {
                // no more work, we leave
                break;
            }

            sleep(Duration::from_millis(5)).await;
        }

        log::info!("crawler: control loop exited");

        // we drop the transmitter in order to close the stream
        drop(urls_to_visit_tx);

        // and then we wait for the streams to complete
        barrier.wait().await;
    }

    pub fn launch_processors<T: Send + 'static>(
        &self,
        processing_concurrency: usize,
        spider: Arc<dyn Spider<Item = T>>,
        items_rx: mpsc::Receiver<T>,
        barrier: Arc<Barrier>,
    ) {
        tokio::spawn(async move {
            let barrier = barrier.clone();
            let res = tokio_stream::wrappers::ReceiverStream::new(items_rx)
                .for_each_concurrent(processing_concurrency, |item| {
                    let spider = spider.clone();
                    async move {
                        let _ = spider.process(item).await;
                    }
                })
                .await;
            barrier.wait().await;
        });
    }

    pub fn launch_scrapers<T: Send + 'static>(
        &self,
        crawling_concurrency: usize,
        spider: Arc<dyn Spider<Item = T>>,
        urls_to_visit_rx: mpsc::Receiver<String>,
        new_urls_tx: mpsc::Sender<(String, Vec<String>)>,
        items_tx: mpsc::Sender<T>,
        active_spiders: Arc<AtomicUsize>,
        delay: Duration,
        barrier: Arc<Barrier>,
    ) {
        tokio::spawn(async move {
            let barrier = barrier.clone();

            let res = tokio_stream::wrappers::ReceiverStream::new(urls_to_visit_rx)
                .for_each_concurrent(crawling_concurrency, |url| async {
                    active_spiders.fetch_add(1, Ordering::SeqCst);
                    let res = spider.scrape(url.clone()).await;

                    let mut new_urls = Vec::new();

                    match res {
                        Ok(items) => {
                            for item in items.0 {
                                items_tx.send(item).await;
                            }

                            new_urls = items.1;
                        }
                        Err(er) => {
                            log::debug!("Error: {}", er);
                        }
                    }
                    new_urls_tx.send((url, new_urls)).await;
                    active_spiders.fetch_sub(1, Ordering::SeqCst);
                })
                .await;

            barrier.wait().await;
        });
    }
}
