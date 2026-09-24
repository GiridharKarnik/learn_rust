use serde::Deserialize;
use thiserror::Error;

// =============================================================================
// STEP 1: TrainError enum
// =============================================================================

#[derive(Debug, Error)]
enum TrainError {
    #[error("Train #{0} not found")]
    NotFound(u32),

    #[error("Train name cannot be empty")]
    EmptyName,

    #[error("Speed must be positive, got {0}")]
    InvalidSpeed(i32),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

// =============================================================================
// STEP 2: Train struct
// =============================================================================

#[derive(Debug, Clone, Deserialize)]
struct Train {
    number: u32,
    name: String,
    speed_kmh: u32,
}

impl Train {
    fn new(number: u32, name: &str, speed_kmh: u32) -> Train {
        Train {
            number,
            name: name.to_string(),
            speed_kmh,
        }
    }

    fn display(&self) -> String {
        format!("#{} {} ({} km/h)", self.number, self.name, self.speed_kmh)
    }
}

// =============================================================================
// STEP 3: validate_train
// =============================================================================

fn validate_train(name: &str, speed: i32) -> Result<(), TrainError> {
    if name.trim().is_empty() {
        return Err(TrainError::EmptyName);
    }

    if speed <= 0 {
        return Err(TrainError::InvalidSpeed(speed));
    }

    Ok(())
}

// =============================================================================
// STEP 4: find_train
// =============================================================================

fn find_train(trains: &[Train], number: u32) -> Result<&Train, TrainError> {
    trains
        .iter()
        .find(|t| t.number == number)
        .ok_or(TrainError::NotFound(number))
}

// =============================================================================
// STEP 5: parse_train_json — uses #[from] for automatic conversion
// =============================================================================

fn parse_train_json(json: &str) -> Result<Train, TrainError> {
    let train: Train = serde_json::from_str(json)?;
    Ok(train)
}

// =============================================================================
// STEP 6: main
// =============================================================================

fn main() {
    let trains = vec![
        Train::new(12001, "Rajdhani Express", 130),
        Train::new(12004, "Duronto Express", 120),
        Train::new(12007, "Shatabdi Express", 150),
    ];

    println!("=== TRAIN LOOKUP SERVICE ===");

    // --- Successful Lookups ---
    println!("\n--- Successful Lookups ---");
    match find_train(&trains, 12001) {
        Ok(train) => println!("Found: {}", train.display()),
        Err(e) => println!("Error: {e}"),
    }
    match find_train(&trains, 12007) {
        Ok(train) => println!("Found: {}", train.display()),
        Err(e) => println!("Error: {e}"),
    }

    // --- Error Cases ---
    println!("\n--- Error Cases ---");
    match find_train(&trains, 99999) {
        Ok(train) => println!("Found: {}", train.display()),
        Err(e) => println!("Error: {e}"),
    }
    match validate_train("", 100) {
        Ok(()) => println!("Valid"),
        Err(e) => println!("Error: {e}"),
    }
    match validate_train("Express", -50) {
        Ok(()) => println!("Valid"),
        Err(e) => println!("Error: {e}"),
    }

    // --- Parse from JSON ---
    println!("\n--- Parse from JSON ---");
    let good_json = r#"{"number": 12001, "name": "Rajdhani Express", "speed_kmh": 130}"#;
    match parse_train_json(good_json) {
        Ok(train) => println!("Parsed: #{} {}", train.number, train.name),
        Err(e) => println!("Parse error: {e}"),
    }

    let bad_json = r#"{"number": 12001, "speed_kmh": 130}"#;
    match parse_train_json(bad_json) {
        Ok(train) => println!("Parsed: #{} {}", train.number, train.name),
        Err(e) => println!("Parse error: {e}"),
    }

    // --- Batch Lookup ---
    println!("\n--- Batch Lookup ---");
    let lookup_ids = [12001, 99999, 12007, 55555];
    let mut found = 0;
    let mut errors = 0;

    for &id in &lookup_ids {
        match find_train(&trains, id) {
            Ok(train) => {
                println!("  ✅ #{} {}", train.number, train.name);
                found += 1;
            }
            Err(e) => {
                println!("  ❌ {e}");
                errors += 1;
            }
        }
    }

    println!("Results: {found} found, {errors} errors");
}
