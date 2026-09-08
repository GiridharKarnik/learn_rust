// =============================================
// EXERCISE 04: Railway Station Management
// =============================================
//
// Build a railway station management system using structs and methods.
// You write EVERYTHING from scratch — structs, impl blocks, and main().
//
// Run with: cargo run
//
// =============================================
// EXPECTED OUTPUT:
// =============================================
//
//   === RAILWAY STATION MANAGEMENT ===
//
//   --- Creating Trains ---
//   Created: Rajdhani Express (#12001) [Express]
//   Created: Shatabdi Express (#12002) [Express]
//   Created: Chennai Local (#43210) [Regular]
//
//   --- Station Info ---
//   Chennai Central (MAS) — 17 platforms
//   Bangalore City (SBC) — 10 platforms
//
//   --- Train Operations ---
//   Rajdhani Express: 0 km/h → accelerating...
//   Rajdhani Express: now at 80 km/h
//   Rajdhani Express: now at 130 km/h
//   Rajdhani Express is fast: true
//   Chennai Local is fast: false
//
//   --- Arrivals ---
//   📢 MAS: Rajdhani Express (#12001) arriving on platform 1
//   📢 MAS: Shatabdi Express (#12002) arriving on platform 3
//   📢 SBC: Chennai Local (#43210) arriving on platform 5
//
//   --- Passenger Manifest ---
//   Passenger: Giridhar Mohan (giridhar@email.com)
//     Ticket #1001: Rajdhani Express | MAS → SBC | ₹1250.00 | Confirmed
//     Ticket #1002: Shatabdi Express | SBC → MAS | ₹875.50 | Waitlisted
//
//   --- Ticket Operations ---
//   Ticket #1002 confirmed!
//     Ticket #1002: Shatabdi Express | SBC → MAS | ₹875.50 | Confirmed
//   Ticket #1001 cancelled. Refund: ₹1062.50
//     Ticket #1001: Rajdhani Express | MAS → SBC | ₹1062.50 | Cancelled (refund pending)
//
//   --- Summary ---
//   Total tickets: 2
//   Total spent: ₹1938.00 (before cancellations)

// =============================================
// STEP 1: Define the `Train` struct
// =============================================
//
// Fields:
//   name: String
//   number: u32
//   speed_kmh: f64
//   is_express: bool
//
// Derive: Debug, Clone

// =============================================
// STEP 2: Implement `Train` methods
// =============================================
//
// Associated functions:
//
//   fn new(name: &str, number: u32) -> Self
//     Creates a regular train. speed_kmh starts at 0.0, is_express = false.
//
//   fn new_express(name: &str, number: u32) -> Self
//     Creates an express train. speed_kmh starts at 0.0, is_express = true.
//
// Methods:
//
//   fn display(&self)
//     Prints: "Created: {name} (#{number}) [{tag}]"
//     where tag is "Express" if is_express, otherwise "Regular".
//
//   fn accelerate(&mut self, amount: f64)
//     Adds amount to speed_kmh. Cap at 200.0 (if speed > 200, set to 200).
//     Prints: "{name}: now at {speed_kmh} km/h"
//
//   fn is_fast(&self) -> bool
//     Returns true if speed_kmh > 100.0

// =============================================
// STEP 3: Define the `Station` struct
// =============================================
//
// Fields:
//   name: String
//   code: String
//   platforms: u8
//
// Derive: Debug

// =============================================
// STEP 4: Implement `Station` methods
// =============================================
//
//   fn new(name: &str, code: &str, platforms: u8) -> Self
//
//   fn info(&self)
//     Prints: "{name} ({code}) — {platforms} platforms"
//     (The — is an em dash, Unicode \u{2014})
//
//   fn announce_arrival(&self, train: &Train, platform: u8)
//     Prints: "📢 {code}: {train.name} (#{train.number}) arriving on platform {platform}"
//     (📢 is Unicode \u{1f4e2})

