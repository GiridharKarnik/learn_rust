// =============================================
// EXERCISE 09: Railway Data Pipeline
// =============================================
//
// Build a complete data analysis pipeline from scratch.
// There is NO code provided — you write everything, including main().
//
// This exercises: structs, methods, Vec, HashMap, and iterator chains.
//
// Run with: cargo run
//
// Expected output:
//
//   === RAILWAY DATA PIPELINE ===
//
//   --- Timetable ---
//   #12001 Rajdhani Express: Chennai → New Delhi (2180 km, ₹4250.00)
//   #12007 Shatabdi Express: Chennai → Bangalore (350 km, ₹755.50)
//   #12245 Duronto Express: Chennai → Mumbai (1280 km, ₹2100.00)
//   #12675 Kovai Express: Chennai → Coimbatore (500 km, ₹350.00)
//   #16525 Island Express: Bangalore → Kanyakumari (890 km, ₹625.00)
//   #12625 Thiruvananthapuram Mail: Chennai → Thiruvananthapuram (920 km, ₹780.00)
//   #22501 Tejas Express: Chennai → Madurai (460 km, ₹1200.00)
//   #20601 Vande Bharat Express: Chennai → Bangalore (350 km, ₹1850.00)
//
//   --- Top 3 Revenue Trains ---
//   1. Vande Bharat Express — ₹11100000.00/day
//   2. Tejas Express — ₹9600000.00/day
//   3. Kovai Express — ₹1925000.00/day
//
//   --- Average Fare by Route ---
//   Bangalore → Kanyakumari: ₹625.00
//   Chennai → Bangalore: ₹1302.75
//   Chennai → Coimbatore: ₹350.00
//   Chennai → Madurai: ₹1200.00
//   Chennai → Mumbai: ₹2100.00
//   Chennai → New Delhi: ₹4250.00
//   Chennai → Thiruvananthapuram: ₹780.00
//
//   --- Reliability Report ---
//   Excellent (>= 95%):
//     Vande Bharat Express (97.2%)
//   Good (80–94.9%):
//     Rajdhani Express (92.5%)
//     Shatabdi Express (88.0%)
//     Duronto Express (85.3%)
//     Tejas Express (94.1%)
//   Needs Improvement (< 80%):
//     Kovai Express (72.4%)
//     Island Express (78.9%)
//     Thiruvananthapuram Mail (65.0%)
//
//   --- Total Stats ---
//   Total distance: 6930 km
//   Total daily passengers: 38200
//   Average on-time: 84.18%
//   Most popular: Tejas Express (8000 passengers/day)
//   Longest route: Rajdhani Express (2180 km)
//
//   --- Long-Distance Trains (>= 500 km), Cheapest First ---
//   1. Kovai Express (500 km) — ₹350.00
//   2. Island Express (890 km) — ₹625.00
//   3. Thiruvananthapuram Mail (920 km) — ₹780.00
//   4. Duronto Express (1280 km) — ₹2100.00
//   5. Rajdhani Express (2180 km) — ₹4250.00

use std::collections::HashMap;

// =============================================
// STEP 1: Define the `TrainRecord` struct
// =============================================
// Fields:
//   number: u32, name: String, from: String, to: String,
//   distance_km: u32, fare: f64, on_time_percent: f64, daily_passengers: u32
// Derive: Debug, Clone

// =============================================
// STEP 2: Implement `TrainRecord` methods
// =============================================
// - fn new(number: u32, name: &str, from: &str, to: &str,
//          distance_km: u32, fare: f64, on_time_percent: f64,
//          daily_passengers: u32) -> Self
//
// - fn revenue(&self) -> f64
//     Returns: fare * daily_passengers as f64
//     Used by top_revenue_trains — not busywork.
//
// - fn display_short(&self) -> String
//     Returns: "#{number} {name}: {from} → {to} ({distance_km} km, ₹{fare:.2})"
//     Used by format_timetable — not busywork.

