// Lesson 07: Collections & Iterators — Railway Route Planner (SOLUTION)

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

// ---------------------------------------------------------------------------
// TODO 1: Train::new and build_schedule
// ---------------------------------------------------------------------------

impl Train {
    fn new(
        number: u32,
        name: &str,
        from: &str,
        to: &str,
        distance_km: u32,
        fare: f64,
        stops: Vec<&str>,
    ) -> Self {
        Train {
            number,
            name: name.to_string(),
            from: from.to_string(),
            to: to.to_string(),
            distance_km,
            fare,
            stops: stops.iter().map(|s| s.to_string()).collect(),
        }
    }
}

fn build_schedule() -> Vec<Train> {
    vec![
        Train::new(
            12001,
            "Rajdhani Express",
            "Chennai",
            "New Delhi",
            2180,
            4250.00,
            vec![
                "Chennai",
                "Katpadi",
                "Vijayawada",
                "Nagpur",
                "Bhopal",
                "Agra",
                "New Delhi",
            ],
        ),
        Train::new(
            12007,
            "Shatabdi Express",
            "Chennai",
            "Bangalore",
            350,
            755.50,
            vec!["Chennai", "Katpadi", "Jolarpettai", "Bangalore"],
        ),
        Train::new(
            12245,
            "Duronto Express",
            "Chennai",
            "Mumbai",
            1280,
            2100.00,
            vec!["Chennai", "Pune", "Mumbai"],
        ),
        Train::new(
            12675,
            "Kovai Express",
            "Chennai",
            "Coimbatore",
            500,
            350.00,
            vec!["Chennai", "Katpadi", "Salem", "Erode", "Coimbatore"],
        ),
        Train::new(
            16525,
            "Island Express",
            "Bangalore",
            "Kanyakumari",
            890,
            625.00,
            vec![
                "Bangalore",
                "Jolarpettai",
                "Erode",
                "Madurai",
                "Kanyakumari",
            ],
        ),
        Train::new(
            12625,
            "Thiruvananthapuram Mail",
            "Chennai",
            "Thiruvananthapuram",
            920,
            780.00,
            vec![
                "Chennai",
                "Salem",
                "Coimbatore",
                "Palakkad",
                "Thiruvananthapuram",
            ],
        ),
    ]
}

// ---------------------------------------------------------------------------
// TODO 2: search_trains
// ---------------------------------------------------------------------------

fn search_trains<'a>(trains: &'a [Train], station: &str) -> Vec<&'a Train> {
    trains
        .iter()
        .filter(|t| t.stops.contains(&station.to_string()))
        .collect()
}

// ---------------------------------------------------------------------------
// TODO 3: fare_summary
// ---------------------------------------------------------------------------

fn fare_summary(trains: &[Train]) -> HashMap<String, f64> {
    let mut fares = HashMap::new();
    for train in trains {
        let route = format!("{} \u{2192} {}", train.from, train.to);
        fares.insert(route, train.fare);
    }
    fares
}

// ---------------------------------------------------------------------------
// TODO 4: busiest_stations
// ---------------------------------------------------------------------------

fn busiest_stations(trains: &[Train]) -> Vec<(String, usize)> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for train in trains {
        for stop in &train.stops {
            *counts.entry(stop.clone()).or_insert(0) += 1;
        }
    }
    let mut result: Vec<(String, usize)> = counts.into_iter().collect();
    result.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    result
}

// ---------------------------------------------------------------------------
// TODO 5: route_summary
// ---------------------------------------------------------------------------

fn route_summary(trains: &[Train]) {
    let total_trains = trains.len();
    let total_distance: u32 = trains.iter().map(|t| t.distance_km).sum();
    let average_fare: f64 = trains.iter().map(|t| t.fare).sum::<f64>() / total_trains as f64;
    let longest = trains.iter().max_by_key(|t| t.distance_km).unwrap();
    let cheapest = trains
        .iter()
        .min_by(|a, b| a.fare.partial_cmp(&b.fare).unwrap())
        .unwrap();

    println!("--- Route Summary ---");
    println!("Total trains: {}", total_trains);
    println!("Total distance: {} km", total_distance);
    println!("Average fare: \u{20b9}{:.2}", average_fare);
    println!(
        "Longest route: #{} {} ({} km)",
        longest.number, longest.name, longest.distance_km
    );
    println!(
        "Cheapest train: #{} {} (\u{20b9}{:.2})",
        cheapest.number, cheapest.name, cheapest.fare
    );
}

// ---------------------------------------------------------------------------
// TODO 6: format_timetable
// ---------------------------------------------------------------------------

fn format_timetable(trains: &[Train]) -> String {
    trains
        .iter()
        .map(|t| {
            format!(
                "#{} {}: {} \u{2192} {} ({} km, \u{20b9}{:.2})",
                t.number, t.name, t.from, t.to, t.distance_km, t.fare
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

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
        println!("  {} : \u{20b9}{:.2}", route, fare);
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
