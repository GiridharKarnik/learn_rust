use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::time::{interval, sleep, timeout};

// =============================================================================
// 1. Basic async fn and .await
// =============================================================================

/// Simulates fetching a train's status from an API.
/// In reality this would be an HTTP call — here we just sleep.
async fn fetch_train_status(number: u32) -> String {
    sleep(Duration::from_millis(200)).await;
    format!("Train {number}: On Time")
}

// =============================================================================
// 2. Sequential execution — one .await after another
// =============================================================================

async fn demo_sequential() {
    println!("\n═══ Sequential Execution ═══");
    let start = Instant::now();

    let a = fetch_train_status(101).await; // 200ms
    let b = fetch_train_status(202).await; // +200ms
    let c = fetch_train_status(303).await; // +200ms

    let elapsed = start.elapsed();
    println!("  {a}");
    println!("  {b}");
    println!("  {c}");
    println!("  ⏱  Took {elapsed:.0?} (expected ~600ms)");
}

// =============================================================================
// 3. Concurrent execution with tokio::join!
// =============================================================================

async fn demo_concurrent() {
    println!("\n═══ Concurrent Execution (tokio::join!) ═══");
    let start = Instant::now();

    let (a, b, c) = tokio::join!(
        fetch_train_status(101),
        fetch_train_status(202),
        fetch_train_status(303),
    );

    let elapsed = start.elapsed();
    println!("  {a}");
    println!("  {b}");
    println!("  {c}");
    println!("  ⏱  Took {elapsed:.0?} (expected ~200ms — all ran at once!)");
}

// =============================================================================
// 4. Dynamic concurrency with JoinSet
// =============================================================================

async fn demo_join_set() {
    println!("\n═══ Dynamic Concurrency (JoinSet) ═══");
    let trains = vec![401, 402, 403, 404, 405];
    let start = Instant::now();

    let mut set = tokio::task::JoinSet::new();
    for number in trains {
        set.spawn(async move { fetch_train_status(number).await });
    }

    while let Some(result) = set.join_next().await {
        println!("  {}", result.unwrap());
    }

    let elapsed = start.elapsed();
    println!("  ⏱  5 trains fetched in {elapsed:.0?} (expected ~200ms)");
}

// =============================================================================
// 5. tokio::spawn — background tasks
// =============================================================================

async fn demo_spawn() {
    println!("\n═══ tokio::spawn (Background Tasks) ═══");

    let handle = tokio::spawn(async {
        sleep(Duration::from_millis(300)).await;
        "🚂 Express 999 cleared the tunnel"
    });

    println!("  ⏳ Main task continues while Express 999 runs in background...");
    sleep(Duration::from_millis(100)).await;
    println!("  📋 Main task doing other work...");

    let result = handle.await.unwrap();
    println!("  {result}");
}

// =============================================================================
// 6. tokio::time — sleep, timeout, interval
// =============================================================================

/// A slow "API call" that takes 2 seconds.
async fn slow_fetch() -> String {
    sleep(Duration::from_secs(2)).await;
    "Delayed train data".to_string()
}

async fn demo_timeout() {
    println!("\n═══ Timeout ═══");

    // This should succeed (200ms fetch vs 1s deadline)
    let fast = timeout(Duration::from_secs(1), fetch_train_status(42)).await;
    match fast {
        Ok(status) => println!("  ✅ Fast fetch: {status}"),
        Err(_) => println!("  ⏰ Fast fetch timed out"),
    }

    // This should time out (2s fetch vs 500ms deadline)
    let slow = timeout(Duration::from_millis(500), slow_fetch()).await;
    match slow {
        Ok(status) => println!("  ✅ Slow fetch: {status}"),
        Err(_) => println!("  ⏰ Slow fetch timed out (as expected)"),
    }
}

async fn demo_interval() {
    println!("\n═══ Interval (Station Announcements) ═══");
    let mut ticker = interval(Duration::from_millis(300));

    for i in 1..=4 {
        ticker.tick().await;
        println!("  📢 Announcement #{i}: Next train in {}s", 5 - i);
    }
}

// =============================================================================
// 7. Async with Result and ?
// =============================================================================

#[derive(Debug)]
struct DispatchError(String);

impl std::fmt::Display for DispatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DispatchError: {}", self.0)
    }
}

impl std::error::Error for DispatchError {}

async fn lookup_platform(train: u32) -> Result<u8, DispatchError> {
    sleep(Duration::from_millis(100)).await;

    match train {
        100..=199 => Ok(1),
        200..=299 => Ok(2),
        300..=399 => Ok(3),
        _ => Err(DispatchError(format!("No platform for train {train}"))),
    }
}

async fn demo_async_result() {
    println!("\n═══ Async + Result ═══");

    // These succeed
    for number in [150, 250, 350] {
        match lookup_platform(number).await {
            Ok(p) => println!("  🚉 Train {number} → Platform {p}"),
            Err(e) => println!("  ❌ {e}"),
        }
    }

    // This fails
    match lookup_platform(999).await {
        Ok(p) => println!("  🚉 Train 999 → Platform {p}"),
        Err(e) => println!("  ❌ {e}"),
    }
}

// =============================================================================
// 8. Channels — mpsc (multiple producer, single consumer)
// =============================================================================

async fn demo_channels() {
    println!("\n═══ Channels (mpsc) ═══");

    let (tx, mut rx) = mpsc::channel::<String>(32);

    // Spawn a producer that sends departure updates
    let producer = tokio::spawn(async move {
        let departures = [
            (101, "Eurostar", "Platform 1"),
            (202, "TGV", "Platform 2"),
            (303, "ICE", "Platform 3"),
        ];

        for (number, name, platform) in departures {
            let msg = format!("🚂 Train {number} ({name}) departing from {platform}");
            tx.send(msg).await.unwrap();
            sleep(Duration::from_millis(200)).await;
        }
        // tx is dropped → channel closes
    });

    // Consumer: receive until channel closes
    while let Some(update) = rx.recv().await {
        println!("  📡 {update}");
    }

    producer.await.unwrap();
    println!("  ✅ All departures announced");
}

// =============================================================================
// Main — run all demos
// =============================================================================

#[tokio::main]
async fn main() {
    println!("🚂 Lesson 14: Async Execution with Tokio\n");

    demo_sequential().await;
    demo_concurrent().await;
    demo_join_set().await;
    demo_spawn().await;
    demo_timeout().await;
    demo_interval().await;
    demo_async_result().await;
    demo_channels().await;

    println!("\n🏁 All demos complete!");
}
