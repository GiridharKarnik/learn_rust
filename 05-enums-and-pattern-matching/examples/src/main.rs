// =============================================================================
// Lesson 05: Enums and Pattern Matching 🚂
// =============================================================================
// This file demonstrates all major enum concepts in Rust using railway themes.
// Run with: cargo run
// =============================================================================

// ---------------------------------------------------------------------------
// Common types used throughout the examples
// ---------------------------------------------------------------------------

/// The operational status of a train.
#[derive(Debug, Clone)]
enum TrainStatus {
    OnTime,
    Delayed(u32),      // delay in minutes
    Cancelled(String), // reason for cancellation
}

/// A train running on the railway network.
#[derive(Debug, Clone)]
struct Train {
    name: String,
    number: u32,
    status: TrainStatus,
}

fn main() {
    section_1_basic_enums();
    section_2_enums_with_different_data();
    section_3_match_guards();
    section_4_option();
    section_5_result();
    section_6_if_let_while_let();
    section_7_methods_on_enums();
    section_8_enums_and_structs_together();
}

// =============================================================================
// 1. Basic Enums
// =============================================================================

fn section_1_basic_enums() {
    println!("\n🚂 ═══════════════════════════════════════════════════════");
    println!("   1. BASIC ENUMS");
    println!("   ═══════════════════════════════════════════════════════\n");

    // Create instances of each variant
    let status1 = TrainStatus::OnTime;
    let status2 = TrainStatus::Delayed(15);
    let status3 = TrainStatus::Cancelled(String::from("Severe weather warning"));

    // Match on each status to produce a human-readable message
    let statuses = [status1, status2, status3];
    for status in &statuses {
        let message = match status {
            TrainStatus::OnTime => String::from("✅ Train is on time"),
            TrainStatus::Delayed(mins) => format!("⏰ Train is delayed by {} minutes", mins),
            TrainStatus::Cancelled(reason) => format!("❌ Train cancelled: {}", reason),
        };
        println!("   {:?} → {}", status, message);
    }

    // Using the wildcard / catch-all pattern
    let mystery_status = TrainStatus::Delayed(5);
    match mystery_status {
        TrainStatus::Cancelled(_) => println!("   This train won't run today."),
        _ => println!("\n   🔔 The train is still expected to run (catch-all matched)."),
    }
}

// =============================================================================
// 2. Enums with Different Data Types
// =============================================================================

/// Events that can occur during a train's journey.
#[derive(Debug)]
enum TrainEvent {
    Departed,                                         // unit variant (no data)
    Arrived { station: String, platform: u8 },        // struct-like variant
    Delayed(u32),                                     // tuple-like (minutes)
    SpeedChanged(f64, f64),                           // (old_speed, new_speed)
    Cancelled { reason: String, refund_percent: u8 }, // struct-like variant
}

fn section_2_enums_with_different_data() {
    println!("\n🚂 ═══════════════════════════════════════════════════════");
    println!("   2. ENUMS WITH DIFFERENT DATA TYPES");
    println!("   ═══════════════════════════════════════════════════════\n");

    let events: Vec<TrainEvent> = vec![
        TrainEvent::Departed,
        TrainEvent::SpeedChanged(0.0, 80.5),
        TrainEvent::Arrived {
            station: String::from("Bristol Temple Meads"),
            platform: 3,
        },
        TrainEvent::Delayed(12),
        TrainEvent::SpeedChanged(80.5, 125.0),
        TrainEvent::Arrived {
            station: String::from("London Paddington"),
            platform: 7,
        },
        TrainEvent::Cancelled {
            reason: String::from("Signal failure"),
            refund_percent: 100,
        },
    ];

    for event in &events {
        let description = match event {
            TrainEvent::Departed => String::from("🚀 Train has departed"),
            TrainEvent::Arrived { station, platform } => {
                format!("🏁 Arrived at {} (platform {})", station, platform)
            }
            TrainEvent::Delayed(mins) => {
                format!("⏰ Delayed by {} minutes", mins)
            }
            TrainEvent::SpeedChanged(old, new) => {
                format!("💨 Speed changed from {:.1} → {:.1} km/h", old, new)
            }
            TrainEvent::Cancelled {
                reason,
                refund_percent,
            } => {
                format!("❌ Cancelled — {} ({}% refund)", reason, refund_percent)
            }
        };
        println!("   {}", description);
    }
}

// =============================================================================
// 3. Match Guards
// =============================================================================

