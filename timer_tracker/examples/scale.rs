use rand::{distributions::Alphanumeric, Rng};
use std::time::Duration;
use timer_tracker::TimerTracker;
use tokio::time::{self};

#[tokio::main]
async fn main() {
    // timer sender and receivers
    let tracker: TimerTracker = Default::default();

    let timer_add_tx = tracker.register();
    let mut timeout_rx = tracker.subscribe();

    let (_, _) = tokio::join!(
        async move {
            for i in 0..1000000 {
                let name = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(32)
                    .map(char::from)
                    .collect::<String>();

                let duration = Duration::from_millis(rand::thread_rng().gen_range(1_000..30_000));

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
                        println!("Timer {} timed out: {}", name, timer_amount);
                    }
                    Err(err) => {
                        use tokio::sync::broadcast::error::RecvError;
                        match err {
                            RecvError::Closed => break,
                            RecvError::Lagged(_) => continue,
                        }
                    }
                }
            }
        }
    );
}
