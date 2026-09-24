// Lesson 12: JSON Handling with Serde — Railway API Data Handler (SOLUTION)

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// ---------------------------------------------------------------------------
// STEP 1: Define TrainStatus enum
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
enum TrainStatus {
    OnTime,
    Delayed { minutes: u32 },
    Cancelled { reason: String },
}

// ---------------------------------------------------------------------------
// STEP 2: Define Train struct
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Train {
    number: u32,
    name: String,
    from: String,
    to: String,
    status: TrainStatus,
    speed_kmh: Option<f64>,
}

// ---------------------------------------------------------------------------
// STEP 3: serialize_trains
// ---------------------------------------------------------------------------

fn serialize_trains(trains: &[Train]) -> Result<String, String> {
    serde_json::to_string_pretty(trains).map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// STEP 4: deserialize_trains
// ---------------------------------------------------------------------------

fn deserialize_trains(json: &str) -> Result<Vec<Train>, String> {
    serde_json::from_str(json).map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// STEP 5: find_delayed
// ---------------------------------------------------------------------------

fn find_delayed(trains: &[Train]) -> Vec<&Train> {
    trains
        .iter()
        .filter(|t| matches!(t.status, TrainStatus::Delayed { .. }))
        .collect()
}

// ---------------------------------------------------------------------------
// STEP 6: to_summary_json
// ---------------------------------------------------------------------------

fn to_summary_json(trains: &[Train]) -> Result<String, String> {
    let total = trains.len();
    let on_time = trains
        .iter()
        .filter(|t| matches!(t.status, TrainStatus::OnTime))
        .count();
    let delayed = trains
        .iter()
        .filter(|t| matches!(t.status, TrainStatus::Delayed { .. }))
        .count();
    let cancelled = trains
        .iter()
        .filter(|t| matches!(t.status, TrainStatus::Cancelled { .. }))
        .count();

    let mut summary = BTreeMap::new();
    summary.insert("total", total);
    summary.insert("on_time", on_time);
    summary.insert("delayed", delayed);
    summary.insert("cancelled", cancelled);

    serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// STEP 7: merge_json_trains
// ---------------------------------------------------------------------------

fn merge_json_trains(json1: &str, json2: &str) -> Result<String, String> {
    let mut trains1: Vec<Train> = deserialize_trains(json1)?;
    let trains2: Vec<Train> = deserialize_trains(json2)?;
    trains1.extend(trains2);
    serialize_trains(&trains1)
}

// ---------------------------------------------------------------------------
// STEP 8: main
// ---------------------------------------------------------------------------

fn main() {
    println!("=== RAILWAY JSON DATA HANDLER ===");

    // -- Create sample trains --
    let trains = vec![
        Train {
            number: 12001,
            name: "Rajdhani Express".to_string(),
            from: "Chennai".to_string(),
            to: "New Delhi".to_string(),
            status: TrainStatus::OnTime,
            speed_kmh: Some(130.0),
        },
        Train {
            number: 12007,
            name: "Shatabdi Express".to_string(),
            from: "Chennai".to_string(),
            to: "Bangalore".to_string(),
            status: TrainStatus::Delayed { minutes: 45 },
            speed_kmh: Some(150.0),
        },
        Train {
            number: 12245,
            name: "Duronto Express".to_string(),
            from: "Chennai".to_string(),
            to: "Mumbai".to_string(),
            status: TrainStatus::OnTime,
            speed_kmh: None,
        },
        Train {
            number: 22501,
            name: "Tejas Express".to_string(),
            from: "Chennai".to_string(),
            to: "Madurai".to_string(),
            status: TrainStatus::Cancelled {
                reason: "Track maintenance".to_string(),
            },
            speed_kmh: None,
        },
    ];

    // -- Serialize --
    println!("\n--- Serialized JSON ---");
    let json = serialize_trains(&trains).expect("Failed to serialize");
    println!("{}", json);

    // -- Deserialize --
    println!("\n--- Deserialized Trains ---");
    let parsed = deserialize_trains(&json).expect("Failed to deserialize");
    for train in &parsed {
        println!("  #{} {} ({:?})", train.number, train.name, train.status);
    }

    // -- Find delayed --
    println!("\n--- Delayed Trains ---");
    let delayed = find_delayed(&parsed);
    for train in &delayed {
        if let TrainStatus::Delayed { minutes } = &train.status {
            println!("  {} — delayed by {} minutes", train.name, minutes);
        }
    }

    // -- Summary --
    println!("\n--- Summary ---");
    let summary = to_summary_json(&parsed).expect("Failed to create summary");
    println!("{}", summary);

    // -- Invalid JSON --
    println!("\n--- Invalid JSON Test ---");
    let bad_json = r#"{"not valid"#;
    match deserialize_trains(bad_json) {
        Ok(_) => println!("  Unexpected success!"),
        Err(e) => println!("  Error (expected): {}", e),
    }

    // -- Merge --
    println!("\n--- Merge Two JSON Arrays ---");
    let batch_a = serialize_trains(&trains[..2]).expect("serialize failed");
    let batch_b = serialize_trains(&trains[2..]).expect("serialize failed");
    let merged = merge_json_trains(&batch_a, &batch_b).expect("Failed to merge");
    let merged_trains: Vec<Train> = serde_json::from_str(&merged).unwrap();
    println!(
        "  Merged {} trains from two JSON arrays:",
        merged_trains.len()
    );
    for train in &merged_trains {
        println!("  #{} {}", train.number, train.name);
    }
}
