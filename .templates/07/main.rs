// =============================================
// EXERCISE 07: Railway Route Planner
// =============================================
//
// Build a route planner using Vec, HashMap, and iterators.
// Complete all the TODOs so the program compiles and produces the expected output.
//
// Run with: cargo run
//
// Expected output:
//
//   === RAILWAY ROUTE PLANNER ===
//
//   --- Timetable ---
//   #12001 Rajdhani Express: Chennai → New Delhi (2180 km, ₹4250.00)
//   #12007 Shatabdi Express: Chennai → Bangalore (350 km, ₹755.50)
//   #12245 Duronto Express: Chennai → Mumbai (1280 km, ₹2100.00)
//   #12675 Kovai Express: Chennai → Coimbatore (500 km, ₹350.00)
//   #16525 Island Express: Bangalore → Kanyakumari (890 km, ₹625.00)
//   #12625 Thiruvananthapuram Mail: Chennai → Thiruvananthapuram (920 km, ₹780.00)
//
//   --- Trains through Katpadi ---
//     #12001 Rajdhani Express
//     #12007 Shatabdi Express
//     #12675 Kovai Express
//
//   --- Trains through Salem ---
//     #12675 Kovai Express
//     #12625 Thiruvananthapuram Mail
//
//   --- Fare Summary ---
//     Bangalore → Kanyakumari : ₹625.00
//     Chennai → Bangalore : ₹755.50
//     Chennai → Coimbatore : ₹350.00
//     Chennai → Mumbai : ₹2100.00
//     Chennai → New Delhi : ₹4250.00
//     Chennai → Thiruvananthapuram : ₹780.00
//
//   --- Busiest Stations ---
//     Chennai: 5 trains
//     Katpadi: 3 trains
//     Bangalore: 2 trains
//     Coimbatore: 2 trains
//     Erode: 2 trains
//
//   --- Route Summary ---
//   Total trains: 6
//   Total distance: 6120 km
//   Average fare: ₹1476.75
//   Longest route: #12001 Rajdhani Express (2180 km)
//   Cheapest train: #12675 Kovai Express (₹350.00)

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Train {
    number: u32,
    name: String,
    from: String,
    to: String,
    distance_km: u32,
    fare: f64,
    stops: Vec<String>,
}

// =============================================
// TODO 1: Train::new and build_schedule
// =============================================
//
// (a) Implement Train::new:
//     fn new(number: u32, name: &str, from: &str, to: &str,
//            distance_km: u32, fare: f64, stops: Vec<&str>) -> Self
//
//     Convert &str fields to String with .to_string()
//     Convert stops Vec<&str> to Vec<String> using:
//       stops.iter().map(|s| s.to_string()).collect()
//
//     WHY: This is your first real iterator chain — .iter().map().collect()
//          is the bread and butter of Rust collections.
//
// (b) Implement build_schedule() -> Vec<Train>
//     Return a vec! with these 6 trains:
//
//     #12001 Rajdhani Express: Chennai → New Delhi, 2180 km, ₹4250.00
//       Stops: Chennai, Katpadi, Vijayawada, Nagpur, Bhopal, Agra, New Delhi
//
//     #12007 Shatabdi Express: Chennai → Bangalore, 350 km, ₹755.50
//       Stops: Chennai, Katpadi, Jolarpettai, Bangalore
//
//     #12245 Duronto Express: Chennai → Mumbai, 1280 km, ₹2100.00
//       Stops: Chennai, Pune, Mumbai
//
//     #12675 Kovai Express: Chennai → Coimbatore, 500 km, ₹350.00
//       Stops: Chennai, Katpadi, Salem, Erode, Coimbatore
//
//     #16525 Island Express: Bangalore → Kanyakumari, 890 km, ₹625.00
//       Stops: Bangalore, Jolarpettai, Erode, Madurai, Kanyakumari
//
//     #12625 Thiruvananthapuram Mail: Chennai → Thiruvananthapuram, 920 km, ₹780.00
//       Stops: Chennai, Salem, Coimbatore, Palakkad, Thiruvananthapuram

// =============================================
// TODO 2: search_trains
// =============================================
//
// fn search_trains<'a>(trains: &'a [Train], station: &str) -> Vec<&'a Train>
//
// Return all trains that STOP at the given station.
// Check if station is in train.stops using .contains().
//
// You can use a for loop:
//   let mut result = Vec::new();
//   for train in trains {
//       if train.stops.contains(&station.to_string()) {
//           result.push(train);
//       }
//   }
//   result
//
// Or use iterators:
//   trains.iter().filter(|t| t.stops.contains(&station.to_string())).collect()
//
// WHY: Practice filtering a collection. The <'a> lifetime just means
//      "the references I return live as long as the input slice."
//      Don't worry about lifetimes too much — the compiler guided you here.

