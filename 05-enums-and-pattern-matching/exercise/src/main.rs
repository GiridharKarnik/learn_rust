// =============================================
// EXERCISE 05: Railway Booking System
// =============================================
//
// Build a railway booking system using enums, pattern matching, Option, and Result.
// Complete all the TODOs so the program compiles and produces the expected output.
//
// Run with: cargo run
//
// Expected output:
//
//   === RAILWAY BOOKING SYSTEM ===
//
//   --- All Bookings ---
//   [✅] Booking #1001: Giridhar
//        Rajdhani Express | MAS → SBC | 350 km
//        Class: First AC | Fare: ₹1575.00
//        Status: Confirmed — Coach A1, Seat 23
//
//   [⏳] Booking #1002: Priya
//        Shatabdi Express | SBC → MAS | 350 km
//        Class: Second AC | Fare: ₹962.50
//        Status: Waitlisted — Position 3
//
//   [🔄] Booking #1003: Arjun
//        Duronto Express | MAS → NDLS | 2200 km
//        Class: Third AC | Fare: ₹3960.00
//        Status: RAC — Seat 42
//
//   [✅] Booking #1004: Meera
//        Garib Rath | MAS → HWH | 1650 km
//        Class: Sleeper | Fare: ₹1237.50
//        Status: Confirmed — Coach S4, Seat 15
//
//   [⏳] Booking #1005: Ravi
//        Chennai Local | MAS → TBM | 25 km
//        Class: General | Fare: ₹11.25
//        Status: Waitlisted — Position 7
//
//   --- Find Booking ---
//   Found: Booking #1003 for Arjun
//   Booking #9999 not found
//
//   --- Quick Lookup (if let) ---
//   Giridhar's booking: First AC class on Rajdhani Express
//
//   --- Parse Coach Class ---
//     '1A' → First AC (₹4.50/km)
//     'SL' → Sleeper (₹0.75/km)
//     'general' → General (₹0.45/km)
//     '2A' → Second AC (₹2.75/km)
//     'XYZ' → Error: Unknown coach class: XYZ
//     'first' → First AC (₹4.50/km)
//
//   --- Booking Updates ---
//   Updated booking #1002:
//   [✅] Booking #1002: Priya
//        Shatabdi Express | SBC → MAS | 350 km
//        Class: Second AC | Fare: ₹962.50
//        Status: Confirmed — Coach B2, Seat 17
//
//   Updated booking #1005:
//   [❌] Booking #1005: Ravi
//        Chennai Local | MAS → TBM | 25 km
//        Class: General | Fare: ₹11.25
//        Status: Cancelled — Refund ₹9.56
//
//   --- Booking Summary ---
//   Total: 5
//   Confirmed: 3
//   Waitlisted: 0
//   RAC: 1
//   Cancelled: 1

// =============================================
// TODO 1: Define the `CoachClass` enum
// =============================================
//
// Variants (no associated data):
//   FirstAC, SecondAC, ThirdAC, Sleeper, General
//
// Derive: Debug, Clone, PartialEq

// =============================================
// TODO 2: Implement methods on `CoachClass`
// =============================================
//
// - fn price_per_km(&self) -> f64
//   Use `match` on self to return the rate per km:
//     FirstAC: 4.50, SecondAC: 2.75, ThirdAC: 1.80, Sleeper: 0.75, General: 0.45
//
// - fn display_name(&self) -> &str
//   Use `match` to return a human-readable name:
//     "First AC", "Second AC", "Third AC", "Sleeper", "General"

// =============================================
// TODO 3: Define the `BookingStatus` enum
// =============================================
//
// This enum has DATA attached to its variants!
//
// Variants:
//   - Confirmed { coach: String, seat: u32 }    ← struct-like variant
//   - Waitlisted(u32)                            ← tuple-like (waitlist position)
//   - Cancelled { refund_amount: f64 }           ← struct-like
//   - RAC(u32)                                   ← tuple-like (seat number)
//
// Derive: Debug, Clone

