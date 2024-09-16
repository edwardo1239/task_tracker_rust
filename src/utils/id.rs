use std::{sync::atomic::AtomicUsize, time::{SystemTime, UNIX_EPOCH}};


static COUNTER: AtomicUsize = AtomicUsize::new(0);


pub fn create_unique_id() -> String {
    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let timestamp = since_epoch.as_millis();

    let count = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    format!("{}-{}", timestamp, count)
}
