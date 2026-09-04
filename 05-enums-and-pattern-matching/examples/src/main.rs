// =============================================================================
// Lesson 05: Enums and Pattern Matching 🚂
// =============================================================================
// This file demonstrates enums and pattern matching in Rust using railway themes.
// Topics: basic enums, enums with data, match, match guards, methods on enums,
//         and composing enums with structs.
//
// Run with: cargo run
// =============================================================================

// ---------------------------------------------------------------------------
// Common type used throughout the examples
// ---------------------------------------------------------------------------

/// The operational status of a train.
#[derive(Debug, Clone)]
enum TrainStatus {
    OnTime,
    Delayed(u32),      // delay in minutes
    Cancelled(String), // reason for cancellation
}

fn main() {
    section_1_basic_enums();
    section_2_enums_with_data();
    section_3_match_guards();
    section_4_methods_on_enums();
    section_5_enums_and_structs();
}

// =============================================================================
// 1. Basic Enum with Match
// =============================================================================
// TrainStatus has three variants. `match` forces you to handle every one.

fn section_1_basic_enums() {
    println!("\n\u{1f682} \u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}");
    println!("   1. BASIC ENUMS WITH MATCH");
    println!("   \u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\n");

    // Create instances of each variant
    let status1 = TrainStatus::OnTime;
    let status2 = TrainStatus::Delayed(15);
    let status3 = TrainStatus::Cancelled(String::from("Severe weather warning"));

    // Match on each status — extract data from variants
    let statuses = [status1, status2, status3];
    for status in &statuses {
        let message = match status {
            TrainStatus::OnTime => String::from("\u{2705} Train is on time"),
            TrainStatus::Delayed(mins) => format!("\u{23f0} Delayed by {} minutes", mins),
            TrainStatus::Cancelled(reason) => format!("\u{274c} Cancelled: {}", reason),
        };
        println!("   {:?} \u{2192} {}", status, message);
    }

    // The _ catch-all pattern — matches anything not already matched
    let mystery_status = TrainStatus::Delayed(5);
    match mystery_status {
        TrainStatus::Cancelled(_) => println!("   Won't run today."),
        _ => println!("\n   \u{1f514} Train is still expected to run (catch-all matched)."),
    }
}

// =============================================================================
// 2. Enums with Mixed Data Variants
// =============================================================================
// Each variant can carry different types and amounts of data.

#[derive(Debug)]
enum TrainEvent {
    Departed,                                         // unit variant — no data
    Arrived { station: String, platform: u8 },        // struct-like — named fields
    Delayed(u32),                                     // tuple-like — one value
    SpeedChanged(f64, f64),                           // tuple-like — two values
    Cancelled { reason: String, refund_percent: u8 }, // struct-like — named fields
}

fn section_2_enums_with_data() {
    println!("\n\u{1f682} \u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}");
    println!("   2. ENUMS WITH MIXED DATA VARIANTS");
    println!("   \u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\n");

    // Create a list of events — each variant carries different data
    let events = vec![
        TrainEvent::Departed,
        TrainEvent::SpeedChanged(0.0, 80.5),
        TrainEvent::Delayed(12),
        TrainEvent::SpeedChanged(80.5, 120.0),
        TrainEvent::Arrived {
            station: String::from("Bangalore City"),
            platform: 4,
        },
        TrainEvent::Cancelled {
            reason: String::from("Track damage ahead"),
            refund_percent: 100,
        },
    ];

    println!("   Processing {} events:\n", events.len());
    for (i, event) in events.iter().enumerate() {
        // match extracts the right data from each variant
        let description = match event {
            TrainEvent::Departed => String::from("\u{1f682} Train has departed"),
            TrainEvent::Arrived { station, platform } => {
                format!("\u{1f3c1} Arrived at {} on platform {}", station, platform)
            }
            TrainEvent::Delayed(mins) => format!("\u{23f0} Delayed by {} minutes", mins),
            TrainEvent::SpeedChanged(old, new) => {
                format!("\u{1f4a8} Speed: {:.1} \u{2192} {:.1} km/h", old, new)
            }
            TrainEvent::Cancelled {
                reason,
                refund_percent,
            } => {
                format!(
                    "\u{274c} Cancelled ({}% refund): {}",
                    refund_percent, reason
                )
            }
        };
        println!("   Event {}: {}", i + 1, description);
    }
}

// =============================================================================
// 3. Match Guards
// =============================================================================
// Add `if` conditions to match arms for finer control.

