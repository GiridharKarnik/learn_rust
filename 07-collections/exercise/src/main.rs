// =============================================
// EXERCISE 07: Train Roster Manager
// =============================================
//
// Build a complete train roster system from scratch.
// There is NO code provided — you write everything, including main().
//
// This exercises: structs, methods, Option, Vec, slices, sorting, retain.
//
// Run with: cargo run
//
// Expected output:
//
//   === TRAIN ROSTER MANAGER ===
//
//   --- Full Roster (6 trains) ---
//     #12001 Rajdhani Express — 130 km/h [active]
//     #12007 Shatabdi Express — 150 km/h [active]
//     #12245 Duronto Express — 120 km/h [active]
//     #12675 Kovai Express — 110 km/h [active]
//     #16525 Island Express — 100 km/h [inactive]
//     #22436 Vande Bharat Express — 180 km/h [active]
//
//   --- Find Train ---
//     Looking for #12007: Found! Shatabdi Express (150 km/h)
//     Looking for #99999: Not found
//
//   --- Active Trains ---
//     #12001 Rajdhani Express — 130 km/h
//     #12007 Shatabdi Express — 150 km/h
//     #12245 Duronto Express — 120 km/h
//     #12675 Kovai Express — 110 km/h
//     #22436 Vande Bharat Express — 180 km/h
//
//   --- Sorted by Speed (fastest first) ---
//     #22436 Vande Bharat Express — 180 km/h
//     #12007 Shatabdi Express — 150 km/h
//     #12001 Rajdhani Express — 130 km/h
//     #12245 Duronto Express — 120 km/h
//     #12675 Kovai Express — 110 km/h
//     #16525 Island Express — 100 km/h
//
//   --- Deactivate & Clean Up (min speed: 115 km/h) ---
//     Deactivated: Kovai Express (110 km/h < 115 km/h)
//     Deactivated: Island Express (100 km/h < 115 km/h)
//     Removed 2 inactive trains.
//     Remaining roster (4 trains):
//       #22436 Vande Bharat Express — 180 km/h [active]
//       #12007 Shatabdi Express — 150 km/h [active]
//       #12001 Rajdhani Express — 130 km/h [active]
//       #12245 Duronto Express — 120 km/h [active]

// =============================================
// STEP 1: Define a `Train` struct
// =============================================
// Fields: name (String), number (u32), speed_kmh (u32), is_active (bool)
// Derive: Debug, Clone

struct Train {
    name: String,
    number: u32,
    speed_kmh: u32,
    is_active: bool,
}

// =============================================
// STEP 2: Implement `Train` methods
// =============================================
// - fn new(name: &str, number: u32, speed_kmh: u32, is_active: bool) -> Self
// - fn display(&self) -> String
//     Returns: "#{number} {name} — {speed_kmh} km/h"
//     Example: "#12001 Rajdhani Express — 130 km/h"

impl Train {
    fn new(name: &str, number: u32, speed_kmh: u32, is_active: bool) -> Self {
        return Train {
            name: String::from(name),
            number,
            speed_kmh,
            is_active,
        };
    }

    fn display(&self) -> String {
        return format!("#{} {} - {} km/h", self.number, self.name, self.speed_kmh);
    }
}

// =============================================
// STEP 3: Implement `build_roster() -> Vec<Train>`
// =============================================
// Return a vec! with these 6 trains:
//   ("Rajdhani Express",      12001, 130, true)
//   ("Shatabdi Express",      12007, 150, true)
//   ("Duronto Express",       12245, 120, true)
//   ("Kovai Express",         12675, 110, true)
//   ("Island Express",        16525, 100, false)   ← starts inactive
//   ("Vande Bharat Express",  22436, 180, true)

fn build_roster() -> Vec<Train> {
    return vec![
        Train {
            name: String::from("Rajdhani Express"),
            number: 12001,
            speed_kmh: 130,
            is_active: true,
        },
        Train {
            name: String::from("Shatabdi Express"),
            number: 12007,
            speed_kmh: 150,
            is_active: true,
        },
        Train {
            name: String::from("Duronto Express"),
            number: 12245,
            speed_kmh: 120,
            is_active: true,
        },
        Train {
            name: String::from("Kovai Express"),
            number: 12675,
            speed_kmh: 110,
            is_active: true,
        },
        Train {
            name: String::from("Island Express"),
            number: 16525,
            speed_kmh: 100,
            is_active: false,
        },
        Train {
            name: String::from("Duronto Express"),
            number: 22436,
            speed_kmh: 180,
            is_active: true,
        },
    ];
}

