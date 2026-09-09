// ======================================================
// Lesson 09 — Panics & Defensive Programming (Railway Edition)
// ======================================================
//
// Run with: cargo run
//
// This file demonstrates every concept from lesson.md.
// Panic-causing lines are COMMENTED OUT so the program runs cleanly.
// Uncomment them one at a time to see each panic in action.

use std::collections::HashMap;

fn main() {
    // =========================================================================
    // 1. 💥 Common Panic Causes (all commented out — uncomment to see each)
    // =========================================================================
    println!("=== 1. 💥 Common Panic Causes (commented out) ===\n");

    // --- unwrap() on None ---
    // UNCOMMENT TO SEE PANIC:
    // let next_train: Option<&str> = None;
    // let name = next_train.unwrap();

    // --- unwrap() on Err ---
    // UNCOMMENT TO SEE PANIC:
    // let platform: i32 = "platform_three".parse().unwrap();

    // --- Index out of bounds ---
    // UNCOMMENT TO SEE PANIC:
    // let coaches = vec!["S1", "S2", "S3"];
    // let missing = coaches[10];

    // --- Integer overflow (debug mode) ---
    // UNCOMMENT TO SEE PANIC:
    // let passengers: u8 = 255;
    // let overflow = passengers + 1;

    // --- Division by zero ---
    // UNCOMMENT TO SEE PANIC:
    // let total_fare: i32 = 1500;
    // let group_size: i32 = 0;
    // let per_person = total_fare / group_size;

    // --- Explicit panic! ---
    // UNCOMMENT TO SEE PANIC:
    // panic!("Derailment detected — emergency stop!");

    // --- todo!() ---
    // UNCOMMENT TO SEE PANIC:
    // fn calculate_dynamic_fare() -> f64 { todo!() }
    // let _fare = calculate_dynamic_fare();

    println!("  (All panic examples are commented out.)");
    println!("  Uncomment one at a time in main.rs to see each panic.\n");

    // =========================================================================
    // 2. 🪜 The Unwrap Ladder — Same Lookup, 7 Different Ways
    // =========================================================================
    println!("=== 2. 🪜 The Unwrap Ladder ===\n");

    let mut schedule: HashMap<&str, &str> = HashMap::new();
    schedule.insert("12001", "Shatabdi Express — Platform 3, 06:00");
    schedule.insert("12007", "Duronto Express — Platform 1, 14:30");
    schedule.insert("12621", "Tamil Nadu Express — Platform 5, 22:00");

    let lookup_train = "99999"; // This train doesn't exist

    // Level 1: .unwrap() — 💥 PANICS (commented out)
    // UNCOMMENT TO SEE PANIC:
    // let info = schedule.get(lookup_train).unwrap();
    println!("  Level 1: .unwrap()        — [commented out, would panic]");

    // Level 2: .expect() — 💥 PANICS with a message (commented out)
    // UNCOMMENT TO SEE PANIC:
    // let info = schedule.get(lookup_train).expect("Train not found in schedule!");
    println!("  Level 2: .expect()        — [commented out, would panic with message]");

    // Level 3: .unwrap_or() — ✅ returns a fallback
    let info = schedule
        .get(lookup_train)
        .unwrap_or(&"No information available");
    println!("  Level 3: .unwrap_or()     — {}", info);

    // Level 4: .unwrap_or_else() — ✅ computes fallback lazily
    let info = schedule.get(lookup_train).unwrap_or_else(|| {
        // This closure only runs if the key is missing
        &"[Fallback] Check station display board"
    });
    println!("  Level 4: .unwrap_or_else()— {}", info);

    // Level 5: .unwrap_or_default() — ✅ uses type's Default
    let delay: Option<i32> = None;
    let delay_minutes = delay.unwrap_or_default(); // 0 for i32
    println!(
        "  Level 5: .unwrap_or_default() — delay = {} minutes",
        delay_minutes
    );

    // Level 6: match — ✅ explicit handling
    print!("  Level 6: match            — ");
    match schedule.get(lookup_train) {
        Some(info) => println!("{}", info),
        None => println!("Train {} not in schedule", lookup_train),
    }

    // Level 7: ? operator (in a function that returns Option)
    let result = get_departure_info(&schedule, lookup_train);
    println!("  Level 7: ? operator       — {:?}", result);

    // Now look up a train that DOES exist:
    println!("\n  Looking up train 12001 (exists):");
    let info = schedule.get("12001").unwrap_or(&"Not found");
    println!("    .unwrap_or()  → {}", info);
    match schedule.get("12001") {
        Some(info) => println!("    match         → {}", info),
        None => println!("    match         → Not found"),
    }
    let result = get_departure_info(&schedule, "12001");
    println!("    ? operator    → {:?}", result);

    // =========================================================================
    // 3. 🛡️ Safe vs Unsafe — Side by Side
    // =========================================================================
    println!("\n=== 3. 🛡️ Safe vs Unsafe Patterns ===\n");

    // --- Safe Indexing ---
    println!("--- Safe Indexing ---");
    let platforms = vec!["Platform 1", "Platform 2", "Platform 3"];

    // ❌ Unsafe: platforms[10] would panic
    // ✅ Safe: .get() returns Option
    match platforms.get(0) {
        Some(p) => println!("  Index 0: {} ✓", p),
        None => println!("  Index 0: not found"),
    }
    match platforms.get(10) {
        Some(p) => println!("  Index 10: {}", p),
        None => println!("  Index 10: out of bounds — no panic! ✓"),
    }

    // --- Safe HashMap Access ---
    println!("\n--- Safe HashMap Access ---");
    let mut gates: HashMap<&str, u32> = HashMap::new();
    gates.insert("North Gate", 500);
    gates.insert("South Gate", 350);

    // ❌ Unsafe: gates["East Gate"] would panic
    // ✅ Safe: .get() returns Option
    let capacity = gates.get("North Gate").unwrap_or(&0);
    println!("  North Gate capacity: {} ✓", capacity);
    let capacity = gates.get("East Gate").unwrap_or(&0);
    println!(
        "  East Gate capacity: {} (default, key missing) ✓",
        capacity
    );

    // --- Safe Parsing ---
    println!("\n--- Safe Parsing ---");
    let inputs = vec!["42", "seven", "3", "Platform 1", "100"];
    for input in &inputs {
        let result: Result<i32, _> = input.parse();
        match result {
            Ok(n) => println!("  Parsed \"{}\" → {} ✓", input, n),
            Err(_) => println!("  Parsed \"{}\" → invalid, using default 0 ✓", input),
        }
    }

    // --- Safe Division ---
    println!("\n--- Safe Division ---");
    let test_cases = vec![(5000, 4), (3000, 0), (1200, 3), (800, 0)];
    for (fare, passengers) in &test_cases {
        match fare_per_person(*fare, *passengers) {
            Some(per) => println!("  ₹{} / {} passengers = ₹{} each ✓", fare, passengers, per),
            None => println!(
                "  ₹{} / {} passengers = can't divide by zero ✓",
                fare, passengers
            ),
        }
    }

    // =========================================================================
    // 4. ✅ Defensive Validation Functions
    // =========================================================================
    println!("\n=== 4. ✅ Defensive Validation ===\n");

    // --- Booking validator ---
    println!("--- Booking Validator ---");
    let bookings = vec![
        ("12001", 3),  // valid
        ("", 2),       // invalid train number
        ("12007", 0),  // invalid passenger count
        ("12007", 10), // too many passengers
        ("99999", 2),  // train not found
        ("12621", 4),  // valid
    ];

    let valid_trains = vec!["12001", "12007", "12621"];

    for (train, passengers) in &bookings {
        match book_ticket(train, *passengers, &valid_trains) {
            Ok(confirmation) => println!("  ✅ {}", confirmation),
            Err(e) => println!("  ❌ Booking ({}, {}) failed: {:?}", train, passengers, e),
        }
    }

    // --- Platform assignment with bounds checking ---
    println!("\n--- Platform Assignment ---");
    let station_platforms = 12; // station has platforms 1–12
    let assignments = vec![
        ("Rajdhani Express", 5),
        ("Shatabdi Express", 0),    // invalid: no platform 0
        ("Duronto Express", 15),    // invalid: exceeds station capacity
        ("Tamil Nadu Express", 12), // valid: last platform
    ];

    for (train, platform) in &assignments {
        match assign_platform(train, *platform, station_platforms) {
            Ok(msg) => println!("  ✅ {}", msg),
            Err(msg) => println!("  ❌ {}", msg),
        }
    }

    // =========================================================================
    // 5. 🔍 Assertions — For Tests and Debug Invariants
    // =========================================================================
    println!("\n=== 5. 🔍 Assertions ===\n");

    // assert! — verify a condition is true
    let train_count = 42;
    assert!(train_count > 0, "Must have at least one train");
    println!("  assert!(train_count > 0) — passed ✓");

    // assert_eq! — verify two values are equal
    let expected_platform = 3;
    let actual_platform = 3;
    assert_eq!(expected_platform, actual_platform, "Platform mismatch");
    println!("  assert_eq!(expected, actual) — passed ✓");

    // assert_ne! — verify two values are NOT equal
    let departures = 10;
    let cancellations = 2;
    assert_ne!(departures, cancellations, "These shouldn't be equal");
    println!("  assert_ne!(departures, cancellations) — passed ✓");

    // Using assertions to validate a struct
    let train = TrainSchedule {
        number: "12001".to_string(),
        name: "Shatabdi Express".to_string(),
        platform: 3,
        coaches: 16,
    };
    assert!(!train.number.is_empty());
    assert!(train.platform > 0 && train.platform <= 24);
    assert!(train.coaches > 0 && train.coaches <= 24);
    println!("  TrainSchedule invariants — all passed ✓");

    // =========================================================================
    // 6. 🚂 Putting It All Together — Defensive Departure Board
    // =========================================================================
    println!("\n=== 6. 🚂 Defensive Departure Board ===\n");

    let departures = build_departure_board();
    for line in &departures {
        println!("  {}", line);
    }

    println!("\n🚂 All examples complete — zero panics! Run with:");
    println!("  cargo run");
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Look up departure info using the ? operator (returns None to caller on miss)
fn get_departure_info(schedule: &HashMap<&str, &str>, train_number: &str) -> Option<String> {
    let info = schedule.get(train_number)?; // returns None if missing
    Some(format!("DEPARTURE: {}", info))
}

/// Safe division — returns None instead of panicking on zero
fn fare_per_person(total_fare: u32, passengers: u32) -> Option<u32> {
    if passengers == 0 {
        None
    } else {
        Some(total_fare / passengers)
    }
}

/// Booking error types — instead of panicking, we describe what went wrong
#[derive(Debug)]
#[allow(dead_code)] // Fields are read via Debug formatting
enum BookingError {
    EmptyTrainNumber,
    InvalidPassengerCount(u32),
    TrainNotFound(String),
}

/// Validate and process a booking — never panics, always returns Result
fn book_ticket(
    train_number: &str,
    passengers: u32,
    valid_trains: &[&str],
) -> Result<String, BookingError> {
    // Validate train number
    if train_number.is_empty() {
        return Err(BookingError::EmptyTrainNumber);
    }

    // Validate passenger count (railway rules: 1-6 per booking)
    if passengers == 0 || passengers > 6 {
        return Err(BookingError::InvalidPassengerCount(passengers));
    }

    // Check train exists (using .contains() instead of unwrap)
    if !valid_trains.contains(&train_number) {
        return Err(BookingError::TrainNotFound(train_number.to_string()));
    }

    Ok(format!(
        "Confirmed: {} seat(s) on train {}",
        passengers, train_number
    ))
}

/// Assign a train to a platform with bounds checking
fn assign_platform(train_name: &str, platform: u32, max_platforms: u32) -> Result<String, String> {
    if platform == 0 {
        return Err(format!(
            "{}: platform 0 doesn't exist (starts at 1)",
            train_name
        ));
    }
    if platform > max_platforms {
        return Err(format!(
            "{}: platform {} exceeds station capacity (max {})",
            train_name, platform, max_platforms
        ));
    }
    Ok(format!("{} → Platform {}", train_name, platform))
}

/// A train schedule entry
#[allow(dead_code)] // Fields used to demonstrate assertion checks
struct TrainSchedule {
    number: String,
    name: String,
    platform: u32,
    coaches: u32,
}

/// Build a departure board defensively — every lookup is safe
fn build_departure_board() -> Vec<String> {
    // Raw data — imagine this comes from a database or sensor
    let mut schedule: HashMap<&str, (&str, Option<u32>, Option<&str>)> = HashMap::new();
    schedule.insert("12001", ("Shatabdi Express", Some(3), Some("06:00")));
    schedule.insert("12007", ("Duronto Express", None, Some("14:30"))); // platform unknown
    schedule.insert("12621", ("Tamil Nadu Express", Some(5), None)); // time unknown
    schedule.insert("16525", ("Island Express", None, None)); // both unknown

    let train_numbers = vec!["12001", "12007", "12621", "16525", "99999"];

    let mut board = Vec::new();
    board.push("╔══════════════════════════════════════════════════════╗".to_string());
    board.push("║          🚂  DEPARTURE BOARD  🚂                    ║".to_string());
    board.push("╠══════════════════════════════════════════════════════╣".to_string());

    for number in &train_numbers {
        // Safe HashMap lookup — .get() returns Option, never panics
        let line = match schedule.get(number) {
            Some((name, platform, time)) => {
                // Safe unwrap alternatives for optional fields
                let platform_str = match platform {
                    Some(p) => format!("P{}", p),
                    None => "TBD".to_string(),
                };
                let time_str = time.unwrap_or(&"--:--");

                format!(
                    "║  {} │ {:22} │ {} │ {} ║",
                    number, name, platform_str, time_str
                )
            }
            None => {
                format!("║  {} │ {:22} │ --- │ --:-- ║", number, "NOT FOUND")
            }
        };
        board.push(line);
    }

    board.push("╚══════════════════════════════════════════════════════╝".to_string());
    board
}
