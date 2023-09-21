use futures::{stream::FusedStream, StreamExt};
use std::{collections::HashMap, time::Duration};
use tokio::{sync::broadcast, sync::mpsc, time};

pub async fn tracker(
    mut timers_rx: mpsc::Receiver<(String, Duration)>,
    timeout_tx: broadcast::Sender<String>,
) {
    let mut registered_timers = HashMap::new();
    let mut timers_futures = futures::stream::FuturesUnordered::new();
    loop {
        tokio::select! {
            Some((name, duration)) = timers_rx.recv() => {
                use std::collections::hash_map::Entry;
                match registered_timers.entry(name.clone()) {
                    Entry::Vacant(entry) => {
                        let (timer, abort) = futures::future::abortable(timer(name, duration));
                        entry.insert(abort);
                        timers_futures.push(timer);
                    },
                    Entry::Occupied(mut entry) => {
                        let (timer, abort) = futures::future::abortable(timer(name, duration));
                        let old = entry.insert(abort);
                        timers_futures.push(timer);
                        old.abort();
                    },
                }
            }
            Some(future_result) = timers_futures.next(), if !timers_futures.is_terminated() => {
                if let Ok(name) = future_result {
                    let _ = timeout_tx.send(name);
                }
            },
            else => break,
        }
    }
}

async fn timer(name: String, duration: Duration) -> String {
    time::sleep(duration).await;
    name
}