// =============================================
// TODO 4: Implement methods on `BookingStatus`
// =============================================
//
// - fn description(&self) -> String
//   Use `match` to return a formatted description:
//     Confirmed { coach, seat }    → "Confirmed — Coach {coach}, Seat {seat}"
//     Waitlisted(pos)              → "Waitlisted — Position {pos}"
//     Cancelled { refund_amount }  → "Cancelled — Refund ₹{refund_amount:.2}"
//     RAC(seat)                    → "RAC — Seat {seat}"
//
//   Hint: use format!() to build the String. For the dash, use —
//
// - fn is_confirmed(&self) -> bool
//   Returns true only for the Confirmed variant.
//   Hint: you can use matches!(self, BookingStatus::Confirmed { .. })
//   Or just use match and return true/false.
//
// - fn emoji(&self) -> &str
//   Confirmed → "✅"
//   Waitlisted → "⏳"
//   Cancelled → "❌"
//   RAC → "🔄"

// =============================================
// TODO 5: Define the `Booking` struct
// =============================================
//
// Fields:
//   - id: u32
//   - passenger: String
//   - train_name: String
//   - from: String
//   - to: String
//   - distance_km: f64
//   - class: CoachClass
//   - status: BookingStatus
//
// Derive: Debug

// =============================================
// TODO 6: Implement methods on `Booking`
// =============================================
//
// Associated function:
//   - fn new(id: u32, passenger: &str, train_name: &str, from: &str, to: &str,
//            distance_km: f64, class: CoachClass, status: BookingStatus) -> Self
//     Convert all &str params to String using .to_string()
//
// Methods:
//   - fn fare(&self) -> f64
//     Returns distance_km * class.price_per_km()
//
//   - fn display(&self)
//     Prints 4 lines:
//       [✅] Booking #1001: Giridhar
//            Rajdhani Express | MAS → SBC | 350 km
//            Class: First AC | Fare: ₹1575.00
//            Status: Confirmed — Coach A1, Seat 23
//
//     Line 1: [{emoji}] Booking #{id}: {passenger}
//     Line 2: (5 spaces){train_name} | {from} → {to} | {distance_km as u64} km
//     Line 3: (5 spaces)Class: {class.display_name()} | Fare: ₹{fare:.2}
//     Line 4: (5 spaces)Status: {status.description()}
//
//   - fn cancel(&mut self)
//     Calculate refund as fare() * 0.85
//     Set status to BookingStatus::Cancelled { refund_amount: refund }
//
//   - fn confirm(&mut self, coach: &str, seat: u32)
//     Set status to BookingStatus::Confirmed { coach: coach.to_string(), seat }

// =============================================
// TODO 7: Implement `find_booking`
// =============================================
//
// fn find_booking(bookings: &[Booking], id: u32) -> Option<&Booking>
//
// Search through the slice of bookings and return Some(&booking) if found,
// or None if no booking matches the id.
//
// You can use a for loop:
//   for booking in bookings {
//       if booking.id == id {
//           return Some(booking);
//       }
//   }
//   None
//
// Or the iterator method: bookings.iter().find(|b| b.id == id)

// =============================================
// TODO 8: Implement `parse_coach_class`
// =============================================
//
// fn parse_coach_class(input: &str) -> Result<CoachClass, String>
//
// Parse a string into a CoachClass. Case-insensitive (use .to_lowercase()).
//   "1a" or "first"   → Ok(CoachClass::FirstAC)
//   "2a" or "second"  → Ok(CoachClass::SecondAC)
//   "3a" or "third"   → Ok(CoachClass::ThirdAC)
//   "sl" or "sleeper" → Ok(CoachClass::Sleeper)
//   "gn" or "general" → Ok(CoachClass::General)
//   anything else      → Err(format!("Unknown coach class: {input}"))
//
// Hint: match on input.to_lowercase().as_str() { ... }
// The .as_str() is needed because .to_lowercase() returns a String,
// and you can't match on String directly — only &str.

