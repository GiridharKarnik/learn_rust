// =============================================================================
// 🚂 SOLUTION: Railway Dispatch Simulator
// =============================================================================

use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::task::JoinSet;
use tokio::time::{sleep, timeout};

// STEP 1: Define TrainUpdate struct
#[derive(Debug, Clone)]
struct TrainUpdate {
    number: u32,
    name: String,
    status: String,
    timestamp: String,
}

impl std::fmt::Display for TrainUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Train {} ({}) — {} [{}]",
            self.number, self.name, self.status, self.timestamp
        )
    }
}

// STEP 2: Simulate fetching a train's status
async fn fetch_train_status(number: u32, name: &str) -> TrainUpdate {
    // Simulate an API call that takes 500ms
    sleep(Duration::from_millis(500)).await;

    let statuses = ["On Time", "Delayed 5min", "Boarding", "Departed"];
    let status = statuses[(number as usize) % statuses.len()];

    println!("  📡 Fetched status for Train {number} ({name})");

    TrainUpdate {
        number,
        name: name.to_string(),
        status: status.to_string(),
        timestamp: format!("t={number}"),
    }
}

// STEP 3: Fetch all sequentially — one at a time
async fn fetch_all_sequential(trains: &[(u32, &str)]) -> Vec<TrainUpdate> {
    let start = Instant::now();
    let mut results = Vec::new();

    for &(number, name) in trains {
        let update = fetch_train_status(number, name).await;
        results.push(update);
    }

    let elapsed = start.elapsed();
    println!("  ⏱  Sequential: {elapsed:.0?} for {} trains", trains.len());
    results
}

// STEP 4: Fetch all concurrently — all at once
async fn fetch_all_concurrent(trains: &[(u32, &str)]) -> Vec<TrainUpdate> {
    let start = Instant::now();

    let mut set = JoinSet::new();
    for &(number, name) in trains {
        let name = name.to_string(); // own the data for the spawned task
        set.spawn(async move { fetch_train_status(number, &name).await });
    }

    let mut results = Vec::new();
    while let Some(res) = set.join_next().await {
        results.push(res.unwrap());
    }

    let elapsed = start.elapsed();
    println!("  ⏱  Concurrent: {elapsed:.0?} for {} trains", trains.len());
    results
}

// STEP 5: Fetch with a timeout deadline
async fn monitor_with_timeout(
    number: u32,
    name: &str,
    timeout_ms: u64,
) -> Result<TrainUpdate, String> {
    let result = timeout(
        Duration::from_millis(timeout_ms),
        fetch_train_status(number, name),
    )
    .await;

    result.map_err(|_| format!("⏰ Timeout fetching Train {number} ({name})"))
}

// STEP 6: Channel-based dispatch system
async fn dispatch_updates(count: u32) {
    let (tx, mut rx) = mpsc::channel::<TrainUpdate>(32);

    // Spawn the producer
    tokio::spawn(async move {
        let names = ["Eurostar", "TGV", "ICE", "Shinkansen", "Pendolino"];
        for i in 1..=count {
            let name = names[((i - 1) as usize) % names.len()];
            let update = TrainUpdate {
                number: 900 + i,
                name: name.to_string(),
                status: "Dispatched".to_string(),
                timestamp: format!("msg-{i}"),
            };
            tx.send(update).await.unwrap();
            sleep(Duration::from_millis(200)).await;
        }
        // tx is dropped here → channel closes
    });

    // Receive updates until the channel closes
    while let Some(update) = rx.recv().await {
        println!("  📨 {update}");
    }
}

// STEP 7: Async main
#[tokio::main]
async fn main() {
    println!("🚂 Railway Dispatch Simulator\n");

    // Our train fleet
    let trains: Vec<(u32, &str)> = vec![
        (101, "Eurostar"),
        (202, "TGV"),
        (303, "ICE"),
        (404, "Shinkansen"),
    ];

    // --- Sequential ---
    println!("═══ Sequential Fetching ═══");
    let sequential_results = fetch_all_sequential(&trains).await;
    for update in &sequential_results {
        println!("  🚂 {update}");
    }

    // --- Concurrent ---
    println!("\n═══ Concurrent Fetching ═══");
    let concurrent_results = fetch_all_concurrent(&trains).await;
    for update in &concurrent_results {
        println!("  🚂 {update}");
    }

    // --- Timeout: success ---
    println!("\n═══ Timeout Tests ═══");
    match monitor_with_timeout(101, "Eurostar", 2000).await {
        Ok(update) => println!("  ✅ {update}"),
        Err(e) => println!("  ❌ {e}"),
    }

    // --- Timeout: failure (100ms is less than the 500ms fetch time) ---
    match monitor_with_timeout(202, "TGV", 100).await {
        Ok(update) => println!("  ✅ {update}"),
        Err(e) => println!("  ❌ {e}"),
    }

    // --- Channel-based dispatch ---
    println!("\n═══ Channel Dispatch ═══");
    dispatch_updates(5).await;

    println!("\n🏁 Simulation complete!");
}
