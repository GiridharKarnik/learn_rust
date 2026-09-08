// =============================================
// EXERCISE 02: Train Dispatch System
// =============================================
//
// Write ALL functions AND main() yourself.
// Your program should produce EXACTLY this output:
//
// === TRAIN DISPATCH SYSTEM ===
//
// --- Fare Calculator ---
// 100 km: ₹75.00
// 500 km: ₹375.00
// 1200 km: ₹725.00
// 0 km: ₹0.00
//
// --- Delay Announcements ---
// Train 12001: On Time ✅
// Train 12002: Running 5 min late ⚠️
// Train 12003: Delayed by 45 min 🔴
// Train 12004: Cancelled ❌
//
// --- Platform Assignment ---
// Rajdhani Express → Platform 1 (Rajdhani/Shatabdi)
// Duronto Express → Platform 3 (Long distance)
// Local Passenger → Platform 5 (Local/Passenger)
// Garib Rath → Platform 3 (Long distance)
// Shatabdi Express → Platform 1 (Rajdhani/Shatabdi)
// Metro Express → Platform 5 (Local/Passenger)
//
// --- Countdown ---
// Train 12001 departing in 5...
// Train 12001 departing in 4...
// Train 12001 departing in 3...
// Train 12001 departing in 2...
// Train 12001 departing in 1...
// 🚂 Train 12001 has departed!
//
// --- Active Trains ---
// Dispatching train 1 ✓
// Dispatching train 2 ✓
// Train 3 is under maintenance — skipping
// Dispatching train 4 ✓
// Dispatching train 5 ✓
// Train 6 is under maintenance — skipping
// Dispatching train 7 ✓
// Max dispatches reached. Stopping.
//
// =============================================
// STEP 1: Implement calculate_fare(distance_km: f64) -> f64
//   - If distance <= 0, return 0.0 (use early return)
//   - First 500 km: ₹0.75 per km
//   - Beyond 500 km: ₹0.50 per km
//   - Example: 1200 km = (500 × 0.75) + (700 × 0.50) = 375 + 350 = 725
//
// STEP 2: Implement print_delay_status(train_number: u32, delay_minutes: u32)
//   - Use a match expression on delay_minutes:
//     0         → "Train {number}: On Time ✅"
//     1..=15    → "Train {number}: Running {delay} min late ⚠️"
//     16..=120  → "Train {number}: Delayed by {delay} min 🔴"
//     _         → "Train {number}: Cancelled ❌"
//
// STEP 3: Implement assign_platform(train_name: &str) -> (u32, &str)
//   - Returns a tuple of (platform_number, category)
//   - If name contains "Rajdhani" or "Shatabdi" → (1, "Rajdhani/Shatabdi")
//   - If name contains "Duronto" or "Garib Rath" → (3, "Long distance")
//   - Otherwise → (5, "Local/Passenger")
//   - Hint: use .contains("...") on the &str
//
// STEP 4: Implement departure_countdown(train_number: u32, seconds: u32)
//   - Use a while loop counting down from `seconds` to 1
//   - Print "Train {number} departing in {remaining}..." each iteration
//   - After the loop, print "🚂 Train {number} has departed!"
//
// STEP 5: Implement dispatch_trains()
//   - Loop through train IDs 1..=10 using a for loop
//   - Skip (continue) trains 3 and 6 — print "{id} is under maintenance — skipping"
//   - Track how many trains dispatched; stop (break) after 5 dispatches
//     printing "Max dispatches reached. Stopping."
//   - For each dispatched train, print "Dispatching train {id} ✓"
//   - Important: check the skip condition BEFORE the stop condition
//
// STEP 6: Write main() that:
//   - Prints "=== TRAIN DISPATCH SYSTEM ==="
//   - Part 1: Prints "--- Fare Calculator ---" header, then loops over
//     distances [100, 500, 1200, 0] calling calculate_fare for each
//   - Part 2: Prints "--- Delay Announcements ---" header, then calls
//     print_delay_status for (12001,0), (12002,5), (12003,45), (12004,999)
//   - Part 3: Prints "--- Platform Assignment ---" header, then loops over
//     train names ["Rajdhani Express", "Duronto Express", "Local Passenger",
//     "Garib Rath", "Shatabdi Express", "Metro Express"] calling assign_platform
//   - Part 4: Prints "--- Countdown ---" header, calls departure_countdown(12001, 5)
//   - Part 5: Prints "--- Active Trains ---" header, calls dispatch_trains()
//   - Separate each section with an empty line before the header

fn main() {
    // Your code here
}