// =============================================
// STEP 5: Define the `Ticket` struct
// =============================================
//
// Fields:
//   id: u32
//   train_name: String
//   from_code: String
//   to_code: String
//   price: f64
//   status: String
//
// Derive: Debug, Clone

// =============================================
// STEP 6: Implement `Ticket` methods
// =============================================
//
//   fn new(id: u32, train_name: &str, from: &str, to: &str, price: f64, status: &str) -> Self
//
//   fn display(&self)
//     Prints: "  Ticket #{id}: {train_name} | {from_code} → {to_code} | ₹{price:.2} | {status}"
//     (→ is Unicode \u{2192}, ₹ is Unicode \u{20b9}, note the two leading spaces)
//
//   fn confirm(&mut self)
//     Sets status to "Confirmed".
//     Prints: "Ticket #{id} confirmed!"
//
//   fn cancel(&mut self)
//     Sets status to "Cancelled (refund pending)".
//     Sets price to price * 0.85 (15% cancellation fee).
//     Prints: "Ticket #{id} cancelled. Refund: ₹{price:.2}"

// =============================================
// STEP 7: Define the `Passenger` struct
// =============================================
//
// Fields:
//   name: String
//   email: String
//   tickets: Vec<Ticket>
//
// Derive: Debug

// =============================================
// STEP 8: Implement `Passenger` methods
// =============================================
//
//   fn new(name: &str, email: &str) -> Self
//     Creates a passenger with an empty tickets Vec.
//
//   fn display(&self)
//     Prints: "Passenger: {name} ({email})"
//
//   fn add_ticket(&mut self, ticket: Ticket)
//     Pushes the ticket onto the tickets Vec.
//
//   fn get_ticket(&mut self, index: usize) -> &mut Ticket
//     Returns a mutable reference to the ticket at the given index.
//
//   fn total_spent(&self) -> f64
//     Returns the sum of all ticket prices.
//
//   fn ticket_count(&self) -> usize
//     Returns the number of tickets.

// =============================================
// STEP 9: Write main()
// =============================================
//
// Your main() should do the following, in order:
//
// 1. Print "=== RAILWAY STATION MANAGEMENT ==="
//
// 2. Print "\n--- Creating Trains ---"
//    Create three trains:
//      - Rajdhani Express (#12001) — express
//      - Shatabdi Express (#12002) — express
//      - Chennai Local (#43210) — regular
//    Call display() on each.
//
// 3. Print "\n--- Station Info ---"
//    Create two stations:
//      - Chennai Central, code "MAS", 17 platforms
//      - Bangalore City, code "SBC", 10 platforms
//    Call info() on each.
//
// 4. Print "\n--- Train Operations ---"
//    Print "Rajdhani Express: {speed} km/h → accelerating..." (speed starts at 0)
//    Accelerate Rajdhani by 80.0, then by 50.0.
//    Print "Rajdhani Express is fast: {}" with is_fast().
//    Print "Chennai Local is fast: {}" with is_fast().
//
// 5. Print "\n--- Arrivals ---"
//    Announce arrivals:
//      - Rajdhani at Chennai Central, platform 1
//      - Shatabdi at Chennai Central, platform 3
//      - Chennai Local at Bangalore City, platform 5
//
// 6. Print "\n--- Passenger Manifest ---"
//    Create passenger: "Giridhar Mohan", "giridhar@email.com"
//    Create two tickets:
//      - #1001, "Rajdhani Express", "MAS" → "SBC", ₹1250.00, "Confirmed"
//      - #1002, "Shatabdi Express", "SBC" → "MAS", ₹875.50, "Waitlisted"
//    Add both tickets to the passenger.
//    Call passenger.display().
//    Display ticket at index 0, then ticket at index 1.
//
// 7. Print "\n--- Ticket Operations ---"
//    Confirm ticket at index 1, then display it.
//    Cancel ticket at index 0, then display it.
//
// 8. Print "\n--- Summary ---"
//    Print "Total tickets: {}" with ticket_count().
//    Print "Total spent: ₹{:.2} (before cancellations)" with total_spent().

fn main() {
    // Your code here
}
