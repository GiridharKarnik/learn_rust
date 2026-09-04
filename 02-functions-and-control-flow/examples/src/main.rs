fn main() {
    // =============================================
    // LESSON 02: Functions & Control Flow — Examples
    // =============================================

    // --- 1. Simple function calls ---
    announce_departure();
    announce_train("Shatabdi Express", 3);

    // --- 2. Functions that return values ---
    let price = ticket_price(350.0);
    println!("Ticket price for 350 km: ₹{price:.2}");

    let free = ticket_price(-10.0);
    println!("Invalid distance price: ₹{free:.2}");

    // --- 3. if/else ---
    let delay = 15;
    check_delay(delay);
    check_delay(0);
    check_delay(5);

    // --- 4. if as an expression ---
    let is_express = true;
    let coach_type = if is_express { "AC Chair" } else { "General" };
    println!("Coach type: {coach_type}");

    // --- 5. loop with break returning a value ---
    let mut signal_checks = 0;
    let signal_status = loop {
        signal_checks += 1;
        println!("  Signal check #{signal_checks}...");
        if signal_checks == 3 {
            break "All clear!";
        }
    };
    println!("Signal: {signal_status}");

    // --- 6. while loop ---
    println!();
    accelerate();

    // --- 7. for loops ---
    println!();
    println!("--- Platforms ---");
    for p in 1..=5 {
        println!("Platform {p}: ready");
    }

    println!();
    println!("--- Route ---");
    let route = ["Chennai", "Katpadi", "Jolarpettai", "Bangalore"];
    for (i, station) in route.iter().enumerate() {
        if i == 0 {
            println!("  🟢 START: {station}");
        } else if i == route.len() - 1 {
            println!("  🔴 END:   {station}");
        } else {
            println!("  ⚪ STOP:  {station}");
        }
    }

    // --- 8. match ---
    println!();
    for platform in 1..=6 {
        let train = match platform {
            1 => "Rajdhani Express",
            2 => "Shatabdi Express",
            3 | 4 => "Duronto Express",
            5 => "Garib Rath",
            _ => "Local service",
        };
        println!("Platform {platform}: {train}");
    }

    // --- 9. match with ranges ---
    println!();
    classify_delay(0);
    classify_delay(7);
    classify_delay(25);
    classify_delay(90);

    // --- 10. break and continue ---
    println!();
    println!("--- Dispatching Trains ---");
    for id in 1..=10 {
        if id == 4 || id == 7 {
            println!("  Train {id}: CANCELLED — skipping");
            continue;
        }
        if id > 8 {
            println!("  Depot closed. Stopping dispatch.");
            break;
        }
        println!("  Train {id}: dispatched ✓");
    }
}

// --- Function definitions ---

fn announce_departure() {
    println!("📢 The train is now departing!");
    println!();
}

fn announce_train(name: &str, platform: u32) {
    println!("📢 {name} is arriving at platform {platform}");
    println!();
}

fn ticket_price(distance_km: f64) -> f64 {
    if distance_km <= 0.0 {
        return 0.0; // early return for invalid input
    }
    distance_km * 0.75 // implicit return — no semicolon!
}

fn check_delay(minutes: u32) {
    if minutes == 0 {
        println!("🟢 On time!");
    } else if minutes < 10 {
        println!("🟡 {minutes} min delay — slight");
    } else {
        println!("🔴 {minutes} min delay — significant");
    }
}

fn accelerate() {
    println!("--- Acceleration ---");
    let mut speed: u32 = 0;
    while speed < 120 {
        speed += 30;
        println!("  🚂 {speed} km/h");
    }
    println!("  Cruising speed reached!");
}

fn classify_delay(minutes: u32) {
    let status = match minutes {
        0 => "✅ On time",
        1..=10 => "🟡 Slight delay",
        11..=30 => "🟠 Moderate delay",
        31..=60 => "🔴 Major delay",
        _ => "🚨 Severe delay",
    };
    println!("Delay {minutes} min → {status}");
}
