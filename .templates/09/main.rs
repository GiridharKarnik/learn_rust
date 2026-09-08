// Lesson 09: Iterators — Railway Data Pipeline
//
// Build a COMPLETE data pipeline for Indian Railway train records.
// You define the struct, implement methods, construct the data, and write
// every iterator-based operation from scratch. Nothing is pre-defined
// except main().
//
// This exercise covers: .iter(), .map(), .filter(), .for_each(),
// .enumerate(), .sum(), .max_by_key(), .take(), .collect(), .join(),
// and combining iterators with HashMap.

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// TODO 1: Define the `TrainRecord` struct
// ---------------------------------------------------------------------------
// Fields:
//   number: u32
//   name: String
//   from: String
//   to: String
//   distance_km: u32
//   fare: f64
//   on_time_percent: f64
//   daily_passengers: u32
//
// Derive: Debug, Clone
//
// WHY: Reinforces struct definition from earlier lessons. Every subsequent
//      TODO depends on this struct, so getting it right is essential.
// ---------------------------------------------------------------------------

// your struct here

// ---------------------------------------------------------------------------
// TODO 2: Implement `TrainRecord` methods
// ---------------------------------------------------------------------------
// Inside `impl TrainRecord`, define three methods:
//
//   fn new(number: u32, name: &str, from: &str, to: &str,
//          distance_km: u32, fare: f64, on_time_percent: f64,
//          daily_passengers: u32) -> Self
//     — Constructs a new TrainRecord (convert &str args to String).
//
//   fn revenue(&self) -> f64
//     — Returns fare * daily_passengers as f64.
//       Used by top_revenue_trains (TODO 5), so this isn't busywork.
//
//   fn display_short(&self) -> String
//     — Returns: "#{number} {name}: {from} → {to} ({distance_km} km, ₹{fare:.2})"
//       Used by format_timetable (TODO 4).
//
// WHY: Reinforces method definition. revenue() and display_short() are
//      consumed by later TODOs, connecting structs to iterators.
// ---------------------------------------------------------------------------

// your impl here

// ---------------------------------------------------------------------------
// TODO 3: Implement `build_records() -> Vec<TrainRecord>`
// ---------------------------------------------------------------------------
// Return a vec! with these 8 train records (use TrainRecord::new()):
//
//   number | name                       | from        | to                  | km   | fare    | on_time | passengers
//   -------|----------------------------|-------------|---------------------|------|---------|---------|----------
//   12001  | "Rajdhani Express"         | "Chennai"   | "New Delhi"         | 2180 | 4250.00 |  92.5   | 3000
//   12007  | "Shatabdi Express"         | "Chennai"   | "Bangalore"         |  350 |  755.50 |  88.0   | 4500
//   12245  | "Duronto Express"          | "Chennai"   | "Mumbai"            | 1280 | 2100.00 |  85.3   | 2200
//   12675  | "Kovai Express"            | "Chennai"   | "Coimbatore"        |  500 |  350.00 |  72.4   | 5500
//   16525  | "Island Express"           | "Bangalore" | "Kanyakumari"       |  890 |  625.00 |  78.9   | 3200
//   12625  | "Thiruvananthapuram Mail"  | "Chennai"   | "Thiruvananthapuram" |  920 |  780.00 |  65.0   | 5800
//   22501  | "Tejas Express"            | "Chennai"   | "Madurai"           |  460 | 1200.00 |  94.1   | 8000
//   20601  | "Vande Bharat Express"     | "Chennai"   | "Bangalore"         |  350 | 1850.00 |  97.2   | 6000
//
// WHY: Practice building a Vec of structs using a constructor.
// ---------------------------------------------------------------------------

fn build_records() -> Vec<TrainRecord> {
    todo!()
}

// ---------------------------------------------------------------------------
// TODO 4: Implement `format_timetable(records: &[TrainRecord]) -> String`
// ---------------------------------------------------------------------------
// Use: .iter().map(|r| r.display_short()).collect::<Vec<_>>().join("\n")
//
// This chains three iterator operations:
//   .map()     — transforms each record into its short display string
//   .collect() — gathers the strings into a Vec<String>
//   .join()    — combines them with newline separators
//
// WHY: .map().collect().join() is the most common iterator chain. This is
//      the pattern you'll use constantly in real Rust code.
// ---------------------------------------------------------------------------

fn format_timetable(records: &[TrainRecord]) -> String {
    todo!()
}

// ---------------------------------------------------------------------------
// TODO 5: Implement `top_revenue_trains(records: &[TrainRecord], n: usize) -> Vec<&TrainRecord>`
// ---------------------------------------------------------------------------
// Steps:
//   1. Collect references into a Vec<&TrainRecord>
//   2. Sort by revenue (use .revenue()) descending
//   3. Take top `n` with .into_iter().take(n).collect()
//
// Hint: Use .sort_by() with .partial_cmp() for f64 comparison.
//       For descending order, compare rev_b to rev_a.
//
// WHY: Practice sorting a collected iterator + .take() to limit results.
// ---------------------------------------------------------------------------