fn section_3_match_guards() {
    println!("\n🚂 ═══════════════════════════════════════════════════════");
    println!("   3. MATCH GUARDS");
    println!("   ═══════════════════════════════════════════════════════\n");

    let statuses = vec![
        TrainStatus::OnTime,
        TrainStatus::Delayed(5),
        TrainStatus::Delayed(15),
        TrainStatus::Delayed(45),
        TrainStatus::Delayed(90),
        TrainStatus::Cancelled(String::from("Strike action")),
    ];

    for status in &statuses {
        let urgency = match status {
            TrainStatus::OnTime => "🟢 All clear — no action needed",
            TrainStatus::Delayed(mins) if *mins <= 10 => "🟡 Minor delay — please wait",
            TrainStatus::Delayed(mins) if *mins <= 30 => "🟠 Moderate delay — check alternatives",
            TrainStatus::Delayed(mins) if *mins <= 60 => "🔴 Major delay — consider rebooking",
            TrainStatus::Delayed(_) => "🚨 Severe delay — refund recommended",
            TrainStatus::Cancelled(_) => "⛔ Cancelled — full refund issued",
        };
        println!("   {:?}", status);
        println!("       → {}\n", urgency);
    }
}

// =============================================================================
// 4. Option<T>
// =============================================================================

/// Search a slice of trains by train number, returning an Option.
fn find_train(trains: &[Train], number: u32) -> Option<&Train> {
    for train in trains {
        if train.number == number {
            return Some(train);
        }
    }
    None
}

fn section_4_option() {
    println!("🚂 ═══════════════════════════════════════════════════════");
    println!("   4. OPTION<T>");
    println!("   ═══════════════════════════════════════════════════════\n");

    let trains = vec![
        Train {
            name: String::from("Flying Scotsman"),
            number: 4472,
            status: TrainStatus::OnTime,
        },
        Train {
            name: String::from("Mallard Express"),
            number: 4468,
            status: TrainStatus::Delayed(20),
        },
        Train {
            name: String::from("Eurostar"),
            number: 9001,
            status: TrainStatus::Cancelled(String::from("Tunnel maintenance")),
        },
    ];

    // --- Matching on Option ---
    println!("   📌 Matching on Option:");
    let result = find_train(&trains, 4472);
    match result {
        Some(train) => println!(
            "      Found: {} (#{}) — {:?}",
            train.name, train.number, train.status
        ),
        None => println!("      Train not found!"),
    }

    let result = find_train(&trains, 9999);
    match result {
        Some(train) => println!("      Found: {}", train.name),
        None => println!("      Train #9999 not found!\n"),
    }

    // --- .is_some() and .is_none() ---
    println!("   📌 .is_some() and .is_none():");
    let found = find_train(&trains, 4468);
    println!("      find_train(4468).is_some() = {}", found.is_some());
    println!("      find_train(4468).is_none() = {}", found.is_none());
    let missing = find_train(&trains, 1111);
    println!("      find_train(1111).is_some() = {}", missing.is_some());
    println!("      find_train(1111).is_none() = {}\n", missing.is_none());

    // --- .unwrap_or() ---
    println!("   📌 .unwrap_or() — provide a default:");
    let default_train = Train {
        name: String::from("Unknown Service"),
        number: 0,
        status: TrainStatus::Cancelled(String::from("Does not exist")),
    };
    let train = find_train(&trains, 9999).unwrap_or(&default_train);
    println!("      Got: {} (#{})\n", train.name, train.number);

    // --- .map() to transform ---
    println!("   📌 .map() — transform the inner value:");
    let name: Option<&str> = find_train(&trains, 4472).map(|t| t.name.as_str());
    println!("      Train 4472 name: {:?}", name);

    let name: Option<&str> = find_train(&trains, 9999).map(|t| t.name.as_str());
    println!("      Train 9999 name: {:?}\n", name);

    // --- .get() on arrays returns Option ---
    println!("   📌 .get() on slices returns Option:");
    let platforms = ["Platform 1", "Platform 2", "Platform 3"];
    println!("      platforms.get(0) = {:?}", platforms.get(0));
    println!("      platforms.get(5) = {:?}", platforms.get(5));
    match platforms.get(1) {
        Some(p) => println!("      Your train departs from: {}", p),
        None => println!("      Invalid platform!"),
    }
}

// =============================================================================
// 5. Result<T, E>
// =============================================================================

