// =============================================
// EXERCISE 02: Train Dispatch System
// =============================================
//
// Build a simple train dispatch system using functions, control flow, and match.
//
// Complete the TODOs below so the program compiles and produces the expected output.
//
// Run with: cargo run
//
// Expected output:
//
//   === TRAIN DISPATCH SYSTEM ===
//
//   --- Fare Calculator ---
//   100 km: ₹75.00
//   500 km: ₹375.00
//   1200 km: ₹725.00
//   0 km: ₹0.00
//
//   --- Delay Announcements ---
//   Train 12001: On Time ✅
//   Train 12002: Running 5 min late ⚠️
//   Train 12003: Delayed by 45 min 🔴
//   Train 12004: Cancelled ❌
//
//   --- Platform Assignment ---
//   Rajdhani Express → Platform 1 (Rajdhani/Shatabdi)
//   Duronto Express → Platform 3 (Long distance)
//   Local Passenger → Platform 5 (Local/Passenger)
//   Garib Rath → Platform 3 (Long distance)
//   Shatabdi Express → Platform 1 (Rajdhani/Shatabdi)
//   Metro Express → Platform 5 (Local/Passenger)
//
//   --- Countdown ---
//   Train 12001 departing in 5...
//   Train 12001 departing in 4...
//   Train 12001 departing in 3...
//   Train 12001 departing in 2...
//   Train 12001 departing in 1...
//   🚂 Train 12001 has departed!
//
//   --- Active Trains ---
//   Dispatching train 1 ✓
//   Dispatching train 2 ✓
//   Train 3 is under maintenance — skipping
//   Dispatching train 4 ✓
//   Dispatching train 5 ✓
//   Train 6 is under maintenance — skipping
//   Dispatching train 7 ✓
//   Max dispatches reached. Stopping.

fn main() {
    println!("=== TRAIN DISPATCH SYSTEM ===");

    // --- Part 1: Fare Calculator ---
    println!();
    println!("--- Fare Calculator ---");
    let distances = [100, 500, 1200, 0];
    for dist in distances {
        let fare = calculate_fare(dist as f64);
        println!("{dist} km: ₹{fare:.2}");
    }

    // --- Part 2: Delay Announcements ---
    println!();
    println!("--- Delay Announcements ---");
    print_delay_status(12001, 0);
    print_delay_status(12002, 5);
    print_delay_status(12003, 45);
    print_delay_status(12004, 999);

    // --- Part 3: Platform Assignment ---
    println!();
    println!("--- Platform Assignment ---");
    let trains = [
        "Rajdhani Express",
        "Duronto Express",
        "Local Passenger",
        "Garib Rath",
        "Shatabdi Express",
        "Metro Express",
    ];
    for train in trains {
        let (platform, category) = assign_platform(train);
        println!("{train} → Platform {platform} ({category})");
    }

    // --- Part 4: Departure Countdown ---
    println!();
    println!("--- Countdown ---");
    departure_countdown(12001, 5);

    // --- Part 5: Dispatch with skip & stop ---
    println!();
    println!("--- Active Trains ---");
    dispatch_trains();
}

// TODO 1: Implement `calculate_fare`
//
// Rules:
//   - 0 or less → return 0.0
//   - 1–500     → distance * 0.75
//   - over 500  → 375.0 + (distance - 500) * 0.50
//
// Verify:
//   100  → 75.00
//   500  → 375.00
//   1200 → 375 + 350 = 725.00
//   0    → 0.00

fn calculate_fare(distance_km: f64) -> f64 {
    // TODO: Implement the tiered fare calculation using if/else
    0.0
}

// TODO 2: Implement `print_delay_status`
//
// Use `match` to print the right message based on delay_minutes:
//   - 0        → "Train {number}: On Time ✅"
//   - 1..=15   → "Train {number}: Running {delay} min late ⚠️"
//   - 16..=120 → "Train {number}: Delayed by {delay} min 🔴"
//   - _        → "Train {number}: Cancelled ❌"

fn print_delay_status(train_number: u32, delay_minutes: u32) {
    // TODO: Use match on delay_minutes and print the appropriate message
}

// TODO 3: Implement `assign_platform`
//
// Given a train name, return a tuple (platform_number, category_name):
//   - If the name contains "Rajdhani" or "Shatabdi"  → (1, "Rajdhani/Shatabdi")
//   - If the name contains "Duronto" or "Garib Rath"  → (3, "Long distance")
//   - Anything else                                    → (5, "Local/Passenger")
//
// Hint: Use `if/else if/else` with the `.contains()` method on strings.
// Example: "Rajdhani Express".contains("Rajdhani") returns true

fn assign_platform(train_name: &str) -> (u32, &str) {
    // TODO: Return the correct (platform, category) tuple
    (0, "Unknown")
}

// TODO 4: Implement `departure_countdown`
//
// Use a `while` loop (or `loop`) to count down from `seconds` to 1, printing:
//   "Train {number} departing in {n}..."
// After the loop, print:
//   "🚂 Train {number} has departed!"

fn departure_countdown(train_number: u32, seconds: u32) {
    // TODO: Count down from `seconds` to 1 using a while loop, then print departed message
}

// TODO 5: Implement `dispatch_trains`
//
// Use a `for` loop over train IDs 1..=10
//   - If the ID is 3 or 6 → print "Train {id} is under maintenance — skipping"
//     and `continue` to skip it
//   - If you've dispatched 5 trains → print "Max dispatches reached. Stopping."
//     and `break` out of the loop
//   - Otherwise → print "Dispatching train {id} ✓"
//
// Hint: You'll need a mutable counter to track how many you've dispatched.

fn dispatch_trains() {
    // TODO: Loop through train IDs, skip maintenance trains, stop after 5 dispatches
}