fn top_revenue_trains(records: &[TrainRecord], n: usize) -> Vec<&TrainRecord> {
    todo!()
}

// ---------------------------------------------------------------------------
// TODO 6: Implement `average_fare_by_route(records: &[TrainRecord]) -> Vec<(String, f64)>`
// ---------------------------------------------------------------------------
// Steps:
//   1. Create a HashMap<String, Vec<f64>> to group fares by route
//   2. For each record, build route as format!("{} → {}", r.from, r.to)
//   3. Use .entry(route).or_insert_with(Vec::new).push(r.fare)
//   4. Convert the HashMap into a Vec<(String, f64)> where f64 is the average fare
//   5. Sort alphabetically by route name
//
// WHY: Practice HashMap + iterators together (reinforces Lesson 08).
//      The entry API + iterator pipeline is a very common Rust pattern.
// ---------------------------------------------------------------------------

fn average_fare_by_route(records: &[TrainRecord]) -> Vec<(String, f64)> {
    todo!()
}

// ---------------------------------------------------------------------------
// TODO 7: Implement `reliability_report(records: &[TrainRecord])`
// ---------------------------------------------------------------------------
// Print trains categorized by on_time_percent:
//
//   Excellent (>= 95%):
//     — filter for on_time_percent >= 95.0
//   Good (80–94.9%):
//     — filter for on_time_percent >= 80.0 && < 95.0
//   Needs Improvement (< 80%):
//     — filter for on_time_percent < 80.0
//
// For each category:
//   1. println! the category header
//   2. Use .iter().filter(...).for_each(...) to print matching trains
//   3. Format each as: "  {name} ({on_time_percent:.1}%)"
//
// WHY: Practice multiple .filter() passes over the same data. The
//      .filter().for_each() pattern is useful for side-effectful iteration.
// ---------------------------------------------------------------------------

fn reliability_report(records: &[TrainRecord]) {
    todo!()
}

// ---------------------------------------------------------------------------
// TODO 8: Implement `total_stats(records: &[TrainRecord])`
// ---------------------------------------------------------------------------
// Print a summary with these five stats:
//
//   Total distance:       .iter().map(|r| r.distance_km).sum()
//   Total daily passengers: .iter().map(|r| r.daily_passengers).sum()
//   Average on-time:      sum of on_time_percent / count, format as {:.2}%
//   Most popular:         .iter().max_by_key(|r| r.daily_passengers)
//   Longest route:        .iter().max_by_key(|r| r.distance_km)
//
// Expected output format:
//   Total distance: {total} km
//   Total daily passengers: {total}
//   Average on-time: {avg:.2}%
//   Most popular: {name} ({passengers} passengers/day)
//   Longest route: {name} ({km} km)
//
// WHY: Practice iterator consumers — .sum(), .max_by_key() — that reduce
//      a collection to a single value.
// ---------------------------------------------------------------------------

fn total_stats(records: &[TrainRecord]) {
    todo!()
}

// ---------------------------------------------------------------------------
// TODO 9: Implement `search_and_transform(records: &[TrainRecord], min_distance: u32) -> String`
// ---------------------------------------------------------------------------
// Build a full pipeline:
//   1. Filter records where distance_km >= min_distance
//   2. Collect into a Vec and sort by fare ascending
//   3. Enumerate (to get 1-based index)
//   4. Map to: "{i}. {name} ({distance_km} km) — ₹{fare:.2}"
//   5. Collect into Vec<String> and .join("\n")
//
// Hint: Collect filtered refs first, sort, then chain
//       .iter().enumerate().map(...).collect::<Vec<_>>().join("\n")
//
// WHY: This is the capstone — filter → sort → enumerate → map → collect →
//      join in one logical pipeline. It uses nearly everything from this lesson.
// ---------------------------------------------------------------------------

fn search_and_transform(records: &[TrainRecord], min_distance: u32) -> String {
    todo!()
}

// =============================================
// main() — DO NOT MODIFY
// =============================================

fn main() {
    println!("=== RAILWAY DATA PIPELINE ===");

    let records = build_records();

    // TODO 4
    println!("\n--- Timetable ---");
    println!("{}", format_timetable(&records));

    // TODO 5
    println!("\n--- Top 3 Revenue Trains ---");
    let top3 = top_revenue_trains(&records, 3);
    for (i, t) in top3.iter().enumerate() {
        let revenue = t.fare * t.daily_passengers as f64;
        println!("{}. {} — ₹{:.2}/day", i + 1, t.name, revenue);
    }

    // TODO 6
    println!("\n--- Average Fare by Route ---");
    let avg_fares = average_fare_by_route(&records);
    for (route, avg) in &avg_fares {
        println!("{}: ₹{:.2}", route, avg);
    }

    // TODO 7
    println!("\n--- Reliability Report ---");
    reliability_report(&records);

    // TODO 8
    println!("\n--- Total Stats ---");
    total_stats(&records);

    // TODO 9
    println!("\n--- Long-Distance Trains (>= 500 km), Cheapest First ---");
    println!("{}", search_and_transform(&records, 500));
}