// =============================================
// TODO 9: Implement `summarize_bookings`
// =============================================
//
// fn summarize_bookings(bookings: &[Booking])
//
// Loop through all bookings, count how many are in each status category.
// Use `match` on &booking.status (borrow the status to avoid moving it).
//
// Print:
//   --- Booking Summary ---
//   Total: 5
//   Confirmed: 3
//   Waitlisted: 0
//   RAC: 1
//   Cancelled: 1
//
// Hint: Use `..` to ignore fields in struct-like variants:
//   BookingStatus::Confirmed { .. } => confirmed += 1,

// =============================================
// main() — DO NOT EDIT BELOW THIS LINE
// =============================================

fn main() {
    println!("=== RAILWAY BOOKING SYSTEM ===");

    // --- Create bookings ---
    println!("\n--- All Bookings ---");
    let mut bookings = vec![
        Booking::new(
            1001,
            "Giridhar",
            "Rajdhani Express",
            "MAS",
            "SBC",
            350.0,
            CoachClass::FirstAC,
            BookingStatus::Confirmed {
                coach: String::from("A1"),
                seat: 23,
            },
        ),
        Booking::new(
            1002,
            "Priya",
            "Shatabdi Express",
            "SBC",
            "MAS",
            350.0,
            CoachClass::SecondAC,
            BookingStatus::Waitlisted(3),
        ),
        Booking::new(
            1003,
            "Arjun",
            "Duronto Express",
            "MAS",
            "NDLS",
            2200.0,
            CoachClass::ThirdAC,
            BookingStatus::RAC(42),
        ),
        Booking::new(
            1004,
            "Meera",
            "Garib Rath",
            "MAS",
            "HWH",
            1650.0,
            CoachClass::Sleeper,
            BookingStatus::Confirmed {
                coach: String::from("S4"),
                seat: 15,
            },
        ),
        Booking::new(
            1005,
            "Ravi",
            "Chennai Local",
            "MAS",
            "TBM",
            25.0,
            CoachClass::General,
            BookingStatus::Waitlisted(7),
        ),
    ];

    for booking in &bookings {
        booking.display();
        println!();
    }

    // --- Find a booking using Option ---
    println!("--- Find Booking ---");
    match find_booking(&bookings, 1003) {
        Some(b) => println!("Found: Booking #{} for {}", b.id, b.passenger),
        None => println!("Booking not found"),
    }
    match find_booking(&bookings, 9999) {
        Some(b) => println!("Found: Booking #{} for {}", b.id, b.passenger),
        None => println!("Booking #9999 not found"),
    }

    // --- if let ---
    println!("\n--- Quick Lookup (if let) ---");
    if let Some(booking) = find_booking(&bookings, 1001) {
        println!(
            "Giridhar's booking: {} class on {}",
            booking.class.display_name(),
            booking.train_name
        );
    }

    // --- Parse coach class using Result ---
    println!("\n--- Parse Coach Class ---");
    let inputs = ["1A", "SL", "general", "2A", "XYZ", "first"];
    for input in inputs {
        match parse_coach_class(input) {
            Ok(class) => println!(
                "  '{}' → {} (₹{:.2}/km)",
                input,
                class.display_name(),
                class.price_per_km()
            ),
            Err(e) => println!("  '{input}' → Error: {e}"),
        }
    }

    // --- Modify bookings ---
    println!("\n--- Booking Updates ---");

    // Confirm waitlisted booking
    bookings[1].confirm("B2", 17);
    println!("Updated booking #1002:");
    bookings[1].display();

    // Cancel a booking
    println!();
    bookings[4].cancel();
    println!("Updated booking #1005:");
    bookings[4].display();

    // --- Summary ---
    println!();
    summarize_bookings(&bookings);
}