// =============================================
// STEP 4: Implement `find_by_number`
// =============================================
// fn find_by_number(roster: &[Train], number: u32) -> Option<&Train>
//
// Search the roster for a train matching the number.
// Return Some(&train) if found, None if not.
// Note: parameter is &[Train] (slice), not &Vec<Train>.

fn find_by_number(roster: &[Train], number: u32) -> Option<&Train> {
    return roster.iter().find(|train| train.number == number);
}

// =============================================
// STEP 5: Implement `active_trains`
// =============================================
// fn active_trains(roster: &[Train]) -> Vec<&Train>
//
// Return a new Vec with references to only the active trains.
// Use a for loop: push matching trains into a new Vec.

fn active_trains(roster: &[Train]) -> Vec<&Train> {
    return roster
        .iter()
        .filter(|train| train.is_active == true)
        .collect();
}

// =============================================
// STEP 6: Implement `display_sorted_by_speed`
// =============================================
// fn display_sorted_by_speed(roster: &mut Vec<Train>)
//
// Sort the roster by speed_kmh DESCENDING (fastest first), then print each.
// Hint: roster.sort_by(|a, b| b.speed_kmh.cmp(&a.speed_kmh))
// Then loop and print: "  {train.display()}"

fn display_sorted_by_speed(roster: &mut Vec<Train>) {
    roster.sort_by(|a, b| b.speed_kmh.cmp(&a.speed_kmh));

    roster
        .iter()
        .for_each(|train| println!("{} [{} km/hr]", train.name, train.speed_kmh));
}

// =============================================
// STEP 7: Implement `deactivate_slow_trains`
// =============================================
// fn deactivate_slow_trains(roster: &mut Vec<Train>, min_speed: u32)
//
// Phase 1 — Deactivate: use .iter_mut() to loop and set is_active = false
//   for any train below min_speed. Print each one:
//   "  Deactivated: {name} ({speed} km/h < {min_speed} km/h)"
//
// Phase 2 — Remove: count inactive trains, then roster.retain(|t| t.is_active)
//   Print: "  Removed {count} inactive trains."

fn deactivate_slow_trains(roster: &mut Vec<Train>, min_speed: u32) {
    roster.iter_mut().for_each(|train| {
        if train.speed_kmh < min_speed {
            train.is_active = false
        }
    });
}

// =============================================
// STEP 8: Write `fn main()`
// =============================================
// Build the roster, then:
//
// 1. Print "=== TRAIN ROSTER MANAGER ==="
//
// 2. Print "--- Full Roster ({count} trains) ---"
//    Loop through &roster, print each: "  {display} [{status}]"
//    where status is "active" or "inactive"
//

//

//

//

fn main() {
    println!("=== TRAIN ROSTER MANAGER ===");

    let mut full_train_roster = build_roster();
    println!(
        "--- Full Roster ({} trains) ---",
        full_train_roster.iter().len()
    );

    for train in &full_train_roster {
        let status = if train.is_active {
            "active"
        } else {
            "inactive"
        };

        println!("  {} [{}]", train.display(), status);
    }

    // 3. Print "--- Find Train ---"
    //    Search for #12007 — print found message or "Not found"
    //    Search for #99999 — print found message or "Not found"
    //    Use match on find_by_number()
    println!("--- Find Train ---");

    let train_12007 = find_by_number(&full_train_roster, 12007);

    match train_12007 {
        Some(train) => println!("Found {} ({}) km/hr", train.name, train.speed_kmh),
        None => println!("Not found"),
    }

    // 4. Print "--- Active Trains ---"
    //    Call active_trains(), loop and print each
    println!("--- Active Trains ---");

    let active_trains = active_trains(&full_train_roster);

    active_trains.iter().for_each(|active_train| {
        print!("{} [{} km/hr]", active_train.name, active_train.speed_kmh);
    });

    // 5. Print "--- Sorted by Speed (fastest first) ---"
    //    Call display_sorted_by_speed()
    println!("--- Sorted by Speed (fastest first) ---");
    display_sorted_by_speed(&mut full_train_roster);

    // 6. Print "--- Deactivate & Clean Up (min speed: 115 km/h) ---"
    //    Call deactivate_slow_trains() with min_speed 115
    //    Print remaining roster with count and status
    println!("--- Deactivate & Clean Up (min speed: 115 km/h) ---");
    deactivate_slow_trains(&mut full_train_roster, 115);

    println!("{} fast trains available", full_train_roster.iter().count());
    full_train_roster.iter().for_each(|t: &Train| {
        if t.is_active {
            println!("fast train {}, [{} km/hr]", t.name, t.speed_kmh);
        }
    });
}
