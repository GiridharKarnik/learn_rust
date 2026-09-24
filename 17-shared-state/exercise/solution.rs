// Lesson 15: Shared State — Railway Ticket Counter (SOLUTION)

use std::sync::{Arc, Mutex};
use tokio::time::Duration;

// ---------------------------------------------------------------------------
// STEP 1: Define TicketPool struct
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct TicketPool {
    available: u32,
    sold: Vec<(String, u32)>,
}

// ---------------------------------------------------------------------------
// STEP 2: Implement TicketPool methods
// ---------------------------------------------------------------------------

impl TicketPool {
    fn new(available: u32) -> Self {
        TicketPool {
            available,
            sold: vec![],
        }
    }

    fn sell(&mut self, passenger: &str, train_number: u32) -> Result<u32, String> {
        if self.available > 0 {
            self.available -= 1;
            self.sold.push((passenger.to_string(), train_number));
            Ok(self.available)
        } else {
            Err("No tickets available".to_string())
        }
    }

    fn available_count(&self) -> u32 {
        self.available
    }

    fn sold_count(&self) -> usize {
        self.sold.len()
    }

    fn sold_log(&self) -> &[(String, u32)] {
        &self.sold
    }
}

// ---------------------------------------------------------------------------
// STEP 4: Implement ticket_counter
// ---------------------------------------------------------------------------

async fn ticket_counter(
    id: u32,
    pool: Arc<Mutex<TicketPool>>,
    passengers: Vec<(&str, u32)>,
) {
    for (passenger, train_number) in passengers {
        {
            let mut guard = pool.lock().unwrap();
            match guard.sell(passenger, train_number) {
                Ok(remaining) => {
                    println!(
                        "[Counter {}] Sold ticket to {} for train {} ({} remaining)",
                        id, passenger, train_number, remaining
                    );
                }
                Err(e) => {
                    println!("[Counter {}] Failed for {}: {}", id, passenger, e);
                }
            }
        } // lock released here
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

// ---------------------------------------------------------------------------
// STEP 5: Implement run_simulation
// ---------------------------------------------------------------------------

async fn run_simulation() {
    // STEP 3: Wrap in Arc<Mutex<TicketPool>>
    let pool = Arc::new(Mutex::new(TicketPool::new(50)));

    println!("Starting simulation with 50 tickets available...\n");

    // Define passenger lists for each counter
    let passengers1: Vec<(&str, u32)> = vec![("Amit", 12001), ("Priya", 12007), ("Meera", 12245)];
    let passengers2: Vec<(&str, u32)> = vec![("Raj", 12001), ("Sita", 12245), ("Vikram", 12001)];
    let passengers3: Vec<(&str, u32)> = vec![("Anita", 12007), ("Deepak", 12245)];

    // Spawn 3 counter tasks
    let pool1 = Arc::clone(&pool);
    let handle1 = tokio::spawn(async move {
        ticket_counter(1, pool1, passengers1).await;
    });

    let pool2 = Arc::clone(&pool);
    let handle2 = tokio::spawn(async move {
        ticket_counter(2, pool2, passengers2).await;
    });

    let pool3 = Arc::clone(&pool);
    let handle3 = tokio::spawn(async move {
        ticket_counter(3, pool3, passengers3).await;
    });

    // Await all tasks
    handle1.await.unwrap();
    handle2.await.unwrap();
    handle3.await.unwrap();

    // Print final stats
    let guard = pool.lock().unwrap();
    println!("\n=== SIMULATION COMPLETE ===");
    println!("Tickets remaining: {}", guard.available_count());
    println!("Tickets sold: {}", guard.sold_count());
    println!("Sales log:");
    for (name, train) in guard.sold_log() {
        println!("  {} → Train {}", name, train);
    }
}

// ---------------------------------------------------------------------------
// STEP 6: main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    println!("=== RAILWAY TICKET COUNTER ===\n");
    run_simulation().await;
}
