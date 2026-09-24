// =============================================
// EXERCISE 12: Railway Query System
// =============================================
//
// Practice generics: generic functions, generic structs, trait bounds,
// where clauses, and monomorphization.
//
// You know: variables, functions, ownership, structs, enums, Option, Result,
// Vec, HashMap, iterators, panics, traits. Now add generics to your toolkit.
//
// Run with: cargo run
//
// Expected output:
//
//   === RAILWAY QUERY SYSTEM ===
//
//   --- print_all: Trains ---
//     [TRAIN] #12001 Rajdhani Express (130km/h)
//     [TRAIN] #12007 Shatabdi Express (150km/h)
//     [TRAIN] #20601 Vande Bharat Express (160km/h)
//
//   --- print_all: Stations ---
//     [STATION] Chennai Central [MAS] — 17 platforms
//     [STATION] New Delhi [NDLS] — 16 platforms
//     [STATION] Mumbai CST [CSMT] — 18 platforms
//
//   --- find_by_name ---
//     Found train: [TRAIN] #20601 Vande Bharat Express (160km/h)
//     Found station: [STATION] Mumbai CST [CSMT] — 18 platforms
//     No route containing 'Kolkata'
//
//   --- largest ---
//     Fastest speed: 160km/h
//     Most platforms: 18
//
//   --- Pair ---
//     first: #12001 Rajdhani Express (130km/h)
//     second: #20601 Vande Bharat Express (160km/h)
//     after swap — first: #20601 Vande Bharat Express (160km/h)
//
//   === DONE ===

#[allow(unused_imports)]
use std::fmt;

// =============================================
// Pre-provided: Displayable trait and data types
// =============================================
// The following trait and structs are already implemented for you.
// Read through them — they show the patterns you'll apply in your TODOs.

trait Displayable {
    fn display_line(&self) -> String;
    fn display_type(&self) -> &str;

    fn display_full(&self) -> String {
        format!("[{}] {}", self.display_type(), self.display_line())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Train {
    number: u32,
    name: String,
    speed_kmh: u32,
}

impl Displayable for Train {
    fn display_line(&self) -> String {
        format!("#{} {} ({}km/h)", self.number, self.name, self.speed_kmh)
    }
    fn display_type(&self) -> &str {
        "TRAIN"
    }
}

impl fmt::Display for Train {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} {} ({}km/h)", self.number, self.name, self.speed_kmh)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Station {
    name: String,
    code: String,
    platforms: u8,
}

impl Displayable for Station {
    fn display_line(&self) -> String {
        format!("{} [{}] \u{2014} {} platforms", self.name, self.code, self.platforms)
    }
    fn display_type(&self) -> &str {
        "STATION"
    }
}

impl fmt::Display for Station {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} [{}]", self.name, self.code)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Route {
    from: String,
    to: String,
    distance_km: u32,
    trains: Vec<String>,
}

impl Displayable for Route {
    fn display_line(&self) -> String {
        format!("{} \u{2192} {} ({}km)", self.from, self.to, self.distance_km)
    }
    fn display_type(&self) -> &str {
        "ROUTE"
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} \u{2192} {} ({}km)", self.from, self.to, self.distance_km)
    }
}

// =============================================
// STEP 1: Write a generic function `print_all`
// =============================================
// fn print_all<T: Displayable>(items: &[T])
//
// For each item, print: "  {display_full}"   (four spaces indent)

// =============================================
// STEP 2: Write a generic function `find_by_name`
// =============================================
// fn find_by_name<'a, T: Displayable>(items: &'a [T], name: &str) -> Option<&'a T>
//
// Return the first item whose display_line() contains `name`.
// Hint: items.iter().find(|item| item.display_line().contains(name))

// =============================================
// STEP 3: Write a generic function `largest`
// =============================================
// fn largest<T: PartialOrd>(list: &[T]) -> &T
//
// Return a reference to the largest item in the slice.
// Hint: start with &list[0], iterate from list[1..], update if item > biggest.

// =============================================
// STEP 4: Define a generic struct `Pair<T>`
// =============================================
// struct Pair<T> { first: T, second: T }
//
// Implement a method: fn swap(self) -> Pair<T>
// This consumes the Pair and returns a new one with first and second swapped.
//
// Add a trait bound so Display works: impl<T: fmt::Display> Pair<T>
// (or split into two impl blocks — one for swap, one for Display methods)

// =============================================
// STEP 5: Write main()
// =============================================
//
// 1. Print "=== RAILWAY QUERY SYSTEM ==="
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
// 5. Print "--- print_all: Trains ---" then call print_all(&trains)
//
// 6. Print "--- print_all: Stations ---" then call print_all(&stations)
//
// 7. Print "--- find_by_name ---"
//    Search trains for "Vande"   — print "  Found train: {display_full}"
//    Search stations for "Mumbai" — print "  Found station: {display_full}"
//    Search routes for "Kolkata" — print "  No route containing 'Kolkata'"
//    (Use find_by_name; for the not-found case use unwrap_or-style logic or a match)
//
// 8. Print "--- largest ---"
//    Collect train speeds into a Vec<u32>: trains.iter().map(|t| t.speed_kmh).collect()
//    Call largest(&speeds) and print "  Fastest speed: {n}km/h"
//    Collect station platforms into a Vec<u8>: stations.iter().map(|s| s.platforms).collect()
//    Call largest(&platforms) and print "  Most platforms: {n}"
//
// 9. Print "--- Pair ---"
//    Create: Pair { first: trains[0].clone(), second: trains[2].clone() }
//    Print "  first: {}" using Display (trains[0])
//    Print "  second: {}" using Display (trains[2])
//    Call .swap() on the pair
//    Print "  after swap — first: {}" using Display (swapped.first)
//
// 10. Print "=== DONE ==="

fn main() {}
