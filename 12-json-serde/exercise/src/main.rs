// =============================================
// EXERCISE 12: Railway API Data Handler
// =============================================
//
// Build functions to serialize and deserialize railway data as JSON.
// There is NO code provided — you write everything, including main().
//
// This exercises: serde, serde_json, Serialize, Deserialize, derive macros,
// #[serde(tag)], #[serde(rename_all)], Option, enums, Vec, BTreeMap.
//
// Run with: cargo run
//
// Expected output:
//
//   === RAILWAY JSON DATA HANDLER ===
//
//   --- Serialized JSON ---
//   [
//     {
//       "number": 12001,
//       "name": "Rajdhani Express",
//       "from": "Chennai",
//       "to": "New Delhi",
//       "status": {
//         "type": "OnTime"
//       },
//       "speedKmh": 130.0
//     },
//     {
//       "number": 12007,
//       "name": "Shatabdi Express",
//       "from": "Chennai",
//       "to": "Bangalore",
//       "status": {
//         "type": "Delayed",
//         "minutes": 45
//       },
//       "speedKmh": 150.0
//     },
//     {
//       "number": 12245,
//       "name": "Duronto Express",
//       "from": "Chennai",
//       "to": "Mumbai",
//       "status": {
//         "type": "OnTime"
//       },
//       "speedKmh": null
//     },
//     {
//       "number": 22501,
//       "name": "Tejas Express",
//       "from": "Chennai",
//       "to": "Madurai",
//       "status": {
//         "type": "Cancelled",
//         "reason": "Track maintenance"
//       },
//       "speedKmh": null
//     }
//   ]
//
//   --- Deserialized Trains ---
//     #12001 Rajdhani Express (OnTime)
//     #12007 Shatabdi Express (Delayed { minutes: 45 })
//     #12245 Duronto Express (OnTime)
//     #22501 Tejas Express (Cancelled { reason: "Track maintenance" })
//
//   --- Delayed Trains ---
//     Shatabdi Express — delayed by 45 minutes
//
//   --- Summary ---
//   {
//     "cancelled": 1,
//     "delayed": 1,
//     "on_time": 2,
//     "total": 4
//   }
//
//   --- Invalid JSON Test ---
//     Error (expected): invalid type: map, expected a sequence at line 1 column 0
//
//   --- Merge Two JSON Arrays ---
//     Merged 4 trains from two JSON arrays:
//     #12001 Rajdhani Express
//     #12007 Shatabdi Express
//     #12245 Duronto Express
//     #22501 Tejas Express

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

// =============================================
// STEP 1: Define the `TrainStatus` enum
// =============================================
// Variants:
//   OnTime                          — unit variant
//   Delayed { minutes: u32 }        — struct variant (NOT tuple — needed for tagged serialization)
//   Cancelled { reason: String }    — struct variant
//
// Derive: Serialize, Deserialize, Debug, Clone
// Add:    #[serde(tag = "type")]    — internally tagged, like TS discriminated unions
//
// This makes OnTime serialize as: {"type":"OnTime"}
// and Delayed as: {"type":"Delayed","minutes":45}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
enum TrainStatus {
    OnTime,
    Delayed { minutes: u32 },
    Cancelled { reason: String },
}

// =============================================
// STEP 2: Define the `Train` struct
// =============================================
// Fields:
//   number: u32
//   name: String
//   from: String
//   to: String
//   status: TrainStatus
//   speed_kmh: Option<f64>         — None becomes null in JSON
//
// Derive: Serialize, Deserialize, Debug, Clone
// Add:    #[serde(rename_all = "camelCase")]
//         This converts snake_case field names to camelCase in JSON.
//         So speed_kmh becomes "speedKmh" in JSON output.
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

// =============================================
// STEP 3: Implement `serialize_trains`
// =============================================
// fn serialize_trains(trains: &[Train]) -> Result<String, String>
//
// Use serde_json::to_string_pretty(trains) to get formatted JSON.
// Map the serde_json error to String using .map_err(|e| e.to_string()).

fn serialise_trains(trains: &[Train]) -> Result<String, String> {
    serde_json::to_string_pretty(trains).map_err(|e| e.to_string())
}

// =============================================
// STEP 4: Implement `deserialize_trains`
// =============================================
// fn deserialize_trains(json: &str) -> Result<Vec<Train>, String>
//
// Use serde_json::from_str(json) to parse JSON into Vec<Train>.
// Map the error to String.

