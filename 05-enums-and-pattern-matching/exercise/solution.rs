// ============================================================================
// LESSON 05 EXERCISE — SOLUTION: Train Status Dashboard
// ============================================================================

// TODO 1: Define the `TrainType` enum
#[derive(Debug, Clone, PartialEq)]
enum TrainType {
    Express,
    Superfast,
    Passenger,
    Local,
    Freight,
}

// TODO 2: Implement methods on `TrainType`
impl TrainType {
    fn display_name(&self) -> &str {
        match self {
            TrainType::Express => "Express",
            TrainType::Superfast => "Superfast",
            TrainType::Passenger => "Passenger",
            TrainType::Local => "Local",
            TrainType::Freight => "Freight",
        }
    }

    fn max_speed(&self) -> u32 {
        match self {
            TrainType::Express => 160,
            TrainType::Superfast => 130,
            TrainType::Passenger => 110,
            TrainType::Local => 80,
            TrainType::Freight => 60,
        }
    }

    fn priority(&self) -> u8 {
        match self {
            TrainType::Express => 1,
            TrainType::Superfast => 2,
            TrainType::Passenger => 3,
            TrainType::Local => 4,
            TrainType::Freight => 5,
        }
    }
}

// TODO 3: Define the `TrainStatus` enum with data
#[derive(Debug, Clone)]
enum TrainStatus {
    OnTime,
    Delayed { minutes: u32, reason: String },
    Cancelled { reason: String },
    Arrived { platform: u8 },
    Departed { next_station: String },
}

// TODO 4: Implement methods on `TrainStatus`
impl TrainStatus {
    fn emoji(&self) -> &str {
        match self {
            TrainStatus::OnTime => "\u{1f7e2}",
            TrainStatus::Delayed { .. } => "\u{1f7e1}",
            TrainStatus::Cancelled { .. } => "\u{1f534}",
            TrainStatus::Arrived { .. } => "\u{1f535}",
            TrainStatus::Departed { .. } => "\u{1f682}",
        }
    }

    fn description(&self) -> String {
        match self {
            TrainStatus::OnTime => String::from("On time"),
            TrainStatus::Delayed { minutes, reason } => {
                format!("Delayed by {} minutes: {}", minutes, reason)
            }
            TrainStatus::Cancelled { reason } => {
                format!("Cancelled: {}", reason)
            }
            TrainStatus::Arrived { platform } => {
                format!("Arrived at platform {}", platform)
            }
            TrainStatus::Departed { next_station } => {
                format!("Departed, next station: {}", next_station)
            }
        }
    }

    fn is_running(&self) -> bool {
        match self {
            TrainStatus::OnTime | TrainStatus::Delayed { .. } | TrainStatus::Departed { .. } => {
                true
            }
            TrainStatus::Cancelled { .. } | TrainStatus::Arrived { .. } => false,
        }
    }
}

// TODO 5: Define the `Train` struct
#[derive(Debug)]
struct Train {
    name: String,
    number: u32,
    train_type: TrainType,
    status: TrainStatus,
}

// TODO 6: Implement methods on `Train`
impl Train {
    fn new(name: &str, number: u32, train_type: TrainType, status: TrainStatus) -> Self {
        Train {
            name: name.to_string(),
            number,
            train_type,
            status,
        }
    }

    fn display(&self) {
        println!(
            "{} {} (#{}) [{}] - {}",
            self.status.emoji(),
            self.name,
            self.number,
            self.train_type.display_name(),
            self.status.description()
        );
    }

    fn is_delayed_over(&self, threshold: u32) -> bool {
        match &self.status {
            TrainStatus::Delayed { minutes, .. } => *minutes > threshold,
            _ => false,
        }
    }
}

// TODO 7: Implement `count_by_status`
fn count_by_status(trains: &[Train]) {
    let mut on_time = 0;
    let mut delayed = 0;
    let mut cancelled = 0;
    let mut arrived = 0;
    let mut departed = 0;

    for train in trains {
        match &train.status {
            TrainStatus::OnTime => on_time += 1,
            TrainStatus::Delayed { .. } => delayed += 1,
            TrainStatus::Cancelled { .. } => cancelled += 1,
            TrainStatus::Arrived { .. } => arrived += 1,
            TrainStatus::Departed { .. } => departed += 1,
        }
    }

    println!("--- Status Summary ---");
    println!("On Time: {}", on_time);
    println!("Delayed: {}", delayed);
    println!("Cancelled: {}", cancelled);
    println!("Arrived: {}", arrived);
    println!("Departed: {}", departed);
}

// TODO 8: Implement `print_delayed_trains`
fn print_delayed_trains(trains: &[Train], min_delay: u32) {
    println!("--- Trains Delayed Over {} Minutes ---", min_delay);
    for train in trains {
        if train.is_delayed_over(min_delay) {
            train.display();
        }
    }
}

// ============================================================================
// main() — identical to the exercise file
// ============================================================================

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
