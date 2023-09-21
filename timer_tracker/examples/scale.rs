use futures::stream::poll_immediate;
use rand::{distributions::Alphanumeric, Rng};
use std::time::Duration;
use timer_tracker::tracker;
use tokio::{
    sync::{broadcast, mpsc},
    time::{self},
};

#[tokio::main]
async fn main() {
    // timer sender and receivers
    let (timer_add_tx, timer_add_rx) = mpsc::channel::<(String, Duration)>(1024);
    let (timeout_tx, mut timeout_rx) = broadcast::channel(1024);

    let (_, _, _) = tokio::join!(
        tracker(timer_add_rx, timeout_tx),
        async move {
            for i in 0..1000000 {
                let name = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(32)
                    .map(char::from)
                    .collect::<String>();

                let duration = Duration::from_millis(rand::thread_rng().gen_range(20_000..30_000));

                timer_add_tx.send((name, duration)).await.unwrap();

                // Batch sends per 1000 to avoid channel lag.
                if i % 1000 == 0 {
                    time::sleep(Duration::from_millis(1)).await;
                }
            }

            println!("Creation done!")
        },
        async move {
            let mut timer_amount = 0;

            loop {
                let timeout_result = timeout_rx.recv().await;
                match timeout_result {
                    Ok(name) => {
                        timer_amount += 1;
                        //println!("Timer {} timed out: {}", name, timer_amount);
                    }
                    Err(_) => break,
                }
            }

            println!("All timed out! {}", timer_amount)
        }
    );
}
