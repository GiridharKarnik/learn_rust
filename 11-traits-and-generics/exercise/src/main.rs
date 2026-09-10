// =============================================
// EXERCISE 11: Railway Display System
// =============================================
//
// Build a system where multiple types (Train, Station, Route) share common
// behavior through traits and generics.
//
// You know: variables, functions, ownership, structs, enums, Option, Result,
// Vec, HashMap, iterators, panics. Now add traits & generics to your toolkit.
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
//   --- Search: find_by_name ---
//   Found train: [TRAIN] #20601 Vande Bharat Express (160km/h)
//   Found station: [STATION] Mumbai CST [CSMT] — 18 platforms
//   No route containing 'Kolkata'
//
//   --- From/Into Conversion ---
//   Route from tuple: Delhi → Agra (200km)
//   Route via .into(): Mumbai → Pune (150km)
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
// STEP 6: Write a generic function `print_all`
// =============================================
// fn print_all<T: Displayable>(items: &[T])
//
// For each item, print: "  {display_full}"   (two spaces indent)

// =============================================
// STEP 7: Write a generic function `find_by_name`
// =============================================
// fn find_by_name<'a, T: Displayable>(items: &'a [T], name: &str) -> Option<&'a T>
//
// Return the first item whose display_line() contains `name`.
// Hint: items.iter().find(|item| item.display_line().contains(name))

// =============================================
// STEP 8: Implement From<(&str, &str, u32)> for Route
// =============================================
// Convert a tuple (from, to, distance_km) into a Route.
// The trains vec should be empty.

// =============================================
// STEP 9: Write main()
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
// 4. Create a Vec of routes:
//    Route { from: "Chennai", to: "New Delhi",   distance_km: 2180, trains: vec![] }
//    Route { from: "Chennai", to: "Mumbai",      distance_km: 1280, trains: vec![] }
//    Route { from: "Chennai", to: "Bangalore",   distance_km: 350,  trains: vec![] }
//
// 5. Print "--- All Trains ---" then call print_all(&trains)
// 6. Print "--- All Stations ---" then call print_all(&stations)
// 7. Print "--- All Routes ---" then call print_all(&routes)
//
// 8. Print "--- Display with println! (Display trait) ---"
//    println!("  {}", trains[0]);
//    println!("  {}", stations[0]);
//    println!("  {}", routes[0]);
//
// 9. Print "--- Search: find_by_name ---"
//    Search trains for "Vande" — print "Found train: {display_full}"
//    Search stations for "Mumbai" — print "Found station: {display_full}"
//    Search routes for "Kolkata" — print "No route containing 'Kolkata'"
//
// 10. Print "--- From/Into Conversion ---"
//     let r1 = Route::from(("Delhi", "Agra", 200));
//     let r2: Route = ("Mumbai", "Pune", 150).into();
//     println!("  Route from tuple: {}", r1);
//     println!("  Route via .into(): {}", r2);
//
// 11. Print "=== DONE ==="

fn main() {}
