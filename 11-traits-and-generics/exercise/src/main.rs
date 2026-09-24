// =============================================
// EXERCISE 11: Railway Display System
// =============================================
//
// Build a system where multiple types (Train, Station, Route) share common
// behavior through traits.
//
// You know: variables, functions, ownership, structs, enums, Option, Result,
// Vec, HashMap, iterators, panics. Now add traits to your toolkit.
//
// Run with: cargo run
//
// Expected output:
//
//   === RAILWAY DISPLAY SYSTEM ===
//
//   --- All Trains ---
//   [TRAIN] #12001 Rajdhani Express (130km/h)
//   [TRAIN] #12007 Shatabdi Express (150km/h)
//   [TRAIN] #20601 Vande Bharat Express (160km/h)
//
//   --- All Stations ---
//   [STATION] Chennai Central [MAS] — 17 platforms
//   [STATION] New Delhi [NDLS] — 16 platforms
//   [STATION] Mumbai CST [CSMT] — 18 platforms
//
//   --- All Routes ---
//   [ROUTE] Chennai → New Delhi (2180km)
//   [ROUTE] Chennai → Mumbai (1280km)
//   [ROUTE] Chennai → Bangalore (350km)
//
//   --- Display with println! (Display trait) ---
//   #12001 Rajdhani Express (130km/h)
//   Chennai Central [MAS]
//   Chennai → New Delhi (2180km)
//
//   --- impl Trait Return ---
//   [ROUTE] Kolkata → Darjeeling (600km)
//
//   === DONE ===

#[allow(unused_imports)]
use std::fmt;

// =============================================
// STEP 1: Define the `Displayable` trait
// =============================================
// Required methods:
//   fn display_line(&self) -> String
//   fn display_type(&self) -> &str
//
// Default method:
//   fn display_full(&self) -> String
//     returns: "[{display_type}] {display_line}"

// =============================================
// STEP 2: Define `Train` struct and implement `Displayable`
// =============================================
// Fields: number: u32, name: String, speed_kmh: u32
// Derive: Debug, Clone, PartialEq
//
// display_line: "#{number} {name} ({speed_kmh}km/h)"
// display_type: "TRAIN"

// =============================================
// STEP 3: Define `Station` struct and implement `Displayable`
// =============================================
// Fields: name: String, code: String, platforms: u8
// Derive: Debug, Clone, PartialEq
//
// display_line: "{name} [{code}] — {platforms} platforms"
//   (note: the dash is an em-dash —, not a hyphen)
// display_type: "STATION"

// =============================================
// STEP 4: Define `Route` struct and implement `Displayable`
// =============================================
// Fields: from: String, to: String, distance_km: u32, trains: Vec<String>
// Derive: Debug, Clone, PartialEq
//
// display_line: "{from} → {to} ({distance_km}km)"
// display_type: "ROUTE"

// =============================================
// STEP 5: Implement `fmt::Display` for Train, Station, Route
// =============================================
// Train:   "#{number} {name} ({speed_kmh}km/h)"
// Station: "{name} [{code}]"
// Route:   "{from} → {to} ({distance_km}km)"
//
// This lets you write: println!("{}", train);

// =============================================
// STEP 6: Add a constructor to Route
// =============================================
// Add a `new` associated function inside an `impl Route` block:
//   fn new(from: &str, to: &str, distance_km: u32) -> Self
//
// Callers pass &str — the constructor handles .to_string() internally.
// Set trains to vec![].
//
// impl Route {
//     fn new(from: &str, to: &str, distance_km: u32) -> Self {
//         Route { from: from.to_string(), ... }
//     }
// }

// =============================================
// STEP 7: Write `make_summary_route`
// =============================================
// fn make_summary_route(from: &str, to: &str, distance_km: u32) -> impl Displayable
//
// Return a Route { from, to, distance_km, trains: vec![] }.
// The caller only sees `impl Displayable` — not that it's a Route.
//
// Key rule: every return path must produce the same concrete type.
// (To return Train OR Route depending on a condition, you'd need
// Box<dyn Displayable> instead — a later topic.)

// =============================================
// STEP 8: Write main()
// =============================================
//
// 1. Print "=== RAILWAY DISPLAY SYSTEM ==="
//
// 2. Create a Vec of trains:
//    Train { number: 12001, name: "Rajdhani Express",      speed_kmh: 130 }
//    Train { number: 12007, name: "Shatabdi Express",      speed_kmh: 150 }
//    Train { number: 20601, name: "Vande Bharat Express",  speed_kmh: 160 }
//
// 3. Create a Vec of stations:
//    Station { name: "Chennai Central", code: "MAS",  platforms: 17 }
//    Station { name: "New Delhi",       code: "NDLS", platforms: 16 }
//    Station { name: "Mumbai CST",      code: "CSMT", platforms: 18 }
//
// 4. Create a Vec of routes using Route::new():
//    Route::new("Chennai", "New Delhi",  2180)
//    Route::new("Chennai", "Mumbai",     1280)
//    Route::new("Chennai", "Bangalore",   350)
//
// 5. Print "--- All Trains ---"
//    For each train, print: "  {display_full}"   (two spaces indent)
//
// 6. Print "--- All Stations ---"
//    For each station, print: "  {display_full}"   (two spaces indent)
//
// 7. Print "--- All Routes ---"
//    For each route, print: "  {display_full}"   (two spaces indent)
//
// 8. Print "--- Display with println! (Display trait) ---"
//    println!("  {}", trains[0]);
//    println!("  {}", stations[0]);
//    println!("  {}", routes[0]);
//
// 9. Print "--- impl Trait Return ---"
//     let summary = make_summary_route("Kolkata", "Darjeeling", 600);
//     println!("  {}", summary.display_full());
//
// 10. Print "=== DONE ==="

fn main() {}
