// =============================================
// EXERCISE 05: Train Status Dashboard
// =============================================
//
// Build a train status dashboard using enums and pattern matching.
// Complete all the TODOs so the program compiles and produces the expected output.
//
// Run with: cargo run
//
// Expected output:
//
//   === TRAIN STATUS DASHBOARD ===
//
//   --- All Trains ---
//   🟢 Rajdhani Express (#12301) [Express] - On time
//   🟡 Shatabdi Express (#12007) [Superfast] - Delayed by 45 minutes: Signal failure at junction
//   🔴 Chennai Express (#12163) [Superfast] - Cancelled: Track maintenance
//   🔵 Nilgiri Express (#12671) [Passenger] - Arrived at platform 3
//   🚂 Mumbai Local (#90123) [Local] - Departed, next station: Dadar
//   🟡 Garib Rath (#12578) [Express] - Delayed by 15 minutes: Waiting for connecting train
//   🟢 Goods Special (#99001) [Freight] - On time
//   🟡 Duronto Express (#12245) [Superfast] - Delayed by 90 minutes: Heavy rainfall
//
//   --- Train Type Info ---
//   Express: max speed 160 km/h, priority 1
//   Superfast: max speed 130 km/h, priority 2
//   Passenger: max speed 110 km/h, priority 3
//   Local: max speed 80 km/h, priority 4
//   Freight: max speed 60 km/h, priority 5
//
//   --- Running Trains ---
//   🟢 Rajdhani Express (#12301) is running
//   🟡 Shatabdi Express (#12007) is running
//   🚂 Mumbai Local (#90123) is running
//   🟡 Garib Rath (#12578) is running
//   🟢 Goods Special (#99001) is running
//   🟡 Duronto Express (#12245) is running
//
//   --- Trains Delayed Over 30 Minutes ---
//   🟡 Shatabdi Express (#12007) [Superfast] - Delayed by 45 minutes: Signal failure at junction
//   🟡 Duronto Express (#12245) [Superfast] - Delayed by 90 minutes: Heavy rainfall
//
//   --- Status Summary ---
//   On Time: 2
//   Delayed: 3
//   Cancelled: 1
//   Arrived: 1
//   Departed: 1

// =============================================
// TODO 1: Define the `TrainType` enum
// =============================================
//
// Variants (no associated data):
//   Express, Superfast, Passenger, Local, Freight
//
// Derive: Debug, Clone, PartialEq

// Your code here
#[derive(Debug, Clone, PartialEq)]
enum TrainType {
    Express,
    Superfast,
    Clone,
    PartialEq,
}

// =============================================
// TODO 2: Implement methods on `TrainType`
// =============================================
//
// Use `match` on `self` for each method.
//
// - fn display_name(&self) -> &str
//     Express → "Express", Superfast → "Superfast",
//     Passenger → "Passenger", Local → "Local", Freight → "Freight"
//
// - fn max_speed(&self) -> u32
//     Express: 160, Superfast: 130, Passenger: 110, Local: 80, Freight: 60
//
// - fn priority(&self) -> u8
//     Express: 1, Superfast: 2, Passenger: 3, Local: 4, Freight: 5

impl TrainType {
    fn display_name(&self) -> &str {
        // TODO: match on self and return the display name
        ""
    }

    fn max_speed(&self) -> u32 {
        // TODO: match on self and return the max speed
        0
    }

    fn priority(&self) -> u8 {
        // TODO: match on self and return the priority
        0
    }
}

// =============================================
// TODO 3: Define the `TrainStatus` enum
// =============================================
//
// This enum has DATA attached to its variants!
//
// Variants:
//   - OnTime                                  ← unit variant (no data)
//   - Delayed { minutes: u32, reason: String } ← struct-like
//   - Cancelled { reason: String }            ← struct-like
//   - Arrived { platform: u8 }                ← struct-like
//   - Departed { next_station: String }       ← struct-like
//
// Derive: Debug, Clone

// Your code here

// =============================================
// TODO 4: Implement methods on `TrainStatus`
// =============================================
//
// - fn emoji(&self) -> &str
//     OnTime → "🟢", Delayed → "🟡", Cancelled → "🔴",
//     Arrived → "🔵", Departed → "🚂"
//
// - fn description(&self) -> String
//     OnTime                       → "On time"
//     Delayed { minutes, reason }  → "Delayed by {minutes} minutes: {reason}"
//     Cancelled { reason }         → "Cancelled: {reason}"
//     Arrived { platform }         → "Arrived at platform {platform}"
//     Departed { next_station }    → "Departed, next station: {next_station}"
//
//   Hint: use format!() for variants with data, String::from() for OnTime.
//
// - fn is_running(&self) -> bool
//     true for: OnTime, Delayed, Departed
//     false for: Cancelled, Arrived
//
//   Hint: you can combine patterns with | like:
//     TrainStatus::OnTime | TrainStatus::Delayed { .. } => true,

impl TrainStatus {
    fn emoji(&self) -> &str {
        // TODO: match on self and return the emoji
        ""
    }

    fn description(&self) -> String {
        // TODO: match on self, extract data, return formatted description
        String::new()
    }

    fn is_running(&self) -> bool {
        // TODO: match on self, return true for running states
        false
    }
}

