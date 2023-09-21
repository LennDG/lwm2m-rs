use std::time::Duration;
use timer_tracker::tracker;
use tokio::{
    sync::{broadcast, mpsc},
    time::{self},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // timer sender and receivers
    let (timer_add_tx, timer_add_rx) = mpsc::channel(10);
    let (timeout_tx, mut timeout_rx) = broadcast::channel(1024);
    let (_, res1, _) = tokio::join!(
        tracker(timer_add_rx, timeout_tx),
        async move {
            timer_add_tx
                .send(("Foo".to_string(), Duration::from_secs(1)))
                .await?;
            timer_add_tx
                .send(("Bar".to_string(), Duration::from_secs(2)))
                .await?;
            timer_add_tx
                .send(("Baz".to_string(), Duration::from_secs(10)))
                .await?;
            timer_add_tx
                .send(("Bonk".to_string(), Duration::from_secs(3)))
                .await?;

            time::sleep(Duration::from_secs(2)).await;
            timer_add_tx
                .send(("Bar".to_string(), Duration::from_secs(10)))
                .await?;

            timer_add_tx
                .send(("Bonk".to_string(), Duration::from_secs(8)))
                .await?;

            Ok(())
        },
        async move {
            loop {
                let timeout_result = timeout_rx.recv().await;
                match timeout_result {
                    Ok(name) => println!("{} timer timed out", name),
                    Err(_) => break,
                }
            }
        }
    );
    res1
}