// =============================================
// TODO 3: fare_summary
// =============================================
//
// fn fare_summary(trains: &[Train]) -> HashMap<String, f64>
//
// Build a HashMap where:
//   key = "{from} → {to}" (use format! with the → arrow)
//   value = the fare
//
// Loop through trains, format the route string, insert into HashMap.
//
// WHY: Practice building a HashMap from data. This is a very common pattern —
//      transforming a Vec into a lookup table.

// =============================================
// TODO 4: busiest_stations
// =============================================
//
// fn busiest_stations(trains: &[Train]) -> Vec<(String, usize)>
//
// Count how many trains stop at each station across ALL trains.
// Return as a Vec of (station_name, count) sorted by count descending.
//
// Steps:
//   1. Create a HashMap<String, usize> for counts
//   2. Loop through every train, then every stop in that train
//   3. Use the entry API to count:
//        *counts.entry(stop.clone()).or_insert(0) += 1;
//   4. Convert HashMap to Vec:
//        let mut result: Vec<(String, usize)> = counts.into_iter().collect();
//   5. Sort descending by count:
//        result.sort_by(|a, b| b.1.cmp(&a.1));
//   6. Return result
//
// WHY: The entry API is THE most important HashMap pattern in Rust.
//      .entry(key).or_insert(default) says "give me the value for this key,
//      or insert a default if it doesn't exist." The * dereferences so you
//      can modify the value in place.

// =============================================
// TODO 5: route_summary
// =============================================
//
// fn route_summary(trains: &[Train])
//
// Print these stats using iterator methods:
//   - Total trains: trains.len()
//   - Total distance: sum all distance_km
//       trains.iter().map(|t| t.distance_km).sum::<u32>()
//   - Average fare: sum all fares / count
//       trains.iter().map(|t| t.fare).sum::<f64>() / trains.len() as f64
//   - Longest route: train with max distance_km
//       trains.iter().max_by_key(|t| t.distance_km).unwrap()
//   - Cheapest train: train with min fare
//       trains.iter().min_by(|a, b| a.fare.partial_cmp(&b.fare).unwrap()).unwrap()
//
// Print format:
//   --- Route Summary ---
//   Total trains: 6
//   Total distance: 6120 km
//   Average fare: ₹1476.75
//   Longest route: #12001 Rajdhani Express (2180 km)
//   Cheapest train: #12675 Kovai Express (₹350.00)
//
// WHY: Practice iterator consumers — .sum(), .max_by_key(), .min_by().
//      Note: .partial_cmp() is needed for f64 because floats can be NaN,
//      and NaN doesn't have a defined ordering.

// =============================================
// TODO 6: format_timetable
// =============================================
//
// fn format_timetable(trains: &[Train]) -> String
//
// Build a single String with all trains, one per line:
//   "#12001 Rajdhani Express: Chennai → New Delhi (2180 km, ₹4250.00)"
//
// Use an iterator chain:
//   trains.iter()
//       .map(|t| format!("#{} {}: {} → {} ({} km, ₹{:.2})",
//           t.number, t.name, t.from, t.to, t.distance_km, t.fare))
//       .collect::<Vec<_>>()
//       .join("\n")
//
// WHY: Practice chaining .map() → .collect() → .join().
//      This pattern (transform each item, collect, join) is extremely common
//      for building formatted output from collections.

// =============================================
// main() — DO NOT EDIT BELOW THIS LINE
// =============================================

fn main() {
    println!("=== RAILWAY ROUTE PLANNER ===");

    let trains = build_schedule();

    // Display timetable
    println!("\n--- Timetable ---");
    println!("{}", format_timetable(&trains));

    // Search for trains through a station
    println!("\n--- Trains through Katpadi ---");
    let katpadi_trains = search_trains(&trains, "Katpadi");
    if katpadi_trains.is_empty() {
        println!("No trains found");
    } else {
        for train in &katpadi_trains {
            println!("  #{} {}", train.number, train.name);
        }
    }

    println!("\n--- Trains through Salem ---");
    let salem_trains = search_trains(&trains, "Salem");
    if salem_trains.is_empty() {
        println!("No trains found");
    } else {
        for train in &salem_trains {
            println!("  #{} {}", train.number, train.name);
        }
    }

    // Fare lookup
    println!("\n--- Fare Summary ---");
    let fares = fare_summary(&trains);
    let mut fare_list: Vec<_> = fares.iter().collect();
    fare_list.sort_by_key(|(route, _)| route.to_string());
    for (route, fare) in &fare_list {
        println!("  {} : ₹{:.2}", route, fare);
    }

    // Busiest stations
    println!("\n--- Busiest Stations ---");
    let stations = busiest_stations(&trains);
    for (station, count) in stations.iter().take(5) {
        println!("  {}: {} trains", station, count);
    }

    // Route summary
    println!();
    route_summary(&trains);
}
