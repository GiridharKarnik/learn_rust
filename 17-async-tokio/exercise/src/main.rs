// =============================================================================
// 🚂 EXERCISE: Railway Dispatch Simulator
// =============================================================================
//
// Build an async railway dispatch system using Tokio. No real network calls —
// use tokio::time::sleep to simulate delays.
//
// Run with: cargo run
// You'll see timing differences between sequential and concurrent execution.
//
// STEP 1: Define a `TrainUpdate` struct
// ---------------------------------------------------------------------
// Fields:
//   - number: u32          (train number, e.g. 101)
//   - name: String          (train name, e.g. "Eurostar")
//   - status: String        (e.g. "On Time", "Delayed 5min")
//   - timestamp: String     (when the update was fetched — use a simple
//                            counter or "now" placeholder; no need for
//                            real chrono timestamps)
//
// Derive Debug and Clone.
//
//
// STEP 2: Implement `async fn fetch_train_status(number: u32, name: &str) -> TrainUpdate`
// ---------------------------------------------------------------------
// - Simulate an API delay with: tokio::time::sleep(Duration::from_millis(500)).await
// - Return a TrainUpdate with:
//     number: the input number
//     name:   the input name (owned String)
//     status: pick any status string (e.g. "On Time")
//     timestamp: format!("t={number}") or any placeholder
// - Print a message like: "📡 Fetched status for Train {number} ({name})"
//
//
// STEP 3: Implement `async fn fetch_all_sequential(trains: &[(u32, &str)]) -> Vec<TrainUpdate>`
// ---------------------------------------------------------------------
// - Loop through `trains` and call fetch_train_status for each, awaiting
//   one at a time.
// - Measure the total time with std::time::Instant::now() / .elapsed()
// - Print the elapsed time at the end.
// - Return the collected results.
//
//
// STEP 4: Implement `async fn fetch_all_concurrent(trains: &[(u32, &str)]) -> Vec<TrainUpdate>`
// ---------------------------------------------------------------------
// - Fetch all trains concurrently. You can use:
//     Option A: tokio::task::JoinSet — spawn each fetch, collect results
//     Option B: tokio::join! — if you know the count at compile time
//   JoinSet is recommended since the number of trains is dynamic.
// - Measure and print elapsed time.
// - Return the results.
// - This should be significantly faster than sequential!
//
//
// STEP 5: Implement `async fn monitor_with_timeout(number: u32, name: &str, timeout_ms: u64) -> Result<TrainUpdate, String>`
// ---------------------------------------------------------------------
// - Use tokio::time::timeout to wrap fetch_train_status with a deadline.
// - If the fetch completes in time, return Ok(update).
// - If it times out, return Err("⏰ Timeout fetching Train {number} ({name})".into())
// - Hint: timeout returns Result<T, Elapsed>. Map the Err.
//
//
// STEP 6: Implement `async fn dispatch_updates(count: u32)`
// ---------------------------------------------------------------------
// - Create an mpsc channel: let (tx, mut rx) = tokio::sync::mpsc::channel(32);
// - Spawn a producer task that:
//     - Sends `count` TrainUpdate messages through the channel
//     - Use train numbers 1..=count and made-up names
//     - Sleep 200ms between each send to simulate staggered arrivals
// - In the main flow (not spawned), receive messages from rx and print them.
// - The loop ends when the sender is dropped and the channel closes.
//
//
// STEP 7: Write the async main function
// ---------------------------------------------------------------------
// - Use #[tokio::main] on main.
// - Define a list of trains: [(u32, &str)] — at least 4 trains with numbers and names.
// - Call fetch_all_sequential and print results.
// - Call fetch_all_concurrent and print results.
// - Call monitor_with_timeout with a generous timeout (should succeed).
// - Call monitor_with_timeout with a tiny timeout like 100ms (should fail since
//   fetch takes 500ms).
// - Call dispatch_updates(5).
// - Print a summary at the end.
//
// =============================================================================

fn main() {
    // TODO: Replace with #[tokio::main] async fn main() and implement the steps above.
    println!("Replace me with your async implementation!");
}
