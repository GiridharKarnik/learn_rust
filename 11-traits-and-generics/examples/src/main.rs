// Lesson 11: Traits & Generics — Comprehensive Examples
//
// Run with: cargo run
//
// This file demonstrates every concept from the lesson using railway-themed types.

use std::fmt;

// =============================================================================
// 1. Defining a Trait
// =============================================================================
// A trait defines shared behavior — any type that implements it must provide
// these methods. Think: "anything that can appear on a departure board."

trait Displayable {
    fn display_line(&self) -> String; // required — every implementor must define this
    fn display_type(&self) -> &str; // required

    // Default implementation — types get this for free, but can override it
    fn display_full(&self) -> String {
        format!("[{}] {}", self.display_type(), self.display_line())
    }
}

// =============================================================================
// 2. Implementing Traits for Multiple Types
// =============================================================================

#[derive(Debug, Clone, PartialEq)]
struct Train {
    number: u32,
    name: String,
    speed_kmh: u32,
}

#[derive(Debug, Clone, PartialEq)]
struct Station {
    name: String,
    code: String,
    platforms: u8,
}

#[derive(Debug, Clone, PartialEq)]
struct Route {
    from: String,
    to: String,
    distance_km: u32,
}

// Each type provides its own implementation of the trait
impl Displayable for Train {
    fn display_line(&self) -> String {
        format!("#{} {} ({}km/h)", self.number, self.name, self.speed_kmh)
    }

    fn display_type(&self) -> &str {
        "TRAIN"
    }
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

    // Override the default implementation
    fn display_full(&self) -> String {
        format!("🚉 {} ({})", self.name, self.code)
    }
}

impl Displayable for Route {
    fn display_line(&self) -> String {
        format!("{} → {} ({}km)", self.from, self.to, self.distance_km)
    }

    fn display_type(&self) -> &str {
        "ROUTE"
    }
}

// =============================================================================
// 3. The Display Trait — Custom Formatting with println!("{}", ...)
// =============================================================================

impl fmt::Display for Train {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} {}", self.number, self.name)
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

// =============================================================================
// 4. Traits as Parameters — "any type that implements Displayable"
// =============================================================================

// Short syntax: &impl Trait
fn print_board_entry(item: &impl Displayable) {
    println!("  {}", item.display_full());
}

// Full generic syntax (identical behavior)
#[allow(dead_code)]
fn print_board_entry_generic<T: Displayable>(item: &T) {
    println!("  {}", item.display_full());
}

// =============================================================================
// 5. Trait Bounds — Multiple constraints with +
// =============================================================================

// This function requires T to implement BOTH Displayable AND Display
fn announce<T: Displayable + fmt::Display>(item: &T) {
    println!("  Board: {}", item.display_full());
    println!("  Short: {}", item); // uses Display trait
}

// Same thing with a where clause (more readable when bounds get long)
fn announce_where<T>(item: &T)
where
    T: Displayable + fmt::Display,
{
    println!("  Board: {}", item.display_full());
    println!("  Short: {}", item);
}

// =============================================================================
// 6. Generics — Functions and Structs
// =============================================================================

// Generic function: works with any slice of Displayable items
fn print_all<T: Displayable>(items: &[T]) {
    for item in items {
        println!("  {}", item.display_full());
    }
}

// Generic function: search by checking display_line
fn find_by_name<'a, T: Displayable>(items: &'a [T], name: &str) -> Option<&'a T> {
    items.iter().find(|item| item.display_line().contains(name))
}

// Generic struct: holds any type
#[derive(Debug)]
struct Departure<T> {
    item: T,
    platform: u8,
    time: String,
}

impl<T: Displayable> Departure<T> {
    fn new(item: T, platform: u8, time: &str) -> Self {
        Departure {
            item,
            platform,
            time: time.to_string(),
        }
    }

    fn board_display(&self) -> String {
        format!(
            "{} | Platform {} | {}",
            self.item.display_full(),
            self.platform,
            self.time
        )
    }
}

// =============================================================================
// 7. The From/Into Traits — Type Conversion
// =============================================================================

// Convert a tuple into a Route
impl From<(&str, &str, u32)> for Route {
    fn from(tuple: (&str, &str, u32)) -> Self {
        Route {
            from: tuple.0.to_string(),
            to: tuple.1.to_string(),
            distance_km: tuple.2,
        }
    }
}

// Convert a &str code into a Station with defaults
impl From<&str> for Station {
    fn from(code: &str) -> Self {
        Station {
            name: format!("Station {}", code),
            code: code.to_string(),
            platforms: 1,
        }
    }
}

// =============================================================================
// 8. Deriving Common Traits
// =============================================================================

#[derive(Debug, Clone, PartialEq, Default)]
struct TicketCounter {
    first_class: u32,
    second_class: u32,
    sleeper: u32,
    general: u32,
}

