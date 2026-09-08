// Lesson 09: Iterators — Railway Data Pipeline (SOLUTION)

use std::collections::HashMap;

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
            to: "Coimbatore".to_string(),
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

// ---------------------------------------------------------------------------
// TODO 1: format_timetable
// ---------------------------------------------------------------------------

fn format_timetable(records: &[TrainRecord]) -> String {
    records
        .iter()
        .map(|r| {
            format!(
                "#{} {}: {} → {} ({} km, ₹{:.2})",
                r.number, r.name, r.from, r.to, r.distance_km, r.fare
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ---------------------------------------------------------------------------
// TODO 2: top_revenue_trains
// ---------------------------------------------------------------------------

fn top_revenue_trains(records: &[TrainRecord], n: usize) -> Vec<&TrainRecord> {
    let mut sorted: Vec<&TrainRecord> = records.iter().collect();
    sorted.sort_by(|a, b| {
        let rev_a = a.fare * a.daily_passengers as f64;
        let rev_b = b.fare * b.daily_passengers as f64;
        rev_b.partial_cmp(&rev_a).unwrap()
    });
    sorted.into_iter().take(n).collect()
}

// ---------------------------------------------------------------------------
// TODO 3: average_fare_by_route
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
// TODO 4: reliability_report
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
// TODO 5: total_stats
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
// TODO 6: search_and_transform
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
// main()
// =============================================

fn main() {
    println!("=== RAILWAY DATA PIPELINE ===");

    let records = build_records();

    // TODO 1
    println!("\n--- Timetable ---");
    println!("{}", format_timetable(&records));

    // TODO 2
    println!("\n--- Top 3 Revenue Trains ---");
    let top3 = top_revenue_trains(&records, 3);
    for (i, t) in top3.iter().enumerate() {
        let revenue = t.fare * t.daily_passengers as f64;
        println!("{}. {} — ₹{:.2}/day", i + 1, t.name, revenue);
    }

    // TODO 3
    println!("\n--- Average Fare by Route ---");
    let avg_fares = average_fare_by_route(&records);
    for (route, avg) in &avg_fares {
        println!("{}: ₹{:.2}", route, avg);
    }

    // TODO 4
    println!("\n--- Reliability Report ---");
    reliability_report(&records);

    // TODO 5
    println!("\n--- Total Stats ---");
    total_stats(&records);

    // TODO 6
    println!("\n--- Long-Distance Trains (>= 500 km), Cheapest First ---");
    println!("{}", search_and_transform(&records, 500));
}
