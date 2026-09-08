// =============================================
// EXERCISE 08: Station Directory
// =============================================
//
// Build a station directory system using HashMap.
// Complete all the TODOs so the program compiles and produces the expected output.
//
// Run with: cargo run
//
// Expected output:
//
//   === STATION DIRECTORY ===
//
//   --- All Stations (sorted by code) ---
//   BCT : Mumbai Central (Mumbai) — 9 platforms
//   HWH : Howrah (Kolkata) — 23 platforms
//   MAS : Chennai Central (Chennai) — 12 platforms
//   NDLS: New Delhi (Delhi) — 16 platforms
//   SBC : Bangalore City (Bangalore) — 10 platforms
//
//   --- Station Lookup ---
//   Looking up MAS: ✓ Chennai Central (Chennai) — 12 platforms
//   Looking up SBC: ✓ Bangalore City (Bangalore) — 10 platforms
//   Looking up XYZ: ✗ Not found
//
//   --- Trains Per City ---
//   Chennai: 4 trains
//   Mumbai: 2 trains
//   Bangalore: 2 trains
//   Delhi: 1 trains
//   Kolkata: 1 trains
//
//   --- Stations By City ---
//   Bangalore: ["Bangalore City"]
//   Chennai: ["Chennai Central"]
//   Delhi: ["New Delhi"]
//   Kolkata: ["Howrah"]
//   Mumbai: ["Mumbai Central"]

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Station {
    name: String,
    code: String,
    platforms: u32,
    city: String,
}

// =============================================
// TODO 1: Build a station lookup
// =============================================
//
// fn build_directory() -> HashMap<String, Station>
//
// Create a HashMap keyed by station code (String) with Station values.
// Add these 5 stations:
//
//   Code  | Name              | City       | Platforms
//   ------+-------------------+------------+----------
//   MAS   | Chennai Central   | Chennai    | 12
//   SBC   | Bangalore City    | Bangalore  | 10
//   NDLS  | New Delhi         | Delhi      | 16
//   BCT   | Mumbai Central    | Mumbai     | 9
//   HWH   | Howrah            | Kolkata    | 23
//
// Steps:
//   1. Create an empty HashMap with HashMap::new()
//   2. Use .insert() to add each station, keyed by code
//   3. Return the HashMap
//
// WHY: Practice basic HashMap construction.

// =============================================
// TODO 2: Look up stations safely
// =============================================
//
// fn lookup_station<'a>(directory: &'a HashMap<String, Station>, code: &str) -> Option<&'a Station>
//
// Look up a station by its code and return an Option.
// Use .get() — NOT indexing with [] (that would panic on missing keys!).
//
// Hint: This is a one-liner.
//
// WHY: Practice .get() returning Option — the safe way to access HashMap values.

// =============================================
// TODO 3: Count trains per city
// =============================================
//
// fn trains_per_city(
//     directory: &HashMap<String, Station>,
//     trains: &[(String, String)],
// ) -> HashMap<String, u32>
//
// Given a list of (from_code, to_code) tuples representing train routes,
// count how many trains DEPART from each city.
//
// Steps:
//   1. Create an empty HashMap for counts
//   2. Loop through each (from_code, _to_code) in trains
//   3. Look up from_code in the directory to find the city
//   4. Use the entry API counting pattern:
//        *counts.entry(city).or_insert(0) += 1;
//   5. Return the counts
//
// Note: If a from_code isn't in the directory, skip it.
//
// WHY: Practice the entry API counting pattern.

// =============================================
// TODO 4: Group stations by city
// =============================================
//
// fn stations_by_city(directory: &HashMap<String, Station>) -> HashMap<String, Vec<String>>
//
// Group station NAMES by their city field.
//
// Steps:
//   1. Create an empty HashMap<String, Vec<String>>
//   2. Loop through all values in the directory
//   3. Use the entry API grouping pattern:
//        by_city.entry(station.city.clone())
//            .or_insert_with(Vec::new)
//            .push(station.name.clone());
//   4. Return the grouped HashMap
//
// WHY: Practice the entry API grouping pattern (or_insert_with(Vec::new).push()).

// =============================================
// TODO 5: Display directory (sorted by code)
// =============================================
//
// fn display_directory(directory: &HashMap<String, Station>)
//
// Print all stations sorted by their code.
//
// Steps:
//   1. Collect the keys into a Vec:
//        let mut codes: Vec<&String> = directory.keys().collect();
//   2. Sort the Vec:
//        codes.sort();
//   3. Loop through sorted codes and print each station:
//        for code in &codes {
//            let station = &directory[code.as_str()];
//            println!("{:<4}: {} ({}) — {} platforms",
//                code, station.name, station.city, station.platforms);
//        }
//
// Format: "MAS : Chennai Central (Chennai) — 12 platforms"
// Use {:<4} to left-align the code in a 4-char field.
//
// WHY: Practice iterating a HashMap in sorted order.

// =============================================
// main() — DO NOT EDIT BELOW THIS LINE
// =============================================

fn main() {
    println!("=== STATION DIRECTORY ===");

    // Build the directory
    let directory = build_directory();

    // Display all stations sorted by code
    println!("\n--- All Stations (sorted by code) ---");
    display_directory(&directory);

    // Look up specific stations
    println!("\n--- Station Lookup ---");
    for code in &["MAS", "SBC", "XYZ"] {
        match lookup_station(&directory, code) {
            Some(station) => {
                println!(
                    "Looking up {}: ✓ {} ({}) — {} platforms",
                    code, station.name, station.city, station.platforms
                );
            }
            None => {
                println!("Looking up {}: ✗ Not found", code);
            }
        }
    }

    // Count trains per city
    println!("\n--- Trains Per City ---");
    let trains = vec![
        ("MAS".to_string(), "SBC".to_string()),  // Chennai → Bangalore
        ("MAS".to_string(), "NDLS".to_string()), // Chennai → Delhi
        ("BCT".to_string(), "NDLS".to_string()), // Mumbai → Delhi
        ("SBC".to_string(), "MAS".to_string()),  // Bangalore → Chennai
        ("MAS".to_string(), "BCT".to_string()),  // Chennai → Mumbai
        ("HWH".to_string(), "NDLS".to_string()), // Kolkata → Delhi
        ("NDLS".to_string(), "MAS".to_string()), // Delhi → Chennai
        ("BCT".to_string(), "HWH".to_string()),  // Mumbai → Kolkata
        ("SBC".to_string(), "HWH".to_string()),  // Bangalore → Kolkata
        ("MAS".to_string(), "HWH".to_string()),  // Chennai → Kolkata
    ];

    let city_counts = trains_per_city(&directory, &trains);
    let mut sorted_counts: Vec<_> = city_counts.iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
    for (city, count) in &sorted_counts {
        println!("{}: {} trains", city, count);
    }

    // Group stations by city
    println!("\n--- Stations By City ---");
    let grouped = stations_by_city(&directory);
    let mut sorted_groups: Vec<_> = grouped.iter().collect();
    sorted_groups.sort_by_key(|(city, _)| city.to_string());
    for (city, stations) in &sorted_groups {
        println!("{}: {:?}", city, stations);
    }
}
