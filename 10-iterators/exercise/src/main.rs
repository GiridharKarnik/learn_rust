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
#[derive(Debug, Clone)]
struct TrainRecord {
    number: u32,
    name: String,
    from: String,
    to: String,
    distance_km: u32,
    fare: f64,
    on_time_percent: f64,
    daily_passengers: u32,
}

// =============================================
// STEP 2: Implement `TrainRecord` methods
// =============================================
impl TrainRecord {
    // - fn new(number: u32, name: &str, from: &str, to: &str,
    //          distance_km: u32, fare: f64, on_time_percent: f64,
    //          daily_passengers: u32) -> Self
    fn new(
        number: u32,
        name: &str,
        from: &str,
        to: &str,
        distance_km: u32,
        fare: f64,
        on_time_percent: f64,
        daily_passengers: u32,
    ) -> Self {
        TrainRecord {
            number,
            name: name.to_string(),
            from: from.to_string(),
            to: to.to_string(),
            distance_km,
            fare,
            on_time_percent,
            daily_passengers,
        }
    }

    // - fn revenue(&self) -> f64
    //     Returns: fare * daily_passengers as f64
    //     Used by top_revenue_trains — not busywork.
    fn revenue(&self) -> f64 {
        self.fare * self.daily_passengers as f64
    }

    // - fn display_short(&self) -> String
    //     Returns: "#{number} {name}: {from} → {to} ({distance_km} km, ₹{fare:.2})"
    //     Used by format_timetable — not busywork.
    fn display_short(&self) -> String {
        format!(
            "#{number} {name}: {from} → {to} ({distance_km} km, ₹{fare:.2})",
            number = self.number,
            name = self.name,
            from = self.from,
            to = self.to,
            distance_km = self.distance_km,
            fare = self.fare
        )
    }
}

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

fn build_records() -> Vec<TrainRecord> {
    vec![
        TrainRecord {
            number: 12001,
            name: "Rajdhani Express".to_string(),
            from: "Chennai".to_string(),
            to: "New Delhi".to_string(),
            distance_km: 2180,
            fare: 4250.00,
            on_time_percent: 92.5,
            daily_passengers: 3000,
        },
        TrainRecord {
            number: 12007,
            name: "Shatabdi Express".to_string(),
            from: "Chennai".to_string(),
            to: "Bangalore".to_string(),
            distance_km: 350,
            fare: 755.50,
            on_time_percent: 88.0,
            daily_passengers: 4500,
        },
        TrainRecord {
            number: 12245,
            name: "Duronto Express".to_string(),
            from: "Chennai".to_string(),
            to: "Mumbai".to_string(),
            distance_km: 1280,
            fare: 2100.00,
            on_time_percent: 85.3,
            daily_passengers: 2200,
        },
        TrainRecord {
            number: 12675,
            name: "Kovai Express".to_string(),
            from: "Chennai".to_string(),
            to: "Coimbatore.to_string()",
            distance_km: 500,
            fare: 350.00,
            on_time_percent: 72.4,
            daily_passengers: 5500,
        },
        TrainRecord {
            number: 16525,
            name: "Island Express".to_string(),
            from: "Bangalore".to_string(),
            to: "Kanyakumari".to_string(),
            distance_km: 890,
            fare: 625.00,
            on_time_percent: 78.9,
            daily_passengers: 3200,
        },
        TrainRecord {
            number: 12625,
            name: "Thiruvananthapuram Mail".to_string(),
            from: "Chennai".to_string(),
            to: "Thiruvananthapuram".to_string(),
            distance_km: 920,
            fare: 780.00,
            on_time_percent: 65.0,
            daily_passengers: 5800,
        },
        TrainRecord {
            number: 22501,
            name: "Tejas Express".to_string(),
            from: "Chennai".to_string(),
            to: "Madurai".to_string(),
            distance_km: 460,
            fare: 1200.00,
            on_time_percent: 94.1,
            daily_passengers: 8000,
        },
        TrainRecord {
            number: 20601,
            name: "Vande Bharat Express".to_string(),
            from: "Chennai".to_string(),
            to: "Bangalore".to_string(),
            distance_km: 350,
            fare: 1850.00,
            on_time_percent: 97.2,
            daily_passengers: 6000,
        },
    ]
}

// =============================================
// STEP 4: Implement `format_timetable`
// =============================================
// fn format_timetable(records: &[TrainRecord]) -> String
//
// Use: records.iter().map(|r| r.display_short()).collect::<Vec<_>>().join("\n")
// This is the most common iterator pattern: .map().collect().join()

fn format_timetable(records: &[TrainRecord]) -> String {
    records
        .iter()
        .map(|r| r.display_short())
        .collect::<Vec<_>>()
        .join("\n")
}
// =============================================
// STEP 5: Implement `top_revenue_trains`
// =============================================
// fn top_revenue_trains(records: &[TrainRecord], n: usize) -> Vec<&TrainRecord>
//
// Collect into Vec<&TrainRecord>, sort by revenue() descending,
// then take first n. Return as Vec.
fn top_revenue_trains(records: &[TrainRecord], n: usize) -> Vec<&TrainRecord> {
    let mut sorted: Vec<&TrainRecord> = records.iter().collect();

    sorted.sort_by(|a, b| b.revenue().partial_cmp(&a.revenue()).unwrap());

    return sorted;
}

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

fn average_fare_by_route(records: &[TrainRecord]) -> Vec<(String, f64)> {
    let mut route_fares: HashMap<String, Vec<f64>> = HashMap::new();
    for r in records {
        let route = format!("{} → {}", r.from, r.to);
        route_fares
            .entry(route)
            .or_insert_with(Vec::new)
            .push(r.fare);
    }

    let mut result: Vec<(String, f64)> = route_fares
        .into_iter()
        .map(|(route, fares)| {
            let avg = fares.iter().sum::<f64>() / fares.len() as f64;
            (route, avg)
        })
        .collect();

    result.sort_by(|a, b| a.0.cmp(&b.0));
    result
}

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
fn reliability_report(records: &[TrainRecord]) {
    //first group the trains into a new hashmap with keys Excellent, Good and Needs Improvement
    let mut service_tiers: HashMap<&str, Vec<&TrainRecord>> = HashMap::new();

    for record in records {
        let key = if record.on_time_percent >= 95.0 {
            "Excellent"
        } else if record.on_time_percent >= 80.0 {
            "Good"
        } else {
            "Needs Improvement"
        };

        service_tiers
            .entry(key)
            .or_insert_with(Vec::new)
            .push(record);
    }

    println!("Excellent (>= 95%):");
    for t in service_tiers.get("Excellent").unwrap_or(&vec![]) {
        println!("  {} ({:.1}%)", t.name, t.on_time_percent);
    }

    println!("Good (80–94.9%):");
    for t in service_tiers.get("Good").unwrap_or(&vec![]) {
        println!("  {} ({:.1}%)", t.name, t.on_time_percent);
    }

    println!("Needs Improvement (< 80%):");
    for t in service_tiers.get("Needs Improvement").unwrap_or(&vec![]) {
        println!("  {} ({:.1}%)", t.name, t.on_time_percent);
    }
}

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

fn total_stats(records: &[TrainRecord]) {
    let total_distance = records.iter().map(|r| r.distance_km).sum::<u32>();
}

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
