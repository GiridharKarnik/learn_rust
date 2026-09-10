use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =========================================================================
// 1. 🚂 Basic Serialization — Rust struct to JSON
// =========================================================================

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Train {
    name: String,
    number: u32,
    speed_kmh: f64,
}

// =========================================================================
// 2. 🏷️ Optional Fields
// =========================================================================

#[derive(Serialize, Deserialize, Debug)]
struct TrainWithDelay {
    name: String,
    number: u32,
    delay_minutes: Option<u32>, // null or missing → None
}

// =========================================================================
// 3. 🔧 Renaming and Customizing
// =========================================================================

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct StationBoard {
    station_name: String, // → "stationName" in JSON
    platform_count: u8,   // → "platformCount" in JSON
    #[serde(rename = "id")]
    station_code: String, // → "id" in JSON
    #[serde(skip)]
    #[allow(dead_code)]
    internal_ref: u64, // not included in JSON at all
    #[serde(default)]
    is_active: bool, // uses false if missing from JSON
}

// =========================================================================
// 4. 🎯 Enums in JSON — Tagged unions (like TS discriminated unions)
// =========================================================================

// Internally tagged — adds "type" field like TS discriminated unions
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum TrainEvent {
    Departure { train: String, platform: u8 },
    Arrival { train: String, platform: u8 },
    Delay { train: String, minutes: u32 },
}

// Without any tag attribute — externally tagged (default)
#[derive(Serialize, Deserialize, Debug)]
enum Signal {
    Green,
    Yellow,
    Red { reason: String },
}

// =========================================================================
// 5. 📦 Vec and HashMap Serialization
// =========================================================================

#[derive(Serialize, Deserialize, Debug)]
struct Route {
    name: String,
    stops: Vec<String>,              // → JSON array
    distances: HashMap<String, u32>, // → JSON object
}

