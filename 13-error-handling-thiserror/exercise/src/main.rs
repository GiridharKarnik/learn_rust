// =============================================================================
// Exercise: Railway Booking Error System
// =============================================================================
//
// Build a complete railway booking system with typed error handling using
// thiserror. Each step builds on the previous one.
//
// STEP 1: Define `BookingError` enum using thiserror
// --------------------------------------------------
// Use #[derive(Debug, thiserror::Error)] and add these variants:
//
//   - TrainNotFound(u32)
//     Error message: "Train #{0} not found"
//
//   - NoSeatsAvailable(CoachClass)
//     Error message: "No seats available in {0:?} class"
//
//   - InvalidPassenger(String)
//     Error message: "Invalid passenger name: {0}"
//
//   - PaymentFailed(String)
//     Error message: "Payment failed: {0}"
//
//   - JsonError(serde_json::Error)
//     Error message: "JSON error: {0}"
//     Use #[from] so serde_json::Error converts automatically with ?
//
//
// STEP 2: Define supporting types
// --------------------------------
// a) CoachClass enum with variants: Standard, FirstClass, Sleeper
//    - Derive: Debug, Clone, serde::Deserialize
//
// b) Train struct with fields:
//    - number: u32
//    - name: String
//    - standard_seats: u32
//    - first_class_seats: u32
//    - sleeper_seats: u32
//    - price_per_seat: f64
//
// c) Booking struct with fields:
//    - train_number: u32
//    - passenger: String
//    - class: CoachClass
//    - confirmation: String
//
//
// STEP 3: Implement find_train
// ----------------------------
// fn find_train<'a>(trains: &'a [Train], number: u32) -> Result<&'a Train, BookingError>
//
// Search through the slice for a train with matching number.
// Return BookingError::TrainNotFound if not found.
//
//
// STEP 4: Implement validate_booking
// -----------------------------------
// fn validate_booking(name: &str, class: &CoachClass, available_seats: u32)
//     -> Result<(), BookingError>
//
// Validation rules:
//   - Name must not be empty (after trimming) → InvalidPassenger
//   - Name must be at least 2 characters → InvalidPassenger
//   - Available seats must be > 0 → NoSeatsAvailable
//
//
// STEP 5: Implement process_payment
// ----------------------------------
// fn process_payment(amount: f64) -> Result<String, BookingError>
//
// Rules:
//   - Reject amounts <= 0 with PaymentFailed("Amount must be positive")
//   - Reject amounts > 1000 with PaymentFailed("Amount exceeds limit")
//   - Return a confirmation string like "PAY-{amount_in_pence}"
//     (e.g., amount 45.50 → "PAY-4550")
//
//
// STEP 6: Implement book_ticket
// ------------------------------
// fn book_ticket(
//     trains: &[Train],
//     train_number: u32,
//     passenger: &str,
//     class: CoachClass,
// ) -> Result<Booking, BookingError>
//
// This function chains everything with ?:
//   1. Find the train using find_train
//   2. Get available seats based on class:
//      - Standard → train.standard_seats
//      - FirstClass → train.first_class_seats
//      - Sleeper → train.sleeper_seats
//   3. Validate using validate_booking
//   4. Process payment using train.price_per_seat
//   5. Return a Booking struct with all the details
//
//
// STEP 7: Implement parse_booking_request
// ----------------------------------------
// fn parse_booking_request(json: &str) -> Result<(u32, String, CoachClass), BookingError>
//
// Define a helper struct BookingRequest with serde::Deserialize:
//   { "train_number": 100, "passenger": "Alice", "class": "Standard" }
//
// Parse the JSON string into BookingRequest. The serde_json::Error
// should automatically convert to BookingError::JsonError via #[from].
// Return the fields as a tuple.
//
//
// STEP 8: Write main() that tests all paths
// -------------------------------------------
// Create a vec of trains and test:
//   a) Successful booking
//   b) TrainNotFound error
//   c) NoSeatsAvailable error
//   d) InvalidPassenger error
//   e) PaymentFailed error
//   f) Successful JSON parse
//   g) JSON parse error (bad JSON)
//
// Print results with match statements showing you handle each variant.

fn main() {}
