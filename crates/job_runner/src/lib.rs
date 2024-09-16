use chrono::Local;
use tokio::task;
use tokio::task::JoinHandle;
use tokio::time::{Duration, Instant};

// Refactor to accept a generic output type `R` from the job function
pub fn run_job<F, Fut, R>(name: &'static str, interval_duration: Duration, job_fn: F) -> JoinHandle<()>
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = R> + Send + 'static,
    R: std::fmt::Debug + Send + 'static, // The result type must implement Debug for printing
{
    task::spawn(async move {
        loop {
            let now = Instant::now();

            println!(
                "{}: start: {}, interval: {} seconds",
                Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
                name,
                interval_duration.as_secs()
            );

            // Await the job function and capture the result
            let result = job_fn().await;

            // Print the successful result
            println!(
                "{}: done in {} seconds: {}. Result: {:?}",
                Local::now().format("%Y-%m-%d %H:%M:%S.%3f"),
                now.elapsed().as_secs(),
                name,
                result
            );

            tokio::time::sleep(interval_duration).await;
        }
    })
}