/// Validate a platform number (must be 1..=15).
fn validate_platform(num: u32) -> Result<u32, String> {
    if num == 0 {
        Err(String::from("Platform numbers start at 1"))
    } else if num > 15 {
        Err(format!("Platform {} does not exist (max is 15)", num))
    } else {
        Ok(num)
    }
}

fn section_5_result() {
    println!("\n🚂 ═══════════════════════════════════════════════════════");
    println!("   5. RESULT<T, E>");
    println!("   ═══════════════════════════════════════════════════════\n");

    // --- Parsing strings to numbers (returns Result) ---
    println!("   📌 Parsing strings with .parse::<u32>():");

    let good_input = "42";
    let bad_input = "not_a_number";

    let parsed: Result<u32, _> = good_input.parse::<u32>();
    match parsed {
        Ok(num) => println!("      Parsed '{}' → {}", good_input, num),
        Err(e) => println!("      Failed to parse '{}': {}", good_input, e),
    }

    let parsed: Result<u32, _> = bad_input.parse::<u32>();
    match parsed {
        Ok(num) => println!("      Parsed '{}' → {}", bad_input, num),
        Err(e) => println!("      Failed to parse '{}': {}\n", bad_input, e),
    }

    // --- .unwrap() and .expect() ---
    println!("   📌 .unwrap() and .expect() — use only when you're certain:");
    let safe_value: u32 = "100".parse().unwrap();
    println!("      .unwrap()  on \"100\" → {}", safe_value);

    let safe_value: u32 = "200".parse().expect("This should always parse");
    println!("      .expect()  on \"200\" → {}\n", safe_value);

    // --- Custom function returning Result ---
    println!("   📌 Custom validate_platform() function:");

    let test_platforms = [0, 1, 7, 15, 16, 99];
    for &num in &test_platforms {
        match validate_platform(num) {
            Ok(p) => println!("      ✅ Platform {} is valid", p),
            Err(e) => println!("      ❌ Platform {} invalid: {}", num, e),
        }
    }
}

// =============================================================================
// 6. if let / while let
// =============================================================================

fn section_6_if_let_while_let() {
    println!("\n🚂 ═══════════════════════════════════════════════════════");
    println!("   6. IF LET / WHILE LET");
    println!("   ═══════════════════════════════════════════════════════\n");

    let trains = vec![
        Train {
            name: String::from("Highland Chieftain"),
            number: 1234,
            status: TrainStatus::OnTime,
        },
        Train {
            name: String::from("Caledonian Sleeper"),
            number: 5678,
            status: TrainStatus::Delayed(30),
        },
    ];

    // --- if let ---
    println!("   📌 if let — concise single-pattern matching:");

    if let Some(train) = find_train(&trains, 1234) {
        println!(
            "      Found train: {} (on time? {:?})",
            train.name, train.status
        );
    }

    if let Some(train) = find_train(&trains, 9999) {
        println!("      Found: {}", train.name);
    } else {
        println!("      Train #9999 not found (else branch)\n");
    }

    // if let with enum variant
    let status = TrainStatus::Delayed(42);
    if let TrainStatus::Delayed(mins) = &status {
        println!("      Delay detected: {} minutes\n", mins);
    }

    // --- while let ---
    println!("   📌 while let — loop until the pattern stops matching:");

    let mut event_queue: Vec<TrainEvent> = vec![
        TrainEvent::Departed,
        TrainEvent::Delayed(5),
        TrainEvent::SpeedChanged(60.0, 100.0),
        TrainEvent::Arrived {
            station: String::from("Edinburgh Waverley"),
            platform: 11,
        },
    ];

    // .pop() returns Option<T> — while let drains the queue
    while let Some(event) = event_queue.pop() {
        match event {
            TrainEvent::Departed => println!("      🚀 Departed!"),
            TrainEvent::Arrived { station, platform } => {
                println!("      🏁 Arrived: {} (platform {})", station, platform);
            }
            TrainEvent::Delayed(mins) => println!("      ⏰ Delayed {} mins", mins),
            TrainEvent::SpeedChanged(old, new) => {
                println!("      💨 Speed {:.0} → {:.0} km/h", old, new);
            }
            TrainEvent::Cancelled { reason, .. } => {
                println!("      ❌ Cancelled: {}", reason);
            }
        }
    }
    println!(
        "      📭 Event queue is now empty (len = {})",
        event_queue.len()
    );
}

// =============================================================================
// 7. Methods on Enums
// =============================================================================