// =============================================
// TODO 5: Define the `Train` struct
// =============================================
//
// Fields:
//   - name: String
//   - number: u32
//   - train_type: TrainType
//   - status: TrainStatus
//
// Derive: Debug

// Your code here

// =============================================
// TODO 6: Implement methods on `Train`
// =============================================
//
// Associated function:
//   - fn new(name: &str, number: u32, train_type: TrainType, status: TrainStatus) -> Self
//     Convert name to String using .to_string()
//
// Methods:
//   - fn display(&self)
//     Print one line:
//       {emoji} {name} (#{number}) [{type_name}] - {description}
//     Example:
//       🟢 Rajdhani Express (#12301) [Express] - On time
//
//   - fn is_delayed_over(&self, threshold: u32) -> bool
//     Returns true ONLY if status is Delayed AND minutes > threshold.
//     Use match on &self.status. For all other variants, return false.

impl Train {
    fn new(name: &str, number: u32, train_type: TrainType, status: TrainStatus) -> Self {
        // TODO: create and return a Train with these fields
        // Hint: convert name to String with .to_string()
        todo!()
    }

    fn display(&self) {
        // TODO: print formatted train info using emoji(), display_name(), description()
        // Format: "{emoji} {name} (#{number}) [{type_name}] - {description}"
    }

    fn is_delayed_over(&self, threshold: u32) -> bool {
        // TODO: match on &self.status
        // If Delayed { minutes, .. } and *minutes > threshold → true
        // All other variants → false
        false
    }
}

// =============================================
// TODO 7: Implement `count_by_status`
// =============================================
//
// fn count_by_status(trains: &[Train])
//
// Loop through all trains, count how many are in each status category.
// Use `match` on &train.status (borrow to avoid moving).
//
// Print:
//   --- Status Summary ---
//   On Time: 2
//   Delayed: 3
//   Cancelled: 1
//   Arrived: 1
//   Departed: 1
//
// Hint: use `..` to ignore fields in struct-like variants:
//   TrainStatus::Delayed { .. } => delayed += 1,

fn count_by_status(trains: &[Train]) {
    // TODO: count trains in each status category and print the summary
}

// =============================================
// TODO 8: Implement `print_delayed_trains`
// =============================================
//
// fn print_delayed_trains(trains: &[Train], min_delay: u32)
//
// Print a header: "--- Trains Delayed Over {min_delay} Minutes ---"
// Then loop through trains, and for each train where
// is_delayed_over(min_delay) is true, call train.display().

fn print_delayed_trains(trains: &[Train], min_delay: u32) {
    // TODO: print header, then display each train delayed over min_delay minutes
}

// =============================================
// main() — DO NOT EDIT BELOW THIS LINE
// =============================================

fn main() {
    println!("=== TRAIN STATUS DASHBOARD ===");

    // --- Create trains ---
    let trains = vec![
        Train::new(
            "Rajdhani Express",
            12301,
            TrainType::Express,
            TrainStatus::OnTime,
        ),
        Train::new(
            "Shatabdi Express",
            12007,
            TrainType::Superfast,
            TrainStatus::Delayed {
                minutes: 45,
                reason: String::from("Signal failure at junction"),
            },
        ),
        Train::new(
            "Chennai Express",
            12163,
            TrainType::Superfast,
            TrainStatus::Cancelled {
                reason: String::from("Track maintenance"),
            },
        ),
        Train::new(
            "Nilgiri Express",
            12671,
            TrainType::Passenger,
            TrainStatus::Arrived { platform: 3 },
        ),
        Train::new(
            "Mumbai Local",
            90123,
            TrainType::Local,
            TrainStatus::Departed {
                next_station: String::from("Dadar"),
            },
        ),
        Train::new(
            "Garib Rath",
            12578,
            TrainType::Express,
            TrainStatus::Delayed {
                minutes: 15,
                reason: String::from("Waiting for connecting train"),
            },
        ),
        Train::new(
            "Goods Special",
            99001,
            TrainType::Freight,
            TrainStatus::OnTime,
        ),
        Train::new(
            "Duronto Express",
            12245,
            TrainType::Superfast,
            TrainStatus::Delayed {
                minutes: 90,
                reason: String::from("Heavy rainfall"),
            },
        ),
    ];

    // --- Display all trains ---
    println!("\n--- All Trains ---");
    for train in &trains {
        train.display();
    }

    // --- Train type info ---
    println!("\n--- Train Type Info ---");
    let types = [
        TrainType::Express,
        TrainType::Superfast,
        TrainType::Passenger,
        TrainType::Local,
        TrainType::Freight,
    ];
    for t in &types {
        println!(
            "{}: max speed {} km/h, priority {}",
            t.display_name(),
            t.max_speed(),
            t.priority()
        );
    }

    // --- Running trains ---
    println!("\n--- Running Trains ---");
    for train in &trains {
        if train.status.is_running() {
            println!(
                "{} {} (#{}) is running",
                train.status.emoji(),
                train.name,
                train.number
            );
        }
    }

    // --- Delayed trains ---
    println!();
    print_delayed_trains(&trains, 30);

    // --- Status summary ---
    println!();
    count_by_status(&trains);
}
