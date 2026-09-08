// Lesson 08: HashMaps — Station Directory (SOLUTION)

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Station {
    name: String,
    code: String,
    platforms: u32,
    city: String,
}

// ---------------------------------------------------------------------------
// TODO 1: Build a station lookup
// ---------------------------------------------------------------------------

fn build_directory() -> HashMap<String, Station> {
    let mut directory = HashMap::new();

    directory.insert(
        "MAS".to_string(),
        Station {
            name: "Chennai Central".to_string(),
            code: "MAS".to_string(),
            platforms: 12,
            city: "Chennai".to_string(),
        },
    );

    directory.insert(
        "SBC".to_string(),
        Station {
            name: "Bangalore City".to_string(),
            code: "SBC".to_string(),
            platforms: 10,
            city: "Bangalore".to_string(),
        },
    );

    directory.insert(
        "NDLS".to_string(),
        Station {
            name: "New Delhi".to_string(),
            code: "NDLS".to_string(),
            platforms: 16,
            city: "Delhi".to_string(),
        },
    );

    directory.insert(
        "BCT".to_string(),
        Station {
            name: "Mumbai Central".to_string(),
            code: "BCT".to_string(),
            platforms: 9,
            city: "Mumbai".to_string(),
        },
    );

    directory.insert(
        "HWH".to_string(),
        Station {
            name: "Howrah".to_string(),
            code: "HWH".to_string(),
            platforms: 23,
            city: "Kolkata".to_string(),
        },
    );

    directory
}

// ---------------------------------------------------------------------------
// TODO 2: Look up stations safely
// ---------------------------------------------------------------------------

fn lookup_station<'a>(directory: &'a HashMap<String, Station>, code: &str) -> Option<&'a Station> {
    directory.get(code)
}

// ---------------------------------------------------------------------------
// TODO 3: Count trains per city
// ---------------------------------------------------------------------------

fn trains_per_city(
    directory: &HashMap<String, Station>,
    trains: &[(String, String)],
) -> HashMap<String, u32> {
    let mut counts: HashMap<String, u32> = HashMap::new();

    for (from_code, _to_code) in trains {
        if let Some(station) = directory.get(from_code.as_str()) {
            *counts.entry(station.city.clone()).or_insert(0) += 1;
        }
    }

    counts
}

// ---------------------------------------------------------------------------
// TODO 4: Group stations by city
// ---------------------------------------------------------------------------

fn stations_by_city(directory: &HashMap<String, Station>) -> HashMap<String, Vec<String>> {
    let mut by_city: HashMap<String, Vec<String>> = HashMap::new();

    for station in directory.values() {
        by_city
            .entry(station.city.clone())
            .or_insert_with(Vec::new)
            .push(station.name.clone());
    }

    by_city
}

// ---------------------------------------------------------------------------
// TODO 5: Display directory (sorted by code)
// ---------------------------------------------------------------------------

fn display_directory(directory: &HashMap<String, Station>) {
    let mut codes: Vec<&String> = directory.keys().collect();
    codes.sort();

    for code in &codes {
        let station = &directory[code.as_str()];
        println!(
            "{:<4}: {} ({}) \u{2014} {} platforms",
            station.code, station.name, station.city, station.platforms
        );
    }
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

fn main() {
    println!("=== STATION DIRECTORY ===");

    // Build the directory
    let directory = build_directory();

    // Display all stations sorted by code
    println!("\n--- All Stations (sorted by code) ---");
    display_directory(&directory);

    // Look up specific stations
    println!("\n--- Station Lookup ---");
    for code in &["MAS", "SBC", "XYZ"] {
        match lookup_station(&directory, code) {
            Some(station) => {
                println!(
                    "Looking up {}: \u{2713} {} ({}) \u{2014} {} platforms",
                    code, station.name, station.city, station.platforms
                );
            }
            None => {
                println!("Looking up {}: \u{2717} Not found", code);
            }
        }
    }

    // Count trains per city
    println!("\n--- Trains Per City ---");
    let trains = vec![
        ("MAS".to_string(), "SBC".to_string()),
        ("MAS".to_string(), "NDLS".to_string()),
        ("BCT".to_string(), "NDLS".to_string()),
        ("SBC".to_string(), "MAS".to_string()),
        ("MAS".to_string(), "BCT".to_string()),
        ("HWH".to_string(), "NDLS".to_string()),
        ("NDLS".to_string(), "MAS".to_string()),
        ("BCT".to_string(), "HWH".to_string()),
        ("SBC".to_string(), "HWH".to_string()),
        ("MAS".to_string(), "HWH".to_string()),
    ];

    let city_counts = trains_per_city(&directory, &trains);
    let mut sorted_counts: Vec<_> = city_counts.iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
    for (city, count) in &sorted_counts {
        println!("{}: {} trains", city, count);
    }

    // Group stations by city
    println!("\n--- Stations By City ---");
    let grouped = stations_by_city(&directory);
    let mut sorted_groups: Vec<_> = grouped.iter().collect();
    sorted_groups.sort_by_key(|(city, _)| city.to_string());
    for (city, stations) in &sorted_groups {
        println!("{}: {:?}", city, stations);
    }
}