fn main() {
    // =====================================================================
    // 1. 🚂 Basic Serialization
    // =====================================================================
    println!("=== 1. 🚂 Basic Serialization ===\n");

    let train = Train {
        name: "Rajdhani Express".to_string(),
        number: 12001,
        speed_kmh: 130.0,
    };

    // Compact JSON
    let compact = serde_json::to_string(&train).expect("serialize failed");
    println!("Compact: {}", compact);

    // Pretty JSON
    let pretty = serde_json::to_string_pretty(&train).expect("serialize failed");
    println!("Pretty:\n{}", pretty);

    // Type mapping:
    println!("\nType mappings:");
    println!("  String  → JSON string:  \"Rajdhani Express\"");
    println!("  u32     → JSON number:  12001");
    println!("  f64     → JSON number:  130.0");
    println!("  bool    → JSON boolean: true/false");
    println!("  Vec     → JSON array:   [...]");
    println!("  HashMap → JSON object:  {{...}}");
    println!("  Option  → JSON null or value");

    // =====================================================================
    // 2. 🔄 Basic Deserialization — JSON to Rust
    // =====================================================================
    println!("\n=== 2. 🔄 Basic Deserialization ===\n");

    let json = r#"{"name":"Shatabdi Express","number":12007,"speed_kmh":150.0}"#;
    let train: Train = serde_json::from_str(json).expect("deserialize failed");
    println!("Deserialized: {:?}", train);
    println!("  name: {}", train.name);
    println!("  number: {}", train.number);
    println!("  speed: {} km/h", train.speed_kmh);

    // Round-trip: serialize → deserialize
    let original = Train {
        name: "Duronto Express".to_string(),
        number: 12245,
        speed_kmh: 120.0,
    };
    let json = serde_json::to_string(&original).unwrap();
    let back: Train = serde_json::from_str(&json).unwrap();
    println!("\nRound-trip: {:?} → JSON → {:?}", original, back);

    // =====================================================================
    // 3. ❌ Error Handling — Invalid JSON
    // =====================================================================
    println!("\n=== 3. ❌ Error Handling ===\n");

    // Invalid JSON syntax
    let bad_json = r#"{"name": broken"#;
    let result: Result<Train, _> = serde_json::from_str(bad_json);
    println!("Bad syntax: {:?}", result.err().unwrap());

    // Wrong types
    let wrong_type = r#"{"name":"Express","number":"not a number","speed_kmh":100.0}"#;
    let result: Result<Train, _> = serde_json::from_str(wrong_type);
    println!("Wrong type: {:?}", result.err().unwrap());

    // Missing required field
    let missing_field = r#"{"name":"Express","number":12001}"#;
    let result: Result<Train, _> = serde_json::from_str(missing_field);
    println!("Missing field: {:?}", result.err().unwrap());

    // Extra fields — silently ignored (not an error!)
    let extra = r#"{"name":"Express","number":12001,"speed_kmh":100.0,"extra":"ignored"}"#;
    let train: Train = serde_json::from_str(extra).unwrap();
    println!("Extra fields ignored: {:?}", train);

    // =====================================================================
    // 4. 🏷️ Optional Fields
    // =====================================================================
    println!("\n=== 4. 🏷️ Optional Fields ===\n");

    // With value
    let json_with_delay = r#"{"name":"Kovai Express","number":12675,"delay_minutes":25}"#;
    let t: TrainWithDelay = serde_json::from_str(json_with_delay).unwrap();
    println!("With delay: {:?}", t);

    // With null
    let json_null = r#"{"name":"Rajdhani Express","number":12001,"delay_minutes":null}"#;
    let t: TrainWithDelay = serde_json::from_str(json_null).unwrap();
    println!("With null:  {:?}", t);

    // Field missing entirely — also becomes None
    let json_missing = r#"{"name":"Rajdhani Express","number":12001}"#;
    let t: TrainWithDelay = serde_json::from_str(json_missing).unwrap();
    println!("Missing:    {:?}", t);

    // Serializing None → null
    let t = TrainWithDelay {
        name: "Express".to_string(),
        number: 99999,
        delay_minutes: None,
    };
    println!("None serializes as: {}", serde_json::to_string(&t).unwrap());

    // =====================================================================
    // 5. 🔧 Renaming and Customizing
    // =====================================================================
    println!("\n=== 5. 🔧 Renaming and Customizing ===\n");

    let board = StationBoard {
        station_name: "Chennai Central".to_string(),
        platform_count: 12,
        station_code: "MAS".to_string(),
        internal_ref: 999999, // won't appear in JSON
        is_active: true,
    };

    let json = serde_json::to_string_pretty(&board).unwrap();
    println!("Serialized with renames:\n{}", json);
    println!("  → station_name became camelCase: stationName");
    println!("  → station_code renamed to: id");
    println!("  → internal_ref skipped entirely");

    // Deserializing camelCase JSON back
    let json = r#"{"stationName":"Bangalore City","platformCount":10,"id":"SBC"}"#;
    let board: StationBoard = serde_json::from_str(json).unwrap();
    println!("\nDeserialized from camelCase:");
    println!("  station_name: {}", board.station_name);
    println!("  platform_count: {}", board.platform_count);
    println!("  station_code: {}", board.station_code);
    println!("  is_active (default): {}", board.is_active); // false — used default

    // =====================================================================
    // 6. 🎯 Enums in JSON
    // =====================================================================
    println!("\n=== 6. 🎯 Enums in JSON ===\n");

    // Internally tagged (with #[serde(tag = "type")])
    let events = vec![
        TrainEvent::Departure {
            train: "Rajdhani Express".to_string(),
            platform: 3,
        },
        TrainEvent::Arrival {
            train: "Shatabdi Express".to_string(),
            platform: 5,
        },
        TrainEvent::Delay {
            train: "Kovai Express".to_string(),
            minutes: 30,
        },
    ];

    println!("Internally tagged (like TS discriminated unions):");
    for event in &events {
        println!("  {}", serde_json::to_string(event).unwrap());
    }

    // Deserializing tagged enum
    let json = r#"{"type":"Delay","train":"Duronto","minutes":15}"#;
    let event: TrainEvent = serde_json::from_str(json).unwrap();
    println!("\nDeserialized tagged enum: {:?}", event);

    // Externally tagged (default — no tag attribute)
    println!("\nExternally tagged (default):");
    let signals = vec![
        Signal::Green,
        Signal::Yellow,
        Signal::Red {
            reason: "Track work ahead".to_string(),
        },
    ];
    for signal in &signals {
        println!("  {}", serde_json::to_string(signal).unwrap());
    }

    // =====================================================================
    // 7. 📦 Vec and HashMap Serialization
    // =====================================================================
    println!("\n=== 7. 📦 Vec and HashMap ===\n");

    // Vec<Train> → JSON array
    let trains = vec![
        Train {
            name: "Rajdhani Express".to_string(),
            number: 12001,
            speed_kmh: 130.0,
        },
        Train {
            name: "Shatabdi Express".to_string(),
            number: 12007,
            speed_kmh: 150.0,
        },
    ];
    println!("Vec<Train> → JSON array:");
    println!("{}", serde_json::to_string_pretty(&trains).unwrap());

    // HashMap<String, T> → JSON object
    let mut speeds: HashMap<String, f64> = HashMap::new();
    speeds.insert("Rajdhani".to_string(), 130.0);
    speeds.insert("Shatabdi".to_string(), 150.0);
    speeds.insert("Duronto".to_string(), 120.0);
    println!("\nHashMap<String, f64> → JSON object:");
    println!("{}", serde_json::to_string_pretty(&speeds).unwrap());

    // Route with nested Vec and HashMap
    let mut distances = HashMap::new();
    distances.insert("Chennai-Katpadi".to_string(), 130);
    distances.insert("Katpadi-Bangalore".to_string(), 220);

    let route = Route {
        name: "Chennai–Bangalore Express Route".to_string(),
        stops: vec![
            "Chennai".to_string(),
            "Katpadi".to_string(),
            "Bangalore".to_string(),
        ],
        distances,
    };
    println!("\nNested struct with Vec + HashMap:");
    println!("{}", serde_json::to_string_pretty(&route).unwrap());

    // =====================================================================
    // 8. 🌐 Untyped JSON (serde_json::Value)
    // =====================================================================
    println!("\n=== 8. 🌐 Untyped JSON ===\n");

    let json = r#"{
        "train": "Rajdhani Express",
        "number": 12001,
        "stops": ["Chennai", "Vijayawada", "New Delhi"],
        "active": true
    }"#;

    let v: serde_json::Value = serde_json::from_str(json).unwrap();

    // Dynamic access like JavaScript
    println!("Dynamic access (like JS):");
    println!("  v[\"train\"]  = {}", v["train"]);
    println!("  v[\"number\"] = {}", v["number"]);
    println!("  v[\"stops\"][0] = {}", v["stops"][0]);
    println!("  v[\"stops\"][2] = {}", v["stops"][2]);
    println!("  v[\"active\"] = {}", v["active"]);

    // Missing keys return Null (not panic)
    println!("  v[\"missing\"] = {} (Null, no panic!)", v["missing"]);

    // Build JSON dynamically with serde_json::json! macro
    let dynamic = serde_json::json!({
        "announcement": "Train arriving",
        "platform": 3,
        "coaches": ["S1", "S2", "A1", "B1"],
    });
    println!("\nBuilt with json! macro:");
    println!("{}", serde_json::to_string_pretty(&dynamic).unwrap());

    // =====================================================================
    // 9. 🌐 TypeScript Comparison Summary
    // =====================================================================
    println!("\n=== 9. 🌐 TypeScript vs Rust+Serde ===\n");

    println!("TS: JSON.parse(str)          → Rust: serde_json::from_str(str)?");
    println!("TS: JSON.stringify(obj)       → Rust: serde_json::to_string(&obj)?");
    println!("TS: runtime type checking     → Rust: compile-time struct matching");
    println!("TS: Zod/io-ts for validation  → Rust: Serde validates by default");
    println!("TS: obj.field (any)           → Rust: value[\"field\"] (serde_json::Value)");
    println!("TS: camelCase native          → Rust: #[serde(rename_all = \"camelCase\")]");
    println!("TS: discriminated unions      → Rust: #[serde(tag = \"type\")]");
    println!("\n🚂 That's Serde! Type-safe JSON handling with zero runtime overhead.");
}
