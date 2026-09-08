// =============================================
// SOLUTION — Don't peek until you've tried!
// =============================================

// TODO 1
#[derive(Debug, Clone)]
struct Train {
    name: String,
    number: u32,
    speed_kmh: u32,
    is_active: bool,
}

// TODO 2
impl Train {
    fn new(name: &str, number: u32, speed_kmh: u32, is_active: bool) -> Self {
        Train {
            name: name.to_string(),
            number,
            speed_kmh,
            is_active,
        }
    }

    fn display(&self) -> String {
        format!(
            "#{} {} \u{2014} {} km/h",
            self.number, self.name, self.speed_kmh
        )
    }
}

// TODO 3
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

// TODO 4
fn find_by_number(roster: &[Train], number: u32) -> Option<&Train> {
    for train in roster {
        if train.number == number {
            return Some(train);
        }
    }
    None
}

// TODO 5
fn active_trains(roster: &[Train]) -> Vec<&Train> {
    let mut result = Vec::new();
    for train in roster {
        if train.is_active {
            result.push(train);
        }
    }
    result
}

// TODO 6
fn display_sorted_by_speed(roster: &mut Vec<Train>) {
    roster.sort_by(|a, b| b.speed_kmh.cmp(&a.speed_kmh));
    for train in roster.iter() {
        println!("  {}", train.display());
    }
}

// TODO 7
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

fn main() {
    println!("=== TRAIN ROSTER MANAGER ===");

    let mut roster = build_roster();

    println!("\n--- Full Roster ({} trains) ---", roster.len());
    for train in &roster {
        let status = if train.is_active {
            "active"
        } else {
            "inactive"
        };
        println!("  {} [{}]", train.display(), status);
    }

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

    println!("\n--- Active Trains ---");
    let active = active_trains(&roster);
    for train in &active {
        println!("  {}", train.display());
    }

    println!("\n--- Sorted by Speed (fastest first) ---");
    display_sorted_by_speed(&mut roster);

    println!("\n--- Deactivate & Clean Up (min speed: 115 km/h) ---");
    deactivate_slow_trains(&mut roster, 115);
    println!("  Remaining roster ({} trains):", roster.len());
    for train in &roster {
        let status = if train.is_active {
            "active"
        } else {
            "inactive"
        };
        println!("    {} [{}]", train.display(), status);
    }
}