fn section_3_match_guards() {
    println!("\n\u{1f682} \u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}");
    println!("   3. MATCH GUARDS");
    println!("   \u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\n");

    let statuses = vec![
        TrainStatus::OnTime,
        TrainStatus::Delayed(5),
        TrainStatus::Delayed(25),
        TrainStatus::Delayed(90),
        TrainStatus::Cancelled(String::from("Flood")),
    ];

    // Match guards let you add conditions to match arms
    for status in &statuses {
        let urgency = match status {
            TrainStatus::Delayed(mins) if *mins < 10 => "Minor delay, no action needed",
            TrainStatus::Delayed(mins) if *mins < 30 => "Moderate delay, inform passengers",
            TrainStatus::Delayed(mins) if *mins < 60 => "Major delay, offer refreshments",
            TrainStatus::Delayed(_) => "Severe delay, arrange alternate transport",
            TrainStatus::Cancelled(_) => "CANCELLED \u{2014} full refund process",
            TrainStatus::OnTime => "All clear",
        };
        println!("   {:?} \u{2192} {}", status, urgency);
    }

    // Match is an expression — it returns a value
    println!("\n   Match as expression:");
    let delay = TrainStatus::Delayed(42);
    let emoji = match &delay {
        TrainStatus::OnTime => "\u{1f7e2}",
        TrainStatus::Delayed(_) => "\u{1f7e1}",
        TrainStatus::Cancelled(_) => "\u{1f534}",
    };
    println!("   Status emoji: {}", emoji);
}

// =============================================================================
// 4. Methods on Enums
// =============================================================================
// Enums can have `impl` blocks, just like structs.

impl TrainStatus {
    fn description(&self) -> String {
        match self {
            TrainStatus::OnTime => String::from("On time"),
            TrainStatus::Delayed(mins) => format!("Delayed by {} minutes", mins),
            TrainStatus::Cancelled(reason) => format!("Cancelled: {}", reason),
        }
    }

    fn is_running(&self) -> bool {
        match self {
            TrainStatus::OnTime | TrainStatus::Delayed(_) => true,
            TrainStatus::Cancelled(_) => false,
        }
    }

    fn emoji(&self) -> &str {
        match self {
            TrainStatus::OnTime => "\u{1f7e2}",
            TrainStatus::Delayed(_) => "\u{1f7e1}",
            TrainStatus::Cancelled(_) => "\u{1f534}",
        }
    }
}

fn section_4_methods_on_enums() {
    println!("\n\u{1f682} \u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}");
    println!("   4. METHODS ON ENUMS");
    println!("   \u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\n");

    let statuses = vec![
        TrainStatus::OnTime,
        TrainStatus::Delayed(20),
        TrainStatus::Cancelled(String::from("Staff shortage")),
    ];

    for status in &statuses {
        println!(
            "   {} {} (running: {})",
            status.emoji(),
            status.description(),
            status.is_running()
        );
    }
}

// =============================================================================
// 5. Enum as a Struct Field — Departure Board
// =============================================================================
// Compose enums with structs for rich data models.

#[derive(Debug)]
struct Train {
    name: String,
    number: u32,
    status: TrainStatus,
}

impl Train {
    fn new(name: &str, number: u32, status: TrainStatus) -> Self {
        Train {
            name: name.to_string(),
            number,
            status,
        }
    }

    fn board_display(&self) {
        println!(
            "   {} {:>5} | {:<20} | {}",
            self.status.emoji(),
            self.number,
            self.name,
            self.status.description()
        );
    }
}

fn section_5_enums_and_structs() {
    println!("\n\u{1f682} \u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}");
    println!("   5. ENUM AS STRUCT FIELD \u{2014} DEPARTURE BOARD");
    println!("   \u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\n");

    let trains = vec![
        Train::new("Rajdhani Express", 12301, TrainStatus::OnTime),
        Train::new("Shatabdi Express", 12007, TrainStatus::Delayed(15)),
        Train::new(
            "Chennai Express",
            12163,
            TrainStatus::Cancelled(String::from("Flooding")),
        ),
        Train::new("Duronto Express", 12245, TrainStatus::OnTime),
        Train::new("Garib Rath", 12578, TrainStatus::Delayed(45)),
    ];

    println!("   \u{250c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}");
    println!("   \u{2502}            CHENNAI CENTRAL \u{2014} DEPARTURES             \u{2502}");
    println!("   \u{251c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2524}");
    for train in &trains {
        train.board_display();
    }
    println!("   \u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2518}");

    // Count running vs. non-running trains
    let running = trains.iter().filter(|t| t.status.is_running()).count();
    println!(
        "\n   {} out of {} trains are running.",
        running,
        trains.len()
    );
}
