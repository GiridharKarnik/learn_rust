// =============================================
// EXERCISE 15: Railway Ticket Counter
// =============================================
//
// Simulate multiple ticket counters (async tasks) selling from a shared pool.
// There is NO code provided — you write everything, including main().
//
// This exercises: Arc, Mutex, tokio::spawn, shared mutable state,
// async/await, lock patterns.
//
// Run with: cargo run
//
// Expected output (order of counter messages may vary):
//
//   === RAILWAY TICKET COUNTER ===
//
//   Starting simulation with 50 tickets available...
//
//   [Counter 1] Sold ticket to Amit for train 12001 (49 remaining)
//   [Counter 1] Sold ticket to Priya for train 12007 (48 remaining)
//   [Counter 2] Sold ticket to Raj for train 12001 (47 remaining)
//   [Counter 2] Sold ticket to Sita for train 12245 (46 remaining)
//   [Counter 2] Sold ticket to Vikram for train 12001 (45 remaining)
//   [Counter 3] Sold ticket to Anita for train 12007 (44 remaining)
//   [Counter 3] Sold ticket to Deepak for train 12245 (43 remaining)
//   [Counter 1] Sold ticket to Meera for train 12245 (42 remaining)
//
//   === SIMULATION COMPLETE ===
//   Tickets remaining: 42
//   Tickets sold: 8
//   Sales log:
//     Amit → Train 12001
//     Priya → Train 12007
//     Raj → Train 12001
//     Sita → Train 12245
//     Vikram → Train 12001
//     Anita → Train 12007
//     Deepak → Train 12245
//     Meera → Train 12245

// =============================================
// STEP 1: Define the `TicketPool` struct
// =============================================
// Fields:
//   available: u32            — number of tickets remaining
//   sold: Vec<(String, u32)>  — list of (passenger_name, train_number)
//
// Derive Debug.

// =============================================
// STEP 2: Implement `TicketPool` methods
// =============================================
//
// impl TicketPool {
//
//   fn new(available: u32) -> Self
//     Create a new pool with the given number of tickets and an empty sold vec.
//
//   fn sell(&mut self, passenger: &str, train_number: u32) -> Result<u32, String>
//     If available > 0:
//       - Decrement available
//       - Push (passenger.to_string(), train_number) to sold
//       - Return Ok(self.available)  (remaining count)
//     Else:
//       - Return Err("No tickets available".to_string())
//
//   fn available_count(&self) -> u32
//     Return self.available
//
//   fn sold_count(&self) -> usize
//     Return self.sold.len()
//
//   fn sold_log(&self) -> &[(String, u32)]
//     Return a slice of the sold vec.
// }

// =============================================
// STEP 3: Wrap in Arc<Mutex<TicketPool>>
// =============================================
//
// This step happens inside run_simulation (Step 5).
// You'll create: Arc::new(Mutex::new(TicketPool::new(50)))
//
// Use std::sync::{Arc, Mutex} — we hold the lock briefly
// (no .await while locked), so std::sync::Mutex is fine with Tokio.

// =============================================
// STEP 4: Implement the counter task
// =============================================
//
// async fn ticket_counter(
//     id: u32,
//     pool: Arc<Mutex<TicketPool>>,
//     passengers: Vec<(&str, u32)>,
// )
//
// For each (passenger, train_number) in passengers:
//   1. Lock the pool
//   2. Call pool.sell(passenger, train_number)
//   3. Match the result:
//      Ok(remaining)  → println!("[Counter {}] Sold ticket to {} for train {} ({} remaining)", ...)
//      Err(e)         → println!("[Counter {}] Failed for {}: {}", id, passenger, e)
//   4. The lock is released when the guard goes out of scope
//   5. Add a small delay: tokio::time::sleep(Duration::from_millis(10)).await
//      (This simulates real work and lets other tasks interleave)

// =============================================
// STEP 5: Implement run_simulation
// =============================================
//
// async fn run_simulation()
//
// 1. Create the shared pool: Arc::new(Mutex::new(TicketPool::new(50)))
//
// 2. Print "Starting simulation with 50 tickets available..."
//
// 3. Define 3 sets of passengers:
//    Counter 1: vec![("Amit", 12001), ("Priya", 12007), ("Meera", 12245)]
//    Counter 2: vec![("Raj", 12001), ("Sita", 12245), ("Vikram", 12001)]
//    Counter 3: vec![("Anita", 12007), ("Deepak", 12245)]
//
// 4. Spawn 3 tokio tasks, one per counter:
//      let pool_clone = Arc::clone(&pool);
//      tokio::spawn(async move { ticket_counter(id, pool_clone, passengers).await })
//
// 5. Await all 3 task handles (join them).
//
// 6. Print final stats:
//    "\n=== SIMULATION COMPLETE ==="
//    "Tickets remaining: {}"
//    "Tickets sold: {}"
//    "Sales log:"
//    For each (name, train) in sold_log:
//      "  {} → Train {}"

// =============================================
// STEP 6: Write main()
// =============================================
//
// #[tokio::main]
// async fn main() {
//     println!("=== RAILWAY TICKET COUNTER ===\n");
//     run_simulation().await;
// }

#[tokio::main]
async fn main() {
    // Your code here
}