impl TicketCounter {
    fn total(&self) -> u32 {
        self.first_class + self.second_class + self.sleeper + self.general
    }
}

impl fmt::Display for TicketCounter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "1A:{} 2A:{} SL:{} GN:{} (total: {})",
            self.first_class,
            self.second_class,
            self.sleeper,
            self.general,
            self.total()
        )
    }
}

// =============================================================================
// 9. Putting It All Together — Generic utility with trait bounds
// =============================================================================

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut biggest = &list[0];
    for item in &list[1..] {
        if item > biggest {
            biggest = item;
        }
    }
    biggest
}

fn main() {
    println!("=== LESSON 11: TRAITS & GENERICS ===\n");

    // --- Create our railway types ---
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
        },
        Route {
            from: "Chennai".to_string(),
            to: "Mumbai".to_string(),
            distance_km: 1280,
        },
        Route {
            from: "Chennai".to_string(),
            to: "Bangalore".to_string(),
            distance_km: 350,
        },
    ];

    // --- Section 2: Implementing Traits ---
    println!("--- Trait Implementations ---");
    for train in &trains {
        println!("  {}", train.display_line());
    }
    for station in &stations {
        println!("  {}", station.display_line());
    }
    println!();

    // --- Section 3: Default Implementations ---
    println!("--- Default vs Overridden display_full() ---");
    println!("  Train (default):   {}", trains[0].display_full());
    println!("  Station (custom):  {}", stations[0].display_full());
    println!("  Route (default):   {}", routes[0].display_full());
    println!();

    // --- Section 4: Traits as Parameters ---
    println!("--- Traits as Parameters (print_board_entry) ---");
    print_board_entry(&trains[0]);
    print_board_entry(&stations[0]);
    print_board_entry(&routes[0]);
    println!();

    // --- Section 5: Trait Bounds (multiple traits) ---
    println!("--- Trait Bounds (announce — needs Displayable + Display) ---");
    announce(&trains[1]);
    println!();
    println!("  With where clause:");
    announce_where(&stations[1]);
    println!();

    // --- Section 6: Generics ---
    println!("--- Generic print_all<T: Displayable> ---");
    println!("  Trains:");
    print_all(&trains);
    println!("  Routes:");
    print_all(&routes);
    println!();

    println!("--- Generic find_by_name ---");
    match find_by_name(&trains, "Vande") {
        Some(t) => println!("  Found: {}", t.display_line()),
        None => println!("  Not found"),
    }
    match find_by_name(&stations, "Mumbai") {
        Some(s) => println!("  Found: {}", s.display_line()),
        None => println!("  Not found"),
    }
    match find_by_name(&routes, "Kolkata") {
        Some(r) => println!("  Found: {}", r.display_line()),
        None => println!("  Not found: no route containing 'Kolkata'"),
    }
    println!();

    // --- Generic struct ---
    println!("--- Generic Departure<T> Struct ---");
    let dep1 = Departure::new(trains[0].clone(), 5, "06:00");
    let dep2 = Departure::new(stations[2].clone(), 1, "Platform info");
    println!("  {}", dep1.board_display());
    println!("  {}", dep2.board_display());
    println!();

    // --- Section 8: Display Trait ---
    println!("--- Display Trait (println! with {{}}) ---");
    println!("  {}", trains[0]);
    println!("  {}", stations[0]);
    println!("  {}", routes[0]);
    println!();

    // --- Section 9: From/Into Conversion ---
    println!("--- From/Into Trait ---");
    let route1 = Route::from(("Delhi", "Agra", 200));
    let route2: Route = ("Mumbai", "Pune", 150).into();
    println!("  From::from():  {}", route1);
    println!("  .into():       {}", route2);

    let station: Station = "BZA".into();
    println!(
        "  Station from code: {} ({} platforms)",
        station, station.platforms
    );
    println!();

    // --- Derived Traits ---
    println!("--- Derived Traits (Debug, Clone, PartialEq, Default) ---");
    let counter1 = TicketCounter {
        first_class: 50,
        second_class: 120,
        sleeper: 400,
        general: 800,
    };
    let counter2 = counter1.clone(); // Clone
    let default_counter = TicketCounter::default(); // Default

    println!("  Display: {}", counter1);
    println!("  Debug:   {:?}", counter1);
    println!("  Equal?   {}", counter1 == counter2); // PartialEq
    println!("  Default: {}", default_counter);
    println!();

    // --- Generic largest function ---
    println!("--- Generic largest<T: PartialOrd> ---");
    let distances = vec![2180, 1280, 350, 920, 460];
    let speeds = vec![130.0, 150.0, 160.0, 110.0];
    println!("  Longest distance: {}km", largest(&distances));
    println!("  Highest speed:    {}km/h", largest(&speeds));
    println!();

    println!("=== END OF EXAMPLES ===");
}
