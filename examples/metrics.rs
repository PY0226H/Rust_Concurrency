use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;
use rand::rngs::ThreadRng;
use std::thread;
use std::time::Duration;

const N: usize = 2;
const M: usize = 4;
fn main() -> Result<()> {
    let metrics = Metrics::new();

    println!("{:?}", metrics.snapshot());

    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..M {
        request_workder(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(1));
        println!("{:?}", metrics.snapshot());
    }
}

fn task_worker(idx: usize, metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng: ThreadRng = rand::rng();
            thread::sleep(Duration::from_millis(rng.random_range(100..5000)));
            metrics.inc(format!("call.thread.workder.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error>(())
    });
    Ok(())
}

fn request_workder(metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng: ThreadRng = rand::rng();
            thread::sleep(Duration::from_millis(rng.random_range(50..800)));
            let page = rng.random_range(1..5);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error>(())
    });
    Ok(())
}