impl TrainStatus {
    /// Returns a human-readable description of the status.
    fn description(&self) -> &str {
        match self {
            TrainStatus::OnTime => "Running on time",
            TrainStatus::Delayed(_) => "Currently delayed",
            TrainStatus::Cancelled(_) => "Service cancelled",
        }
    }

    /// Returns true if the train is still expected to run.
    fn is_running(&self) -> bool {
        match self {
            TrainStatus::OnTime | TrainStatus::Delayed(_) => true,
            TrainStatus::Cancelled(_) => false,
        }
    }

    /// Returns an emoji representing the status.
    fn emoji(&self) -> &str {
        match self {
            TrainStatus::OnTime => "✅",
            TrainStatus::Delayed(_) => "⏰",
            TrainStatus::Cancelled(_) => "❌",
        }
    }
}

fn section_7_methods_on_enums() {
    println!("\n🚂 ═══════════════════════════════════════════════════════");
    println!("   7. METHODS ON ENUMS");
    println!("   ═══════════════════════════════════════════════════════\n");

    let statuses = vec![
        TrainStatus::OnTime,
        TrainStatus::Delayed(25),
        TrainStatus::Cancelled(String::from("Flooding on tracks")),
    ];

    for status in &statuses {
        println!(
            "   {} {:?}\n      description: \"{}\"\n      is_running:   {}\n",
            status.emoji(),
            status,
            status.description(),
            status.is_running(),
        );
    }
}

// =============================================================================
// 8. Enums + Structs Together
// =============================================================================

impl Train {
    /// Create a new on-time train.
    fn new(name: &str, number: u32) -> Self {
        Train {
            name: String::from(name),
            number,
            status: TrainStatus::OnTime,
        }
    }

    /// Returns a formatted departure-board style summary.
    fn board_display(&self) -> String {
        let status_text = match &self.status {
            TrainStatus::OnTime => String::from("ON TIME"),
            TrainStatus::Delayed(mins) => format!("DELAYED {}min", mins),
            TrainStatus::Cancelled(reason) => format!("CANCELLED ({})", reason),
        };
        format!(
            "{} #{:>5}  │  {}  │  {}",
            self.status.emoji(),
            self.number,
            status_text,
            self.name,
        )
    }

    /// Delay the train by a given number of minutes.
    fn delay(&mut self, minutes: u32) {
        self.status = TrainStatus::Delayed(minutes);
    }

    /// Cancel the train with a reason.
    fn cancel(&mut self, reason: &str) {
        self.status = TrainStatus::Cancelled(String::from(reason));
    }

    /// Check whether this train is still running.
    fn is_running(&self) -> bool {
        self.status.is_running()
    }
}

fn section_8_enums_and_structs_together() {
    println!("\n🚂 ═══════════════════════════════════════════════════════");
    println!("   8. ENUMS + STRUCTS TOGETHER");
    println!("   ═══════════════════════════════════════════════════════\n");

    let mut trains = vec![
        Train::new("Flying Scotsman", 4472),
        Train::new("Mallard Express", 4468),
        Train::new("Eurostar", 9001),
        Train::new("Caledonian Sleeper", 1234),
        Train::new("Pendolino", 3900),
    ];

    // Mutate some trains
    trains[1].delay(15);
    trains[2].cancel("Tunnel maintenance");
    trains[4].delay(55);

    // Display a departure board
    println!("   ┌─────────────────────────────────────────────────────────┐");
    println!("   │              🚉  DEPARTURE BOARD  🚉                   │");
    println!("   ├─────────────────────────────────────────────────────────┤");
    for train in &trains {
        println!("   │  {}  │", train.board_display());
    }
    println!("   └─────────────────────────────────────────────────────────┘\n");

    // Filter running trains
    let running: Vec<&Train> = trains.iter().filter(|t| t.is_running()).collect();
    println!("   🟢 Trains still running: {}", running.len());
    for train in &running {
        println!(
            "      • {} (#{}) — {:?}",
            train.name, train.number, train.status
        );
    }

    let cancelled: Vec<&Train> = trains.iter().filter(|t| !t.is_running()).collect();
    println!("\n   🔴 Cancelled trains: {}", cancelled.len());
    for train in &cancelled {
        println!(
            "      • {} (#{}) — {:?}",
            train.name, train.number, train.status
        );
    }

    println!("\n   🎉 Lesson 05 complete! You've mastered enums and pattern matching.\n");
}
