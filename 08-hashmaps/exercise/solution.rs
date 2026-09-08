// Lesson 08: HashMaps — Station Directory (SOLUTION)

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// TODO 1: Define the Station struct
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct Station {
    name: String,
    code: String,
    platforms: u32,
    city: String,
}

// ---------------------------------------------------------------------------
// TODO 2: Implement Station methods
// ---------------------------------------------------------------------------

impl Station {
    fn new(name: &str, code: &str, platforms: u32, city: &str) -> Self {
        Station {
            name: name.to_string(),
            code: code.to_string(),
            platforms,
            city: city.to_string(),
        }
    }

    fn display(&self) -> String {
        format!(
            "{}: {} ({}) \u{2014} {} platforms",
            self.code, self.name, self.city, self.platforms
        )
    }
}

// ---------------------------------------------------------------------------
// TODO 3: Build a station directory
// ---------------------------------------------------------------------------

fn build_directory() -> HashMap<String, Station> {
    let mut directory = HashMap::new();

    directory.insert(
        "MAS".to_string(),
        Station::new("Chennai Central", "MAS", 12, "Chennai"),
    );

    directory.insert(
        "SBC".to_string(),
        Station::new("Bangalore City", "SBC", 10, "Bangalore"),
    );

    directory.insert(
        "NDLS".to_string(),
        Station::new("New Delhi", "NDLS", 16, "Delhi"),
    );

    directory.insert(
        "BCT".to_string(),
        Station::new("Mumbai Central", "BCT", 9, "Mumbai"),
    );

    directory.insert(
        "HWH".to_string(),
        Station::new("Howrah", "HWH", 23, "Kolkata"),
    );

    directory
}

// ---------------------------------------------------------------------------
// TODO 4: Look up stations safely
// ---------------------------------------------------------------------------

fn lookup_station<'a>(directory: &'a HashMap<String, Station>, code: &str) -> Option<&'a Station> {
    directory.get(code)
}

// ---------------------------------------------------------------------------
// TODO 5: Count trains per city
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
// TODO 6: Group stations by city
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
// TODO 7: Display directory (sorted by code)
// ---------------------------------------------------------------------------

fn display_directory(directory: &HashMap<String, Station>) {
    let mut codes: Vec<&String> = directory.keys().collect();
    codes.sort();

    for code in &codes {
        let station = &directory[code.as_str()];
        println!("{}", station.display());
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
