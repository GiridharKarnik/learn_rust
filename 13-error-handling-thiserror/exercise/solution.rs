use serde::Deserialize;
use thiserror::Error;

// =============================================================================
// STEP 1: BookingError enum
// =============================================================================

#[derive(Debug, Error)]
enum BookingError {
    #[error("Train #{0} not found")]
    TrainNotFound(u32),

    #[error("No seats available in {0:?} class")]
    NoSeatsAvailable(CoachClass),

    #[error("Invalid passenger name: {0}")]
    InvalidPassenger(String),

    #[error("Payment failed: {0}")]
    PaymentFailed(String),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

// =============================================================================
// STEP 2: Supporting types
// =============================================================================

#[derive(Debug, Clone, Deserialize)]
enum CoachClass {
    Standard,
    FirstClass,
    Sleeper,
}

struct Train {
    number: u32,
    name: String,
    standard_seats: u32,
    first_class_seats: u32,
    sleeper_seats: u32,
    price_per_seat: f64,
}

#[derive(Debug)]
struct Booking {
    train_number: u32,
    passenger: String,
    class: CoachClass,
    confirmation: String,
}

// =============================================================================
// STEP 3: find_train
// =============================================================================

fn find_train<'a>(trains: &'a [Train], number: u32) -> Result<&'a Train, BookingError> {
    trains
        .iter()
        .find(|t| t.number == number)
        .ok_or(BookingError::TrainNotFound(number))
}

// =============================================================================
// STEP 4: validate_booking
// =============================================================================

fn validate_booking(
    name: &str,
    class: &CoachClass,
    available_seats: u32,
) -> Result<(), BookingError> {
    let trimmed = name.trim();

    if trimmed.is_empty() {
        return Err(BookingError::InvalidPassenger(name.to_string()));
    }

    if trimmed.len() < 2 {
        return Err(BookingError::InvalidPassenger(name.to_string()));
    }

    if available_seats == 0 {
        return Err(BookingError::NoSeatsAvailable(class.clone()));
    }

    Ok(())
}

// =============================================================================
// STEP 5: process_payment
// =============================================================================

fn process_payment(amount: f64) -> Result<String, BookingError> {
    if amount <= 0.0 {
        return Err(BookingError::PaymentFailed(
            "Amount must be positive".to_string(),
        ));
    }

    if amount > 1000.0 {
        return Err(BookingError::PaymentFailed(
            "Amount exceeds limit".to_string(),
        ));
    }

    let pence = (amount * 100.0) as u64;
    Ok(format!("PAY-{pence}"))
}

// =============================================================================
// STEP 6: book_ticket — chains everything with ?
// =============================================================================

fn book_ticket(
    trains: &[Train],
    train_number: u32,
    passenger: &str,
    class: CoachClass,
) -> Result<Booking, BookingError> {
    let train = find_train(trains, train_number)?;

    let available = match &class {
        CoachClass::Standard => train.standard_seats,
        CoachClass::FirstClass => train.first_class_seats,
        CoachClass::Sleeper => train.sleeper_seats,
    };

    validate_booking(passenger, &class, available)?;

    let confirmation = process_payment(train.price_per_seat)?;

    Ok(Booking {
        train_number: train.number,
        passenger: passenger.to_string(),
        class,
        confirmation,
    })
}

// =============================================================================
// STEP 7: parse_booking_request — uses #[from] for JSON errors
// =============================================================================

#[derive(Deserialize)]
struct BookingRequest {
    train_number: u32,
    passenger: String,
    class: CoachClass,
}

fn parse_booking_request(json: &str) -> Result<(u32, String, CoachClass), BookingError> {
    // serde_json::Error automatically converts to BookingError::JsonError via #[from]
    let request: BookingRequest = serde_json::from_str(json)?;
    Ok((request.train_number, request.passenger, request.class))
}

// =============================================================================
// STEP 8: main — test all success and error paths
// =============================================================================

fn main() {
    let trains = vec![
        Train {
            number: 100,
            name: "Highland Express".to_string(),
            standard_seats: 120,
            first_class_seats: 30,
            sleeper_seats: 15,
            price_per_seat: 45.50,
        },
        Train {
            number: 200,
            name: "Coastal Runner".to_string(),
            standard_seats: 80,
            first_class_seats: 0, // sold out!
            sleeper_seats: 10,
            price_per_seat: 35.00,
        },
        Train {
            number: 300,
            name: "Border Limited".to_string(),
            standard_seats: 60,
            first_class_seats: 20,
            sleeper_seats: 8,
            price_per_seat: 1500.00, // very expensive — will fail payment
        },
    ];

    println!("=== Railway Booking Error System ===\n");

    // (a) Successful booking
    print!("Booking standard on #100: ");
    match book_ticket(&trains, 100, "Alice Thompson", CoachClass::Standard) {
        Ok(booking) => println!("✓ {:?}", booking),
        Err(e) => println!("✗ {e}"),
    }

    // (b) TrainNotFound
    print!("Booking on #999: ");
    match book_ticket(&trains, 999, "Bob Smith", CoachClass::Standard) {
        Ok(booking) => println!("✓ {:?}", booking),
        Err(BookingError::TrainNotFound(n)) => println!("✗ Train #{n} doesn't exist"),
        Err(e) => println!("✗ {e}"),
    }

    // (c) NoSeatsAvailable
    print!("Booking first class on #200: ");
    match book_ticket(&trains, 200, "Charlie Davis", CoachClass::FirstClass) {
        Ok(booking) => println!("✓ {:?}", booking),
        Err(BookingError::NoSeatsAvailable(class)) => {
            println!("✗ No {class:?} seats left — try another class")
        }
        Err(e) => println!("✗ {e}"),
    }

    // (d) InvalidPassenger
    print!("Booking with empty name: ");
    match book_ticket(&trains, 100, "  ", CoachClass::Standard) {
        Ok(booking) => println!("✓ {:?}", booking),
        Err(BookingError::InvalidPassenger(name)) => {
            println!("✗ Invalid name '{name}' — please provide your full name")
        }
        Err(e) => println!("✗ {e}"),
    }

    // (e) PaymentFailed (price_per_seat > 1000)
    print!("Booking on expensive #300: ");
    match book_ticket(&trains, 300, "Diana Prince", CoachClass::Standard) {
        Ok(booking) => println!("✓ {:?}", booking),
        Err(BookingError::PaymentFailed(reason)) => {
            println!("✗ Payment issue: {reason}")
        }
        Err(e) => println!("✗ {e}"),
    }

    println!();

    // (f) Successful JSON parse
    let good_json = r#"{"train_number": 100, "passenger": "Eve Torres", "class": "Standard"}"#;
    print!("Parsing valid JSON: ");
    match parse_booking_request(good_json) {
        Ok((train, passenger, class)) => {
            println!("✓ Train #{train}, {passenger}, {class:?}")
        }
        Err(e) => println!("✗ {e}"),
    }

    // (g) JSON parse error
    let bad_json = r#"{"train_number": "not a number"}"#;
    print!("Parsing bad JSON: ");
    match parse_booking_request(bad_json) {
        Ok(_) => println!("✓ Unexpected success"),
        Err(BookingError::JsonError(e)) => {
            println!("✗ JSON parse failed: {e}")
        }
        Err(e) => println!("✗ {e}"),
    }
}
