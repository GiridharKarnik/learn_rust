// =============================================
// EXERCISE 08: Station Directory
// =============================================
//
// Build a complete station directory system from scratch.
// There is NO code provided — you write everything, including main().
//
// This exercises: structs, methods, HashMap, Option, entry API, sorting.
//
// Run with: cargo run
//
// Expected output:
//
//   === STATION DIRECTORY ===
//
//   --- All Stations (sorted by code) ---
//   BCT: Mumbai Central (Mumbai) — 9 platforms
//   HWH: Howrah (Kolkata) — 23 platforms
//   MAS: Chennai Central (Chennai) — 12 platforms
//   NDLS: New Delhi (Delhi) — 16 platforms
//   SBC: Bangalore City (Bangalore) — 10 platforms
//
//   --- Station Lookup ---
//   Looking up MAS: Found — Chennai Central (Chennai) — 12 platforms
//   Looking up SBC: Found — Bangalore City (Bangalore) — 10 platforms
//   Looking up XYZ: Not found
//
//   --- Trains Per City ---
//   Chennai: 4 trains
//   Bangalore: 2 trains
//   Mumbai: 2 trains
//   Delhi: 1 trains
//   Kolkata: 1 trains
//
//   --- Stations By City ---
//   Bangalore: ["Bangalore City"]
//   Chennai: ["Chennai Central"]
//   Delhi: ["New Delhi"]
//   Kolkata: ["Howrah"]
//   Mumbai: ["Mumbai Central"]

// =============================================
// STEP 1: Define the `Station` struct
// =============================================
// Fields: name (String), code (String), platforms (u32), city (String)
// Derive: Debug, Clone

// =============================================
// STEP 2: Implement `Station` methods
// =============================================
// - fn new(name: &str, code: &str, platforms: u32, city: &str) -> Self
// - fn display(&self) -> String
//     Returns: "{code}: {name} ({city}) — {platforms} platforms"
//     Example: "MAS: Chennai Central (Chennai) — 12 platforms"

// =============================================
// STEP 3: Implement `build_directory() -> HashMap<String, Station>`
// =============================================
// Create a HashMap keyed by station code. Add these 5 stations:
//   Code  | Name              | City       | Platforms
//   MAS   | Chennai Central   | Chennai    | 12
//   SBC   | Bangalore City    | Bangalore  | 10
//   NDLS  | New Delhi         | Delhi      | 16
//   BCT   | Mumbai Central    | Mumbai     | 9
//   HWH   | Howrah            | Kolkata    | 23
//
// Use: let mut dir = HashMap::new();
//      dir.insert("MAS".to_string(), Station::new(...));
// Don't forget: use std::collections::HashMap;

// =============================================
// STEP 4: Implement `lookup_station`
// =============================================
// fn lookup_station<'a>(directory: &'a HashMap<String, Station>, code: &str) -> Option<&'a Station>
//
// Use directory.get(code) — returns Option<&Station>.
// This is a one-liner.

// =============================================
// STEP 5: Implement `trains_per_city`
// =============================================
// fn trains_per_city(directory: &HashMap<String, Station>, trains: &[(String, String)]) -> HashMap<String, u32>
//
// `trains` is a list of (from_code, to_code) pairs.
// Count how many trains DEPART from each city.
//
// Steps:
//   1. Create empty HashMap for counts
//   2. For each (from_code, _) in trains:
//      - Look up from_code in directory to get the city
//      - Use entry API: *counts.entry(city).or_insert(0) += 1;
//   3. Return counts
//
// Skip any from_code not found in the directory.

// =============================================
// STEP 6: Implement `stations_by_city`
// =============================================
// fn stations_by_city(directory: &HashMap<String, Station>) -> HashMap<String, Vec<String>>
//
// Group station NAMES by their city.
//
// Use entry API grouping pattern:
//   grouped.entry(station.city.clone())
//       .or_insert_with(Vec::new)
//       .push(station.name.clone());

// =============================================
// STEP 7: Implement `display_directory`
// =============================================
// fn display_directory(directory: &HashMap<String, Station>)
//
// Print all stations sorted by code.
// Steps:
//   1. Collect keys: let mut codes: Vec<&String> = directory.keys().collect();
//   2. Sort: codes.sort();
//   3. Loop and print each using station.display()

// =============================================
// STEP 8: Write `fn main()`
// =============================================
//
// 1. Print "=== STATION DIRECTORY ==="
//
// 2. Print "--- All Stations (sorted by code) ---"
//    Call display_directory()
//
// 3. Print "--- Station Lookup ---"
//    Look up "MAS", "SBC", "XYZ" using lookup_station()
//    Use match: Some → print found info, None → print "Not found"
//
// 4. Print "--- Trains Per City ---"
//    Create this train route list:
//      vec![
//          ("MAS", "SBC"),  ("MAS", "NDLS"), ("BCT", "NDLS"),
//          ("SBC", "MAS"),  ("MAS", "BCT"),  ("HWH", "NDLS"),
//          ("NDLS", "MAS"), ("BCT", "HWH"),  ("SBC", "HWH"),
//          ("MAS", "HWH"),
//      ]
//    (Convert each &str to String with .to_string())
//    Call trains_per_city(), sort result by count descending, print each
//
// 5. Print "--- Stations By City ---"
//    Call stations_by_city(), sort by city name, print each

fn main() {
    // Your code here
}
