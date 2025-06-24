use anyhow::{Result, anyhow};
use std::{sync::mpsc, thread};

const NUM_PRODUCERS: usize = 4;
#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: u32,
}
fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    //create producers
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }

    //create consumer
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer received: {:?}", msg);
        }
    });

    consumer
        .join()
        .map_err(|e| anyhow!("consumer error: {:?}", e))?;

    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<u32>();
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(std::time::Duration::from_millis(sleep_time));
    }
}

impl Msg {
    fn new(idx: usize, value: u32) -> Self {
        Self { idx, value }
    }
}
