// Lesson 11: Traits & Generics — Railway Display System (SOLUTION)

use std::fmt;

// ---------------------------------------------------------------------------
// STEP 1: Define the `Displayable` trait
// ---------------------------------------------------------------------------

trait Displayable {
    fn display_line(&self) -> String;
    fn display_type(&self) -> &str;

    fn display_full(&self) -> String {
        format!("[{}] {}", self.display_type(), self.display_line())
    }
}

// ---------------------------------------------------------------------------
// STEP 2: Define `Train` and implement `Displayable`
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// STEP 3: Define `Station` and implement `Displayable`
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
struct Station {
    name: String,
    code: String,
    platforms: u8,
}

impl Displayable for Station {
    fn display_line(&self) -> String {
        format!(
            "{} [{}] — {} platforms",
            self.name, self.code, self.platforms
        )
    }

    fn display_type(&self) -> &str {
        "STATION"
    }
}

// ---------------------------------------------------------------------------
// STEP 4: Define `Route` and implement `Displayable`
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
struct Route {
    from: String,
    to: String,
    distance_km: u32,
    trains: Vec<String>,
}

impl Displayable for Route {
    fn display_line(&self) -> String {
        format!("{} → {} ({}km)", self.from, self.to, self.distance_km)
    }

    fn display_type(&self) -> &str {
        "ROUTE"
    }
}

// ---------------------------------------------------------------------------
// STEP 5: Implement fmt::Display for Train, Station, Route
// ---------------------------------------------------------------------------

impl fmt::Display for Train {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} {} ({}km/h)", self.number, self.name, self.speed_kmh)
    }
}

impl fmt::Display for Station {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} [{}]", self.name, self.code)
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} → {} ({}km)", self.from, self.to, self.distance_km)
    }
}

// ---------------------------------------------------------------------------
// STEP 6: Generic function print_all
// ---------------------------------------------------------------------------

fn print_all<T: Displayable>(items: &[T]) {
    for item in items {
        println!("  {}", item.display_full());
    }
}

// ---------------------------------------------------------------------------
// STEP 7: Generic function find_by_name
// ---------------------------------------------------------------------------

fn find_by_name<'a, T: Displayable>(items: &'a [T], name: &str) -> Option<&'a T> {
    items.iter().find(|item| item.display_line().contains(name))
}

// ---------------------------------------------------------------------------
// STEP 8: Implement From<(&str, &str, u32)> for Route
// ---------------------------------------------------------------------------

impl From<(&str, &str, u32)> for Route {
    fn from(tuple: (&str, &str, u32)) -> Self {
        Route {
            from: tuple.0.to_string(),
            to: tuple.1.to_string(),
            distance_km: tuple.2,
            trains: vec![],
        }
    }
}

// ---------------------------------------------------------------------------
// STEP 9: main()
// ---------------------------------------------------------------------------

fn main() {
    println!("=== RAILWAY DISPLAY SYSTEM ===");

    let trains = vec![
        Train {
            number: 12001,
            name: "Rajdhani Express".to_string(),
            speed_kmh: 130,
        },
        Train {
            number: 12007,
            name: "Shatabdi Express".to_string(),
            speed_kmh: 150,
        },
        Train {
            number: 20601,
            name: "Vande Bharat Express".to_string(),
            speed_kmh: 160,
        },
    ];

    let stations = vec![
        Station {
            name: "Chennai Central".to_string(),
            code: "MAS".to_string(),
            platforms: 17,
        },
        Station {
            name: "New Delhi".to_string(),
            code: "NDLS".to_string(),
            platforms: 16,
        },
        Station {
            name: "Mumbai CST".to_string(),
            code: "CSMT".to_string(),
            platforms: 18,
        },
    ];

    let routes = vec![
        Route {
            from: "Chennai".to_string(),
            to: "New Delhi".to_string(),
            distance_km: 2180,
            trains: vec![],
        },
        Route {
            from: "Chennai".to_string(),
            to: "Mumbai".to_string(),
            distance_km: 1280,
            trains: vec![],
        },
        Route {
            from: "Chennai".to_string(),
            to: "Bangalore".to_string(),
            distance_km: 350,
            trains: vec![],
        },
    ];

    println!();
    println!("--- All Trains ---");
    print_all(&trains);

    println!();
    println!("--- All Stations ---");
    print_all(&stations);

    println!();
    println!("--- All Routes ---");
    print_all(&routes);

    println!();
    println!("--- Display with println! (Display trait) ---");
    println!("  {}", trains[0]);
    println!("  {}", stations[0]);
    println!("  {}", routes[0]);

    println!();
    println!("--- Search: find_by_name ---");
    match find_by_name(&trains, "Vande") {
        Some(t) => println!("  Found train: {}", t.display_full()),
        None => println!("  Not found"),
    }
    match find_by_name(&stations, "Mumbai") {
        Some(s) => println!("  Found station: {}", s.display_full()),
        None => println!("  Not found"),
    }
    match find_by_name(&routes, "Kolkata") {
        Some(r) => println!("  Found route: {}", r.display_full()),
        None => println!("  No route containing 'Kolkata'"),
    }

    println!();
    println!("--- From/Into Conversion ---");
    let r1 = Route::from(("Delhi", "Agra", 200));
    let r2: Route = ("Mumbai", "Pune", 150).into();
    println!("  Route from tuple: {}", r1);
    println!("  Route via .into(): {}", r2);

    println!();
    println!("=== DONE ===");
}
