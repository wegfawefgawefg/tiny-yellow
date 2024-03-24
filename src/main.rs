use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use tokio::task;
use tokio::time::{sleep, Duration};

pub mod common;
pub mod lol;
pub mod name_generator;
pub mod pardemo;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

lazy_static::lazy_static! {
    static ref NAMES: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
}

async fn inc_counter() {
    loop {
        sleep(Duration::from_secs(1)).await;
        let val = COUNTER.fetch_add(1, Ordering::SeqCst);
    }
}

async fn gen_name() {
    loop {
        sleep(Duration::from_secs(1)).await;
        let name = name_generator::gen_name_of_rand_length();
        NAMES.lock().unwrap().push(name);
    }
}

async fn print_counter() {
    loop {
        sleep(Duration::from_secs(2)).await;
        let val = COUNTER.load(Ordering::SeqCst);
        println!("Counter is at {}", val);
    }
}

async fn print_names() {
    loop {
        sleep(Duration::from_secs(3)).await;
        let names = NAMES.lock().unwrap();
        println!("Names: {:?}", *names);
    }
}

#[tokio::main]
async fn main() {
    tokio::join!(
        inc_counter(),
        inc_counter(),
        print_counter(),
        gen_name(),
        print_names(),
        gen_name(),
    );
}
