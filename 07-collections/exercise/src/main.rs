// Lesson 07: Vec and Slices — Train Roster Manager (SOLUTION)

#[derive(Debug, Clone)]
struct Train {
    name: String,
    number: u32,
    speed_kmh: u32,
    is_active: bool,
}

impl Train {
    fn new(name: &str, number: u32, speed_kmh: u32, is_active: bool) -> Self {
        Train {
            name: name.to_string(),
            number,
            speed_kmh,
            is_active,
        }
    }
}

impl std::fmt::Display for Train {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "#{} {} — {} km/h",
            self.number, self.name, self.speed_kmh
        )
    }
}

// ---------------------------------------------------------------------------
// TODO 1: Create and populate a roster
// ---------------------------------------------------------------------------

fn build_roster() -> Vec<Train> {
    vec![
        Train::new("Rajdhani Express", 12001, 130, true),
        Train::new("Shatabdi Express", 12007, 150, true),
        Train::new("Duronto Express", 12245, 120, true),
        Train::new("Kovai Express", 12675, 110, true),
        Train::new("Island Express", 16525, 100, false),
        Train::new("Vande Bharat Express", 22436, 180, true),
    ]
}

// ---------------------------------------------------------------------------
// TODO 2: Find a train by number
// ---------------------------------------------------------------------------

fn find_by_number(roster: &[Train], number: u32) -> Option<&Train> {
    for train in roster {
        if train.number == number {
            return Some(train);
        }
    }
    None
}

// ---------------------------------------------------------------------------
// TODO 3: Get active trains
// ---------------------------------------------------------------------------

fn active_trains(roster: &[Train]) -> Vec<&Train> {
    let mut result = Vec::new();
    for train in roster {
        if train.is_active {
            result.push(train);
        }
    }
    result
}

// ---------------------------------------------------------------------------
// TODO 4: Sort and display
// ---------------------------------------------------------------------------

fn display_sorted_by_speed(roster: &mut Vec<Train>) {
    roster.sort_by(|a, b| b.speed_kmh.cmp(&a.speed_kmh));
    for train in roster.iter() {
        println!("  {}", train);
    }
}

// ---------------------------------------------------------------------------
// TODO 5: Deactivate slow trains and clean up
// ---------------------------------------------------------------------------

fn deactivate_slow_trains(roster: &mut Vec<Train>, min_speed: u32) {
    // Phase 1: Deactivate
    for train in roster.iter_mut() {
        if train.speed_kmh < min_speed {
            println!(
                "  Deactivated: {} ({} km/h < {} km/h)",
                train.name, train.speed_kmh, min_speed
            );
            train.is_active = false;
        }
    }

    // Phase 2: Remove inactive
    let inactive_count = roster.iter().filter(|t| !t.is_active).count();
    roster.retain(|t| t.is_active);
    println!("  Removed {} inactive trains.", inactive_count);
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

fn main() {
    println!("=== TRAIN ROSTER MANAGER ===");

    // Build the roster
    let mut roster = build_roster();

    // Display full roster
    println!("\n--- Full Roster ({} trains) ---", roster.len());
    for train in &roster {
        let status = if train.is_active {
            "active"
        } else {
            "inactive"
        };
        println!("  {} [{}]", train, status);
    }

    // Find trains by number
    println!("\n--- Find Train ---");
    match find_by_number(&roster, 12007) {
        Some(train) => println!(
            "  Looking for #12007: Found! {} ({} km/h)",
            train.name, train.speed_kmh
        ),
        None => println!("  Looking for #12007: Not found"),
    }
    match find_by_number(&roster, 99999) {
        Some(train) => println!(
            "  Looking for #99999: Found! {} ({} km/h)",
            train.name, train.speed_kmh
        ),
        None => println!("  Looking for #99999: Not found"),
    }

    // Active trains only
    println!("\n--- Active Trains ---");
    let active = active_trains(&roster);
    for train in &active {
        println!("  {}", train);
    }

    // Sort by speed
    println!("\n--- Sorted by Speed (fastest first) ---");
    display_sorted_by_speed(&mut roster);

    // Deactivate and clean up
    println!("\n--- Deactivate & Clean Up (min speed: 115 km/h) ---");
    deactivate_slow_trains(&mut roster, 115);
    println!("  Remaining roster ({} trains):", roster.len());
    for train in &roster {
        let status = if train.is_active {
            "active"
        } else {
            "inactive"
        };
        println!("    {} [{}]", train, status);
    }
}
