// =============================================
// SOLUTION — Don't peek until you've tried!
// =============================================

fn main() {
    println!("=== TRAIN DISPATCH SYSTEM ===");

    // --- Part 1: Fare Calculator ---
    println!();
    println!("--- Fare Calculator ---");
    let distances = [100, 500, 1200, 0];
    for dist in distances {
        let fare = calculate_fare(dist as f64);
        println!("{dist} km: ₹{fare:.2}");
    }

    // --- Part 2: Delay Announcements ---
    println!();
    println!("--- Delay Announcements ---");
    print_delay_status(12001, 0);
    print_delay_status(12002, 5);
    print_delay_status(12003, 45);
    print_delay_status(12004, 999);

    // --- Part 3: Platform Assignment ---
    println!();
    println!("--- Platform Assignment ---");
    let trains = [
        "Rajdhani Express",
        "Duronto Express",
        "Local Passenger",
        "Garib Rath",
        "Shatabdi Express",
        "Metro Express",
    ];
    for train in trains {
        let (platform, category) = assign_platform(train);
        println!("{train} → Platform {platform} ({category})");
    }

    // --- Part 4: Departure Countdown ---
    println!();
    println!("--- Countdown ---");
    departure_countdown(12001, 5);

    // --- Part 5: Dispatch with skip & stop ---
    println!();
    println!("--- Active Trains ---");
    dispatch_trains();
}

fn calculate_fare(distance_km: f64) -> f64 {
    if distance_km <= 0.0 {
        return 0.0;
    }
    if distance_km <= 500.0 {
        distance_km * 0.75
    } else {
        500.0 * 0.75 + (distance_km - 500.0) * 0.50
    }
}

fn print_delay_status(train_number: u32, delay_minutes: u32) {
    match delay_minutes {
        0 => println!("Train {train_number}: On Time ✅"),
        1..=15 => println!("Train {train_number}: Running {delay_minutes} min late ⚠️"),
        16..=120 => println!("Train {train_number}: Delayed by {delay_minutes} min 🔴"),
        _ => println!("Train {train_number}: Cancelled ❌"),
    }
}

fn assign_platform(train_name: &str) -> (u32, &str) {
    if train_name.contains("Rajdhani") || train_name.contains("Shatabdi") {
        (1, "Rajdhani/Shatabdi")
    } else if train_name.contains("Duronto") || train_name.contains("Garib Rath") {
        (3, "Long distance")
    } else {
        (5, "Local/Passenger")
    }
}

fn departure_countdown(train_number: u32, seconds: u32) {
    let mut remaining = seconds;
    while remaining > 0 {
        println!("Train {train_number} departing in {remaining}...");
        remaining -= 1;
    }
    println!("🚂 Train {train_number} has departed!");
}

fn dispatch_trains() {
    let mut dispatched = 0;
    for id in 1..=10 {
        if id == 3 || id == 6 {
            println!("Train {id} is under maintenance — skipping");
            continue;
        }
        if dispatched == 5 {
            println!("Max dispatches reached. Stopping.");
            break;
        }
        println!("Dispatching train {id} ✓");
        dispatched += 1;
    }
}
