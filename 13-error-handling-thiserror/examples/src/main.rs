use serde::Deserialize;
use thiserror::Error;

// =============================================================================
// Example 1: Basic thiserror enum — Railway Signal Errors
// =============================================================================

#[derive(Debug, Error)]
enum SignalError {
    #[error("Signal '{name}' is stuck on {state}")]
    Stuck { name: String, state: String },

    #[error("Signal at km {0} has no power")]
    NoPower(f64),

    #[error("Unknown signal code: {0:#04x}")]
    UnknownCode(u8),

    #[error("Signal system offline")]
    Offline,
}

fn check_signal(code: u8) -> Result<String, SignalError> {
    match code {
        0x01 => Ok("green".to_string()),
        0x02 => Ok("yellow".to_string()),
        0x03 => Ok("red".to_string()),
        0x00 => Err(SignalError::Offline),
        other => Err(SignalError::UnknownCode(other)),
    }
}

fn demo_basic_error() {
    println!("=== Example 1: Basic thiserror ===\n");

    let codes = [0x01, 0x03, 0x00, 0xFF];
    for code in codes {
        match check_signal(code) {
            Ok(color) => println!("  Signal code {code:#04x} → {color}"),
            Err(e) => println!("  Signal code {code:#04x} → ERROR: {e}"),
        }
    }

    // Demonstrating named-field variants
    let stuck = SignalError::Stuck {
        name: "North Junction A".to_string(),
        state: "red".to_string(),
    };
    println!("  Stuck signal: {stuck}");

    let no_power = SignalError::NoPower(42.7);
    println!("  No power: {no_power}");
    println!();
}

// =============================================================================
// Example 2: #[from] for automatic conversion
// =============================================================================

#[derive(Debug, Error)]
enum TimetableError {
    #[error("Failed to read timetable file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse timetable JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Timetable is empty — no trains scheduled")]
    Empty,

    #[error("Train #{0} appears more than once")]
    DuplicateTrain(u32),
}

#[derive(Debug, Deserialize)]
struct RawTimetable {
    trains: Vec<RawTrain>,
}

#[derive(Debug, Deserialize)]
struct RawTrain {
    number: u32,
    destination: String,
}

/// Demonstrates ? converting different error types automatically via #[from]
fn parse_timetable(json_str: &str) -> Result<Vec<RawTrain>, TimetableError> {
    // serde_json::Error is auto-converted to TimetableError::Json via #[from]
    let timetable: RawTimetable = serde_json::from_str(json_str)?;

    if timetable.trains.is_empty() {
        return Err(TimetableError::Empty);
    }

    Ok(timetable.trains)
}

fn demo_from_conversion() {
    println!("=== Example 2: #[from] conversion ===\n");

    // Success case
    let good_json = r#"{"trains": [{"number": 100, "destination": "Edinburgh"}]}"#;
    match parse_timetable(good_json) {
        Ok(trains) => println!("  Parsed {} train(s): {:?}", trains.len(), trains),
        Err(e) => println!("  Error: {e}"),
    }

    // JSON parse error — serde_json::Error converted via #[from]
    let bad_json = r#"{"trains": broken}"#;
    match parse_timetable(bad_json) {
        Ok(_) => println!("  Unexpected success"),
        Err(e) => println!("  Bad JSON error: {e}"),
    }

    // Empty timetable — custom variant
    let empty_json = r#"{"trains": []}"#;
    match parse_timetable(empty_json) {
        Ok(_) => println!("  Unexpected success"),
        Err(e) => println!("  Empty error: {e}"),
    }

    println!();
}

// =============================================================================
// Example 3: #[source] for error chaining with context
// =============================================================================

#[derive(Debug, Error)]
enum ScheduleError {
    #[error("Failed to load schedule for route '{route}'")]
    LoadFailed {
        route: String,
        #[source]
        cause: TimetableError,
    },

    #[error("Schedule validation failed for route '{0}'")]
    ValidationFailed(String),
}

fn load_route_schedule(route: &str, json: &str) -> Result<Vec<RawTrain>, ScheduleError> {
    // We can't use ? here because #[source] doesn't generate From.
    // Instead, we map the error manually to add context.
    let trains = parse_timetable(json).map_err(|cause| ScheduleError::LoadFailed {
        route: route.to_string(),
        cause,
    })?;

    Ok(trains)
}