fn deserialize_trains(json: &str) -> Result<Vec<Train>, String> {
    serde_json::from_str(json).map_err(|e| e.to_string())
}

// =============================================
// STEP 5: Implement `find_delayed`
// =============================================
// fn find_delayed(trains: &[Train]) -> Vec<&Train>
//
// Filter trains where status is Delayed (any number of minutes).
// Hint: use matches!(t.status, TrainStatus::Delayed { .. })

fn find_delayed(trains: &[Train]) -> Vec<&Train> {
    trains
        .iter()
        .filter(|t| matches!(t.status, TrainStatus::Delayed { .. }))
        .collect()
}

// =============================================
// STEP 6: Implement `to_summary_json`
// =============================================
// fn to_summary_json(trains: &[Train]) -> Result<String, String>
//
// Count trains by status: total, on_time, delayed, cancelled.
// Store counts in a BTreeMap<&str, usize> (BTreeMap keeps keys sorted
// so the JSON output is deterministic — HashMap would randomize key order).
//
// Keys: "total", "on_time", "delayed", "cancelled"
// Serialize to pretty JSON and return.

fn to_summary_json(trains: &[Train]) -> Result<String, String> {
    let mut map: BTreeMap<&str, usize> = BTreeMap::new();

    for train in trains {
        let key: &str = match train.status {
            TrainStatus::Cancelled { .. } => "cancelled",
            TrainStatus::Delayed { .. } => "delayed",
            TrainStatus::OnTime => "on_time",
        };

        *map.entry(key).or_insert(0) += 1;
    }

    map.insert("total", trains.len());

    serde_json::to_string_pretty(&map).map_err(|e| e.to_string())
}

// =============================================
// STEP 7: Implement `merge_json_trains`
// =============================================
// fn merge_json_trains(json1: &str, json2: &str) -> Result<String, String>
//
// 1. Deserialize both JSON strings into Vec<Train>
// 2. Combine them (extend the first with the second)
// 3. Serialize back to pretty JSON

fn merge_json_trains(json1: &str, json2: &str) -> Result<String, String> {
    let mut train1 = serde_json::from_str::<Vec<Train>>(json1).map_err(|e| e.to_string())?;
    let train2 = serde_json::from_str::<Vec<Train>>(json2).map_err(|e| e.to_string())?;

    train1.extend(train2);

    return serde_json::to_string(&train1).map_err(|e| e.to_string());
}

// =============================================
// STEP 8: Write `fn main()`
// =============================================
//
// 1. Print "=== RAILWAY JSON DATA HANDLER ==="
//
// 2. Create a Vec of 4 trains:
//    - Train { number: 12001, name: "Rajdhani Express",  from: "Chennai", to: "New Delhi",
//      status: TrainStatus::OnTime, speed_kmh: Some(130.0) }
//    - Train { number: 12007, name: "Shatabdi Express",  from: "Chennai", to: "Bangalore",
//      status: TrainStatus::Delayed { minutes: 45 }, speed_kmh: Some(150.0) }
//    - Train { number: 12245, name: "Duronto Express",   from: "Chennai", to: "Mumbai",
//      status: TrainStatus::OnTime, speed_kmh: None }
//    - Train { number: 22501, name: "Tejas Express",     from: "Chennai", to: "Madurai",
//      status: TrainStatus::Cancelled { reason: "Track maintenance".to_string() }, speed_kmh: None }
//
// 3. Print "\n--- Serialized JSON ---"
//    Serialize trains, print the JSON.
//
// 4. Print "\n--- Deserialized Trains ---"
//    Deserialize the JSON back. For each train print:
//      "  #{number} {name} ({status:?})"
//
// 5. Print "\n--- Delayed Trains ---"
//    Call find_delayed. For each delayed train print:
//      "  {name} — delayed by {minutes} minutes"
//    (Use if let TrainStatus::Delayed { minutes } = &train.status)
//
// 6. Print "\n--- Summary ---"
//    Call to_summary_json, print the result.
//
// 7. Print "\n--- Invalid JSON Test ---"
//    Try deserializing: r#"{"not valid"#
//    Print:  "  Error (expected): {error_message}"
//
// 8. Print "\n--- Merge Two JSON Arrays ---"
//    Serialize trains[..2] and trains[2..] into two separate JSON strings.
//    Call merge_json_trains to combine them.
//    Deserialize the merged result, print:
//      "  Merged {count} trains from two JSON arrays:"
//    Then for each: "  #{number} {name}"

fn main() {
    // Your code here
}
