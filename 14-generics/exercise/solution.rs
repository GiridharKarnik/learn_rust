// Lesson 14: Generics — Railway Query System (SOLUTION)

use std::fmt;

// ---------------------------------------------------------------------------
// Pre-provided: Displayable trait and data types
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// STEP 1: print_all
// ---------------------------------------------------------------------------

fn print_all<T: Displayable>(items: &[T]) {
    for item in items {
        println!("  {}", item.display_full());
    }
}

// ---------------------------------------------------------------------------
// STEP 2: find_by_name
// ---------------------------------------------------------------------------

fn find_by_name<'a, T: Displayable>(items: &'a [T], name: &str) -> Option<&'a T> {
    items.iter().find(|item| item.display_line().contains(name))
}

// ---------------------------------------------------------------------------
// STEP 3: largest
// ---------------------------------------------------------------------------

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut biggest = &list[0];
    for item in &list[1..] {
        if item > biggest {
            biggest = item;
        }
    }
    biggest
}

// ---------------------------------------------------------------------------
// STEP 4: Pair<T>
// ---------------------------------------------------------------------------

struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn swap(self) -> Pair<T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

// ---------------------------------------------------------------------------
// STEP 5: main
// ---------------------------------------------------------------------------

fn main() {
    println!("=== RAILWAY QUERY SYSTEM ===");

    let trains = vec![
        Train { number: 12001, name: "Rajdhani Express".to_string(), speed_kmh: 130 },
        Train { number: 12007, name: "Shatabdi Express".to_string(), speed_kmh: 150 },
        Train { number: 20601, name: "Vande Bharat Express".to_string(), speed_kmh: 160 },
    ];

    let stations = vec![
        Station { name: "Chennai Central".to_string(), code: "MAS".to_string(), platforms: 17 },
        Station { name: "New Delhi".to_string(), code: "NDLS".to_string(), platforms: 16 },
        Station { name: "Mumbai CST".to_string(), code: "CSMT".to_string(), platforms: 18 },
    ];

    let routes = vec![
        Route { from: "Chennai".to_string(), to: "New Delhi".to_string(), distance_km: 2180, trains: vec![] },
        Route { from: "Chennai".to_string(), to: "Mumbai".to_string(), distance_km: 1280, trains: vec![] },
        Route { from: "Chennai".to_string(), to: "Bangalore".to_string(), distance_km: 350, trains: vec![] },
    ];

    println!();
    println!("--- print_all: Trains ---");
    print_all(&trains);

    println!();
    println!("--- print_all: Stations ---");
    print_all(&stations);

    println!();
    println!("--- find_by_name ---");
    match find_by_name(&trains, "Vande") {
        Some(train) => println!("  Found train: {}", train.display_full()),
        None => println!("  No train containing 'Vande'"),
    }
    match find_by_name(&stations, "Mumbai") {
        Some(station) => println!("  Found station: {}", station.display_full()),
        None => println!("  No station containing 'Mumbai'"),
    }
    match find_by_name(&routes, "Kolkata") {
        Some(route) => println!("  Found route: {}", route.display_full()),
        None => println!("  No route containing 'Kolkata'"),
    }

    println!();
    println!("--- largest ---");
    let speeds: Vec<u32> = trains.iter().map(|t| t.speed_kmh).collect();
    println!("  Fastest speed: {}km/h", largest(&speeds));
    let platforms: Vec<u8> = stations.iter().map(|s| s.platforms).collect();
    println!("  Most platforms: {}", largest(&platforms));

    println!();
    println!("--- Pair ---");
    let pair = Pair { first: trains[0].clone(), second: trains[2].clone() };
    println!("  first: {}", pair.first);
    println!("  second: {}", pair.second);
    let swapped = pair.swap();
    println!("  after swap — first: {}", swapped.first);

    println!();
    println!("=== DONE ===");
}