// =============================================
// STEP 3: Implement `build_records() -> Vec<TrainRecord>`
// =============================================
// Return a vec! with these 8 trains using TrainRecord::new():
//   (12001, "Rajdhani Express",        "Chennai", "New Delhi",          2180, 4250.00, 92.5, 3000)
//   (12007, "Shatabdi Express",        "Chennai", "Bangalore",          350,  755.50, 88.0, 4500)
//   (12245, "Duronto Express",         "Chennai", "Mumbai",            1280, 2100.00, 85.3, 2200)
//   (12675, "Kovai Express",           "Chennai", "Coimbatore",         500,  350.00, 72.4, 5500)
//   (16525, "Island Express",          "Bangalore", "Kanyakumari",      890,  625.00, 78.9, 3200)
//   (12625, "Thiruvananthapuram Mail", "Chennai", "Thiruvananthapuram",  920,  780.00, 65.0, 5800)
//   (22501, "Tejas Express",           "Chennai", "Madurai",            460, 1200.00, 94.1, 8000)
//   (20601, "Vande Bharat Express",    "Chennai", "Bangalore",          350, 1850.00, 97.2, 6000)

// =============================================
// STEP 4: Implement `format_timetable`
// =============================================
// fn format_timetable(records: &[TrainRecord]) -> String
//
// Use: records.iter().map(|r| r.display_short()).collect::<Vec<_>>().join("\n")
// This is the most common iterator pattern: .map().collect().join()

// =============================================
// STEP 5: Implement `top_revenue_trains`
// =============================================
// fn top_revenue_trains(records: &[TrainRecord], n: usize) -> Vec<&TrainRecord>
//
// Collect into Vec<&TrainRecord>, sort by revenue() descending,
// then take first n. Return as Vec.

// =============================================
// STEP 6: Implement `average_fare_by_route`
// =============================================
// fn average_fare_by_route(records: &[TrainRecord]) -> Vec<(String, f64)>
//
// Group records by route ("{from} → {to}"), compute average fare per route.
// Steps:
//   1. Build HashMap<String, Vec<f64>> — route → list of fares
//   2. Convert to Vec<(String, f64)> — route → average fare
//   3. Sort by route name
// This combines HashMap (Lesson 08) with iterators.

// =============================================
// STEP 7: Implement `reliability_report`
// =============================================
// fn reliability_report(records: &[TrainRecord])
//
// Print trains in 3 categories based on on_time_percent:
//   Excellent (>= 95%): filter and print
//   Good (80–94.9%): filter and print
//   Needs Improvement (< 80%): filter and print
// Use .filter().for_each() for each category.

// =============================================
// STEP 8: Implement `total_stats`
// =============================================
// fn total_stats(records: &[TrainRecord])
//
// Print these stats using iterator consumers:
//   Total distance: .map(|r| r.distance_km).sum::<u32>()
//   Total daily passengers: .map(|r| r.daily_passengers).sum::<u32>()
//   Average on-time: .map(|r| r.on_time_percent).sum::<f64>() / count
//   Most popular: .max_by_key(|r| r.daily_passengers)
//   Longest route: .max_by_key(|r| r.distance_km)

// =============================================
// STEP 9: Implement `search_and_transform`
// =============================================
// fn search_and_transform(records: &[TrainRecord], min_distance: u32) -> String
//
// One pipeline: filter by distance → sort by fare → number them → format → join
//   1. Filter: keep records where distance_km >= min_distance
//   2. Sort: by fare ascending (cheapest first)
//   3. Enumerate + map: "1. Kovai Express (500 km) — ₹350.00"
//   4. Collect and join with "\n"

// =============================================
// STEP 10: Write `fn main()`
// =============================================
//
// 1. Print "=== RAILWAY DATA PIPELINE ==="
// 2. Build records with build_records()
// 3. Print "--- Timetable ---" + format_timetable()
// 4. Print "--- Top 3 Revenue Trains ---"
//    Call top_revenue_trains(3), loop with enumerate, print revenue
// 5. Print "--- Average Fare by Route ---"
//    Call average_fare_by_route(), print each
// 6. Print "--- Reliability Report ---"
//    Call reliability_report()
// 7. Print "--- Total Stats ---"
//    Call total_stats()
// 8. Print "--- Long-Distance Trains (>= 500 km), Cheapest First ---"
//    Print search_and_transform(500)

fn main() {
    // Your code here
}