fn print_error_chain(err: &dyn std::error::Error) {
    println!("  Error: {err}");
    let mut source = err.source();
    let mut depth = 1;
    while let Some(cause) = source {
        println!("  {: >width$} Caused by: {cause}", "", width = depth * 2);
        source = cause.source();
        depth += 1;
    }
}

fn demo_error_chaining() {
    println!("=== Example 3: Error chaining with #[source] ===\n");

    let bad_json = r#"not json at all"#;
    match load_route_schedule("Edinburgh Express", bad_json) {
        Ok(_) => println!("  Unexpected success"),
        Err(ref e) => print_error_chain(e),
    }

    println!();
}

// =============================================================================
// Example 4: Composing multiple fallible operations with ?
// =============================================================================

#[derive(Debug, Error)]
enum BookingError {
    #[error("Train #{0} not found in schedule")]
    TrainNotFound(u32),

    #[error("Insufficient seats: need {requested}, have {available}")]
    InsufficientSeats { requested: u32, available: u32 },

    #[error("Invalid passenger name: '{0}'")]
    InvalidName(String),

    #[error("Payment declined: {0}")]
    PaymentDeclined(String),
}

#[derive(Debug)]
struct Booking {
    train: u32,
    passenger: String,
    seat: u32,
    confirmation: String,
}

fn find_available_seats(train_number: u32) -> Result<u32, BookingError> {
    match train_number {
        100 => Ok(45),
        200 => Ok(0),
        300 => Ok(12),
        _ => Err(BookingError::TrainNotFound(train_number)),
    }
}

fn validate_passenger(name: &str) -> Result<(), BookingError> {
    if name.trim().is_empty() {
        return Err(BookingError::InvalidName(name.to_string()));
    }
    if name.len() < 2 {
        return Err(BookingError::InvalidName(name.to_string()));
    }
    Ok(())
}

fn charge_payment(amount: f64) -> Result<String, BookingError> {
    if amount > 500.0 {
        return Err(BookingError::PaymentDeclined(
            "Amount exceeds single transaction limit".to_string(),
        ));
    }
    Ok(format!("CONF-{}", (amount * 100.0) as u64))
}

/// Chains multiple fallible operations with ? — the railway pattern
fn book_seat(train: u32, passenger: &str, price: f64) -> Result<Booking, BookingError> {
    let available = find_available_seats(train)?;

    if available == 0 {
        return Err(BookingError::InsufficientSeats {
            requested: 1,
            available: 0,
        });
    }

    validate_passenger(passenger)?;

    let confirmation = charge_payment(price)?;

    Ok(Booking {
        train,
        passenger: passenger.to_string(),
        seat: available, // assign last available
        confirmation,
    })
}

fn demo_composition() {
    println!("=== Example 4: Composing with ? ===\n");

    let test_cases = vec![
        (100, "Alice Smith", 45.0),   // success
        (999, "Bob Jones", 30.0),     // train not found
        (200, "Charlie Day", 25.0),   // no seats
        (300, "", 50.0),              // invalid name
        (100, "Diana Prince", 999.0), // payment declined
    ];

    for (train, name, price) in test_cases {
        match book_seat(train, name, price) {
            Ok(booking) => println!("  ✓ Booked: {:?}", booking),
            Err(e) => println!("  ✗ Failed (train={train}, name=\"{name}\"): {e}"),
        }
    }

    println!();
}

// =============================================================================
// Example 5: Matching on error variants for recovery
// =============================================================================

fn demo_matching() {
    println!("=== Example 5: Matching on errors for recovery ===\n");

    let result = book_seat(999, "Eve Torres", 50.0);

    match result {
        Ok(booking) => println!("  Booked seat {} on train {}", booking.seat, booking.train),
        Err(BookingError::TrainNotFound(n)) => {
            println!("  Train #{n} not found — suggesting alternatives...");
            println!("  → Try train #100 (Edinburgh Express)");
            println!("  → Try train #300 (Glasgow Shuttle)");
        }
        Err(BookingError::InsufficientSeats { available, .. }) => {
            println!("  No seats! Only {available} remaining. Added to waitlist.");
        }
        Err(BookingError::InvalidName(name)) => {
            println!("  Bad name '{name}' — please re-enter your details.");
        }
        Err(BookingError::PaymentDeclined(reason)) => {
            println!("  Payment issue: {reason}. Try another card?");
        }
    }

    println!();
}

// =============================================================================
// Main — run all examples
// =============================================================================

fn main() {
    demo_basic_error();
    demo_from_conversion();
    demo_error_chaining();
    demo_composition();
    demo_matching();
}
