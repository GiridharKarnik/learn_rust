use std::sync::{Arc, Mutex, RwLock};
use std::thread;

// =========================================================================
// 1. 🚂 Arc — Shared Ownership Across Threads
// =========================================================================

fn arc_basics() {
    println!("=== 1. 🚂 Arc — Shared Ownership ===\n");

    let train_name = Arc::new(String::from("Rajdhani Express"));

    // Clone is cheap — just increments reference count
    let name1 = Arc::clone(&train_name);
    let name2 = Arc::clone(&train_name);

    // All three point to the SAME string on the heap
    println!("Original: {}", train_name);
    println!("Clone 1:  {}", name1);
    println!("Clone 2:  {}", name2);

    // Ref count
    println!("Reference count: {}", Arc::strong_count(&train_name));

    // Sharing across threads
    let data = Arc::new(vec!["Chennai", "Delhi", "Mumbai"]);
    let mut handles = vec![];

    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("  Thread {} sees station: {}", i, data_clone[i]);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
}

// =========================================================================
// 2. 🔒 Mutex — Mutual Exclusion
// =========================================================================

fn mutex_basics() {
    println!("\n=== 2. 🔒 Mutex — Mutual Exclusion ===\n");

    let counter = Mutex::new(0u32);

    // Acquire the lock, mutate, auto-release on drop
    {
        let mut guard = counter.lock().unwrap();
        *guard += 1;
        println!("Inside lock: counter = {}", *guard);
    } // guard dropped → lock released

    // Lock again
    {
        let mut guard = counter.lock().unwrap();
        *guard += 1;
        println!("After second lock: counter = {}", *guard);
    }

    // Short-lived lock pattern — extract value and release
    let value = {
        let guard = counter.lock().unwrap();
        *guard // copy the value
    }; // lock released immediately
    println!("Extracted value (lock released): {}", value);
}

// =========================================================================
// 3. 🚉 Arc<Mutex<T>> — The Standard Combo
// =========================================================================

