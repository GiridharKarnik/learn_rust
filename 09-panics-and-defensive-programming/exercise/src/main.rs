// =============================================
// EXERCISE 09: Railway Ticket Validator
// =============================================
//
// Build a crash-proof ticket validation system from scratch.
// The goal: ZERO panics possible. Every error is handled gracefully.
// There is NO code provided — you write everything, including main().
//
// This exercises: Result, Option, validation, safe indexing, defensive patterns.
//
// Run with: cargo run
//
// Expected output:
//
//   === RAILWAY TICKET VALIDATOR ===
//
//   --- Booking Errors ---
//     Ticket #3: passenger name is empty
//     Ticket #4: origin and destination are the same
//     Unknown ticket class: XYZ
//     Ticket #6: invalid price ₹-100.00
//     Ticket #8: price ₹15000.00 exceeds maximum
//
//   --- Valid Tickets ---
//     Ticket #1: Giridhar | Chennai → Bangalore | FirstAC | ₹1250.00
//     Ticket #2: Priya | Mumbai → Delhi | Sleeper | ₹450.00
//     Ticket #7: Suresh | Mumbai → Chennai | General | ₹350.00
//
//   --- Safe Lookups ---
//     Ticket #1: Found — Giridhar | Chennai → Bangalore
//     Ticket #99: Not found
//     Index 0: Found — Giridhar | Chennai → Bangalore
//     Index 99: Not found (safe — no panic!)
//
//   --- Refund Calculations ---
//     85% refund on ₹1250.00: ₹1062.50
//     150% refund: Error — Invalid refund percentage: 150
//     Refund on ₹-500.00: Error — Invalid price: -500

// =============================================
// STEP 1: Define `TicketClass` enum
// =============================================
// Variants: FirstAC, SecondAC, Sleeper, General
// Derive: Debug, Clone

// =============================================
// STEP 2: Implement `parse_ticket_class`
// =============================================
// fn parse_ticket_class(input: &str) -> Result<TicketClass, String>
//
// Case-insensitive (use .to_lowercase()):
//   "1a" or "first"     → Ok(FirstAC)
//   "2a" or "second"    → Ok(SecondAC)
//   "sl" or "sleeper"   → Ok(Sleeper)
//   "gn" or "general"   → Ok(General)
//   "" (empty)           → Err("Ticket class cannot be empty")
//   anything else        → Err("Unknown ticket class: {input}")
//
// WHY: Return Result instead of panicking on bad input.

// =============================================
// STEP 3: Define `Ticket` struct
// =============================================
// Fields: id (u32), passenger (String), from (String), to (String),
//         class (TicketClass), price (f64)
// Derive: Debug, Clone

// =============================================
// STEP 4: Implement `Ticket` methods
// =============================================
// - fn new(id, passenger: &str, from: &str, to: &str, class: TicketClass, price: f64) -> Self
// - fn display(&self) -> String
//     Returns: "Ticket #{id}: {passenger} | {from} → {to} | {class:?} | ₹{price:.2}"

// =============================================
// STEP 5: Implement `validate_ticket`
// =============================================
// fn validate_ticket(ticket: &Ticket) -> Result<(), String>
//
// Check these conditions — return Err with message if any fail:
//   - passenger name empty     → "Ticket #{id}: passenger name is empty"
//   - from == to               → "Ticket #{id}: origin and destination are the same"
//   - price <= 0               → "Ticket #{id}: invalid price ₹{price:.2}"
//   - price > 10000            → "Ticket #{id}: price ₹{price:.2} exceeds maximum"
//   - All checks pass          → Ok(())
//
// WHY: Validation returns Result, not panic. Caller decides what to do.

// =============================================
// STEP 6: Implement `find_ticket`
// =============================================
// fn find_ticket(tickets: &[Ticket], id: u32) -> Option<&Ticket>
//
// Search by id. Return Some or None. Never panic.

// =============================================
// STEP 7: Implement `get_ticket_at`
// =============================================
// fn get_ticket_at(tickets: &[Ticket], index: usize) -> Option<&Ticket>
//
// Use tickets.get(index) — NOT tickets[index].
// .get() returns Option, [] panics on out of bounds.
//
// WHY: This is the #1 defensive pattern — safe indexing.

// =============================================
// STEP 8: Implement `calculate_refund`
// =============================================
// fn calculate_refund(price: f64, percent: f64) -> Result<f64, String>
//
//   percent < 0 or > 100  → Err("Invalid refund percentage: {percent}")
//   price < 0             → Err("Invalid price: {price}")
//   otherwise             → Ok(price * percent / 100.0)
//
// WHY: Validate numeric inputs. Bad math doesn't panic but
//      returns nonsense — Result catches that.

// =============================================
// STEP 9: Implement `process_booking`
// =============================================
// fn process_booking(inputs: &[(&str, &str, &str, &str, f64)]) -> (Vec<Ticket>, Vec<String>)
//
// Takes raw booking data: (passenger, from, to, class_str, price)
// Returns: (valid_tickets, error_messages)
//
// For each input:
//   1. Parse class_str with parse_ticket_class
//      → If Err, push the error message into errors and skip this booking
//   2. Create a Ticket (auto-incrementing id starting from 1)
//   3. Validate with validate_ticket
//      → If Err, push the error message into errors and skip
//   4. If all passed, push ticket into valid_tickets
//
// NOTE: id increments for EVERY attempt, not just successful ones.
//       So the first booking is #1, the second is #2, even if #1 failed.
//
// WHY: This is the capstone — combining Result, validation, and
//      graceful error collection. No panics anywhere.

// =============================================
// STEP 10: Write `fn main()`
// =============================================
//
// 1. Print "=== RAILWAY TICKET VALIDATOR ==="
//
// 2. Define booking inputs:
//   ("Giridhar", "Chennai", "Bangalore", "1A", 1250.00)     ← valid
//   ("Priya", "Mumbai", "Delhi", "SL", 450.00)              ← valid
//   ("", "Chennai", "Mumbai", "2A", 800.00)                 ← empty passenger
//   ("Arjun", "Bangalore", "Bangalore", "GN", 200.00)       ← same from/to
//   ("Meera", "Delhi", "Chennai", "XYZ", 500.00)            ← invalid class
//   ("Ravi", "Chennai", "Kolkata", "1A", -100.00)           ← negative price
//   ("Suresh", "Mumbai", "Chennai", "general", 350.00)      ← valid
//   ("Anita", "Chennai", "Delhi", "first", 15000.00)        ← price too high
//
// 3. Call process_booking
//
// 4. Print "--- Booking Errors ---"
//    Print each error with "  " indent
//
// 5. Print "--- Valid Tickets ---"
//    Print each ticket with ticket.display()
//
// 6. Print "--- Safe Lookups ---"
//    find_ticket for id 1 → print found info
//    find_ticket for id 99 → print "Not found"
//    get_ticket_at index 0 → print found info
//    get_ticket_at index 99 → print "Not found (safe — no panic!)"
//
// 7. Print "--- Refund Calculations ---"
//    calculate_refund(1250.0, 85.0) → print Ok result
//    calculate_refund(1250.0, 150.0) → print Err
//    calculate_refund(-500.0, 85.0) → print Err

fn main() {
    // Your code here
}
