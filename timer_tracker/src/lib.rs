use futures::{stream::FusedStream, StreamExt};
use std::{collections::HashMap, time::Duration};
use tokio::{sync::broadcast, sync::mpsc, time};

pub struct TimerTracker {
    timers_tx: mpsc::Sender<(String, Duration)>,
    //timers_rx: mpsc::Receiver<(String, Duration)>,
    timeout_tx: broadcast::Sender<String>,
}

impl TimerTracker {
    pub fn new() -> Self {
        let capacity = 1024;

        let (timers_tx, timers_rx) = mpsc::channel(2048);
        let (timeout_tx, _) = broadcast::channel(capacity);

        let thread_timeout_tx = timeout_tx.clone();
        tokio::spawn(async move {
            tracker(timers_rx, thread_timeout_tx, capacity).await;
        });

        TimerTracker {
            timers_tx,
            //timers_rx,
            timeout_tx,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.timeout_tx.subscribe()
    }

    pub fn register(&self) -> mpsc::Sender<(String, Duration)> {
        self.timers_tx.clone()
    }
}

impl Default for TimerTracker {
    fn default() -> Self {
        TimerTracker::new()
    }
}

async fn tracker(
    mut timers_rx: mpsc::Receiver<(String, Duration)>,
    timeout_tx: broadcast::Sender<String>,
    capacity: usize,
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
                    //Only send a message when the channel is not full
                    while timeout_tx.len() >= capacity {
                        time::sleep(Duration::from_millis(1)).await;
                    };
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
