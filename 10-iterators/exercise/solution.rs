// Lesson 09: Iterators — Railway Data Pipeline (SOLUTION)

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// TODO 1: Define the `TrainRecord` struct
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// TODO 2: Implement `TrainRecord` methods
// ---------------------------------------------------------------------------

impl TrainRecord {
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

    fn revenue(&self) -> f64 {
        self.fare * self.daily_passengers as f64
    }

    fn display_short(&self) -> String {
        format!(
            "#{} {}: {} → {} ({} km, ₹{:.2})",
            self.number, self.name, self.from, self.to, self.distance_km, self.fare
        )
    }
}

// ---------------------------------------------------------------------------
// TODO 3: build_records
// ---------------------------------------------------------------------------

fn build_records() -> Vec<TrainRecord> {
    vec![
        TrainRecord::new(
            12001,
            "Rajdhani Express",
            "Chennai",
            "New Delhi",
            2180,
            4250.00,
            92.5,
            3000,
        ),
        TrainRecord::new(
            12007,
            "Shatabdi Express",
            "Chennai",
            "Bangalore",
            350,
            755.50,
            88.0,
            4500,
        ),
        TrainRecord::new(
            12245,
            "Duronto Express",
            "Chennai",
            "Mumbai",
            1280,
            2100.00,
            85.3,
            2200,
        ),
        TrainRecord::new(
            12675,
            "Kovai Express",
            "Chennai",
            "Coimbatore",
            500,
            350.00,
            72.4,
            5500,
        ),
        TrainRecord::new(
            16525,
            "Island Express",
            "Bangalore",
            "Kanyakumari",
            890,
            625.00,
            78.9,
            3200,
        ),
        TrainRecord::new(
            12625,
            "Thiruvananthapuram Mail",
            "Chennai",
            "Thiruvananthapuram",
            920,
            780.00,
            65.0,
            5800,
        ),
        TrainRecord::new(
            22501,
            "Tejas Express",
            "Chennai",
            "Madurai",
            460,
            1200.00,
            94.1,
            8000,
        ),
        TrainRecord::new(
            20601,
            "Vande Bharat Express",
            "Chennai",
            "Bangalore",
            350,
            1850.00,
            97.2,
            6000,
        ),
    ]
}

// ---------------------------------------------------------------------------
// TODO 4: format_timetable
// ---------------------------------------------------------------------------

fn format_timetable(records: &[TrainRecord]) -> String {
    records
        .iter()
        .map(|r| r.display_short())
        .collect::<Vec<_>>()
        .join("\n")
}

// ---------------------------------------------------------------------------
// TODO 5: top_revenue_trains
// ---------------------------------------------------------------------------

fn top_revenue_trains(records: &[TrainRecord], n: usize) -> Vec<&TrainRecord> {
    let mut sorted: Vec<&TrainRecord> = records.iter().collect();
    sorted.sort_by(|a, b| {
        let rev_a = a.revenue();
        let rev_b = b.revenue();
        rev_b.partial_cmp(&rev_a).unwrap()
    });
    sorted.into_iter().take(n).collect()
}

// ---------------------------------------------------------------------------
// TODO 6: average_fare_by_route
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// TODO 7: reliability_report
// ---------------------------------------------------------------------------

fn reliability_report(records: &[TrainRecord]) {
    println!("Excellent (>= 95%):");
    records
        .iter()
        .filter(|r| r.on_time_percent >= 95.0)
        .for_each(|r| println!("  {} ({:.1}%)", r.name, r.on_time_percent));

    println!("Good (80–94.9%):");
    records
        .iter()
        .filter(|r| r.on_time_percent >= 80.0 && r.on_time_percent < 95.0)
        .for_each(|r| println!("  {} ({:.1}%)", r.name, r.on_time_percent));

    println!("Needs Improvement (< 80%):");
    records
        .iter()
        .filter(|r| r.on_time_percent < 80.0)
        .for_each(|r| println!("  {} ({:.1}%)", r.name, r.on_time_percent));
}

// ---------------------------------------------------------------------------
// TODO 8: total_stats
// ---------------------------------------------------------------------------

fn total_stats(records: &[TrainRecord]) {
    let total_distance: u32 = records.iter().map(|r| r.distance_km).sum();
    let total_passengers: u32 = records.iter().map(|r| r.daily_passengers).sum();
    let avg_on_time: f64 =
        records.iter().map(|r| r.on_time_percent).sum::<f64>() / records.len() as f64;
    let most_popular = records.iter().max_by_key(|r| r.daily_passengers).unwrap();
    let longest = records.iter().max_by_key(|r| r.distance_km).unwrap();

    println!("Total distance: {} km", total_distance);
    println!("Total daily passengers: {}", total_passengers);
    println!("Average on-time: {:.2}%", avg_on_time);
    println!(
        "Most popular: {} ({} passengers/day)",
        most_popular.name, most_popular.daily_passengers
    );
    println!(
        "Longest route: {} ({} km)",
        longest.name, longest.distance_km
    );
}

// ---------------------------------------------------------------------------
// TODO 9: search_and_transform
// ---------------------------------------------------------------------------

fn search_and_transform(records: &[TrainRecord], min_distance: u32) -> String {
    let mut filtered: Vec<&TrainRecord> = records
        .iter()
        .filter(|r| r.distance_km >= min_distance)
        .collect();

    filtered.sort_by(|a, b| a.fare.partial_cmp(&b.fare).unwrap());

    filtered
        .iter()
        .enumerate()
        .map(|(i, r)| {
            format!(
                "{}. {} ({} km) — ₹{:.2}",
                i + 1,
                r.name,
                r.distance_km,
                r.fare
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
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
