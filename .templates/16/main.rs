// =============================================================================
// Exercise: Train Lookup Service
// =============================================================================
//
// Build a train lookup service with typed error handling using thiserror.
// Each step builds on the previous one.
//
// Expected output:
//
//   === TRAIN LOOKUP SERVICE ===
//
//   --- Successful Lookups ---
//   Found: #12001 Rajdhani Express (130 km/h)
//   Found: #12007 Shatabdi Express (150 km/h)
//
//   --- Error Cases ---
//   Error: Train #99999 not found
//   Error: Train name cannot be empty
//   Error: Speed must be positive, got -50
//
//   --- Parse from JSON ---
//   Parsed: #12001 Rajdhani Express
//   Parse error: JSON error: missing field `name`
//
//   --- Batch Lookup ---
//     ✅ #12001 Rajdhani Express
//     ❌ Train #99999 not found
//     ✅ #12007 Shatabdi Express
//     ❌ Train #55555 not found
//   Results: 2 found, 2 errors
//
//
// STEP 1: Define `TrainError` enum using thiserror
// -------------------------------------------------
// Use #[derive(Debug, thiserror::Error)] and add these variants:
//
//   - NotFound(u32)
//     Error message: "Train #{0} not found"
//
//   - EmptyName
//     Error message: "Train name cannot be empty"
//
//   - InvalidSpeed(i32)
//     Error message: "Speed must be positive, got {0}"
//
//   - Json(serde_json::Error)
//     Error message: "JSON error: {0}"
//     Use #[from] so serde_json::Error converts automatically with ?
//
//
// STEP 2: Define `Train` struct
// ------------------------------
// Fields: number (u32), name (String), speed_kmh (u32)
// Derive: Debug, Clone, serde::Deserialize
//
// Implement:
//   - Train::new(number, name, speed_kmh) -> Train
//   - Train::display(&self) -> String
//     Returns: "#{number} {name} ({speed_kmh} km/h)"
//
//
// STEP 3: Implement `validate_train`
// -----------------------------------
// fn validate_train(name: &str, speed: i32) -> Result<(), TrainError>
//
//   - If name (after trimming) is empty → Err(TrainError::EmptyName)
//   - If speed <= 0 → Err(TrainError::InvalidSpeed(speed))
//   - Otherwise → Ok(())
//
//
// STEP 4: Implement `find_train`
// -------------------------------
// fn find_train(trains: &[Train], number: u32) -> Result<&Train, TrainError>
//
// Search through the slice for a train with matching number.
// Return TrainError::NotFound if not found.
//
//
// STEP 5: Implement `parse_train_json`
// --------------------------------------
// fn parse_train_json(json: &str) -> Result<Train, TrainError>
//
// Use serde_json::from_str(json) with the ? operator.
// The #[from] attribute on the Json variant makes the conversion automatic.
//
//
// STEP 6: Write main()
// ---------------------
// - Create a vec of 3 trains: Rajdhani Express (#12001, 130 km/h),
//   Duronto Express (#12004, 120 km/h), Shatabdi Express (#12007, 150 km/h)
// - Do successful lookups for #12001 and #12007
// - Test each error case: not found (#99999), empty name, invalid speed (-50)
// - Parse valid JSON and invalid JSON (missing "name" field)
// - Batch lookup: try [12001, 99999, 12007, 55555], count successes and errors

fn main() {}