fn arc_mutex_combo() {
    println!("\n=== 3. 🚉 Arc<Mutex<T>> — Shared Mutable State ===\n");

    // Simulate 4 platforms processing departures
    let departure_count = Arc::new(Mutex::new(0u32));
    let mut handles = vec![];

    for platform in 1..=4 {
        let count = Arc::clone(&departure_count);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut guard = count.lock().unwrap();
                *guard += 1;
            }
            println!("  Platform {} processed 10 departures", platform);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    let total = *departure_count.lock().unwrap();
    println!("Total departures across all platforms: {}", total);
    assert_eq!(total, 40, "Should always be exactly 40!");

    // More complex shared state: a Vec
    println!("\nShared departure log:");
    let log: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];

    let trains = vec![
        ("Rajdhani Express", 3),
        ("Shatabdi Express", 5),
        ("Duronto Express", 1),
        ("Tejas Express", 7),
    ];

    for (name, platform) in trains {
        let log_clone = Arc::clone(&log);
        let handle = thread::spawn(move || {
            let entry = format!("{} departed from Platform {}", name, platform);
            let mut guard = log_clone.lock().unwrap();
            guard.push(entry);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    let guard = log.lock().unwrap();
    for entry in guard.iter() {
        println!("  {}", entry);
    }
}

// =========================================================================
// 4. 📖 RwLock — Multiple Readers, One Writer
// =========================================================================

fn rwlock_demo() {
    println!("\n=== 4. 📖 RwLock — Multiple Readers OR One Writer ===\n");

    let schedule = Arc::new(RwLock::new(vec![
        "06:00 — Rajdhani Express".to_string(),
        "07:30 — Shatabdi Express".to_string(),
    ]));

    // Spawn multiple reader threads
    let mut handles = vec![];

    for i in 1..=3 {
        let schedule_clone = Arc::clone(&schedule);
        let handle = thread::spawn(move || {
            let reader = schedule_clone.read().unwrap();
            println!("  Reader {} sees {} trains:", i, reader.len());
            for train in reader.iter() {
                println!("    {}", train);
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    // Writer — gets exclusive access
    {
        let mut writer = schedule.write().unwrap();
        writer.push("09:00 — Duronto Express".to_string());
        println!("\n  Writer added a new train");
    }

    // Readers see the updated data
    let reader = schedule.read().unwrap();
    println!("\n  After write, schedule has {} trains:", reader.len());
    for train in reader.iter() {
        println!("    {}", train);
    }
}

// =========================================================================
// 5. ⚡ With Tokio — Async Shared State
// =========================================================================

async fn tokio_shared_state() {
    println!("\n=== 5. ⚡ Tokio — Async Shared State ===\n");

    // Using std::sync::Mutex with tokio (fine for quick operations)
    println!("--- std::sync::Mutex with Tokio ---");
    let counter = Arc::new(Mutex::new(0u32));
    let mut tasks = vec![];

    for station_id in 1..=5 {
        let counter_clone = Arc::clone(&counter);
        let task = tokio::spawn(async move {
            // Lock briefly, no .await while holding
            let mut guard = counter_clone.lock().unwrap();
            *guard += 1;
            println!("  Station {} checked in (count: {})", station_id, *guard);
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }
    println!("Final count: {}", *counter.lock().unwrap());

    // Using tokio::sync::Mutex (needed when holding across .await)
    println!("\n--- tokio::sync::Mutex (async-aware) ---");
    let board = Arc::new(tokio::sync::Mutex::new(Vec::<String>::new()));
    let mut tasks = vec![];

    let announcements = vec![
        "Rajdhani Express arriving Platform 3",
        "Shatabdi Express departing Platform 5",
        "Duronto Express delayed by 15 minutes",
    ];

    for announcement in announcements {
        let board_clone = Arc::clone(&board);
        let task = tokio::spawn(async move {
            let mut guard = board_clone.lock().await; // async lock
            // Simulate async work while holding the lock
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            guard.push(announcement.to_string());
            println!("  Posted: {}", announcement);
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }

    let guard = board.lock().await;
    println!("\nDeparture board ({} announcements):", guard.len());
    for msg in guard.iter() {
        println!("  📢 {}", msg);
    }
}

// =========================================================================
// 6. 🏗️ Real-World Pattern — Shared App State
// =========================================================================

#[derive(Debug)]
struct DepartureBoard {
    station: String,
    departures: Vec<Departure>,
}

#[derive(Debug, Clone)]
struct Departure {
    train: String,
    platform: u8,
    time: String,
}

async fn app_state_pattern() {
    println!("\n=== 6. 🏗️ Real-World Pattern — Shared App State ===\n");

    let board = Arc::new(tokio::sync::RwLock::new(DepartureBoard {
        station: "Chennai Central".to_string(),
        departures: vec![],
    }));

    // Writer task — adds departures
    let board_w = Arc::clone(&board);
    let writer = tokio::spawn(async move {
        let new_departures = vec![
            Departure {
                train: "Rajdhani Express".to_string(),
                platform: 3,
                time: "06:00".to_string(),
            },
            Departure {
                train: "Shatabdi Express".to_string(),
                platform: 5,
                time: "07:30".to_string(),
            },
            Departure {
                train: "Duronto Express".to_string(),
                platform: 1,
                time: "09:00".to_string(),
            },
        ];

        for dep in new_departures {
            let mut guard = board_w.write().await;
            println!("  [Writer] Adding {} at {}", dep.train, dep.time);
            guard.departures.push(dep);
            // Drop guard before sleeping to release the lock
            drop(guard);
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
    });

    // Wait for writer to finish, then read
    writer.await.unwrap();

    // Multiple reader tasks
    let mut readers = vec![];
    for display_id in 1..=3 {
        let board_r = Arc::clone(&board);
        let reader = tokio::spawn(async move {
            let guard = board_r.read().await;
            println!(
                "  [Display {}] {} has {} departures",
                display_id,
                guard.station,
                guard.departures.len()
            );
            for dep in &guard.departures {
                println!(
                    "    {} — Platform {} at {}",
                    dep.train, dep.platform, dep.time
                );
            }
        });
        readers.push(reader);
    }

    for r in readers {
        r.await.unwrap();
    }
}

// =========================================================================
// Main — Run All Examples
// =========================================================================

#[tokio::main]
async fn main() {
    // Thread-based examples
    arc_basics();
    mutex_basics();
    arc_mutex_combo();
    rwlock_demo();

    // Async (Tokio) examples
    tokio_shared_state().await;
    app_state_pattern().await;

    println!("\n🚂 All shared state examples complete!");
}
