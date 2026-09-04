// =============================================
// SOLUTION — Don't peek until you've tried!
// =============================================

fn main() {
    println!("=== LOOPS PRACTICE ===\n");

    println!("--- Challenge 1: Platform Announcements ---");
    platform_announcements();

    println!("\n--- Challenge 2: Speed Control ---");
    speed_control();

    println!("\n--- Challenge 3: Passenger Boarding ---");
    passenger_boarding();

    println!("\n--- Challenge 4: Station Finder ---");
    let found = station_finder();
    println!("Result: {found}");

    println!("\n--- Challenge 5: Ticket Queue ---");
    ticket_queue();

    println!("\n--- Challenge 6: Train Timetable ---");
    timetable();
}

fn platform_announcements() {
    for platform in 1..=8 {
        if platform == 4 || platform == 7 {
            continue;
        }
        println!("Platform {platform}: Now boarding");
    }
}

fn speed_control() {
    let mut speed: u32 = 0;
    while speed < 120 {
        speed += 15;
        if speed == 60 {
            println!("Speed: {speed} km/h \u{2190} speed check passed");
        } else if speed >= 120 {
            println!("Speed: {speed} km/h \u{2190} cruising speed reached!");
        } else {
            println!("Speed: {speed} km/h");
        }
    }
}

fn passenger_boarding() {
    let total: u32 = 50;
    let mut remaining = total;
    loop {
        if remaining >= 8 {
            remaining -= 8;
            println!("Boarded 8 passengers, {remaining} remaining");
        } else {
            println!("Final group: boarded last {remaining} passengers");
            remaining = 0;
        }
        if remaining == 0 {
            break;
        }
    }
    println!("All {total} passengers aboard!");
}

fn station_finder() -> String {
    let stations = ["Chennai", "Perambur", "Katpadi", "Jolarpettai", "Bangalore"];

    for (index, station) in stations.iter().enumerate() {
        if *station == "Katpadi" {
            return format!("{station} found at stop {}", index + 1);
        }
    }

    String::from("Station not found")
}

fn ticket_queue() {
    for counter in 1..=3 {
        for passenger in 1..=4 {
            if counter == 2 && passenger == 2 {
                println!("Counter {counter}: \u{26a0} passenger {passenger} invalid ticket \u{2014} skipped");
                continue;
            }
            println!("Counter {counter}: served passenger {passenger}");
        }
    }
}

fn timetable() {
    let names = ["Rajdhani", "Shatabdi", "Kovai Exp", "Duronto", "Garib Rath"];
    let departures = ["06:00", "07:30", "CANCELLED", "10:15", "22:00"];
    let platforms = [1, 2, 4, 3, 5];

    println!("{:<14} | {:<7} | {}", "Train", "Departs", "Platform");
    println!("---------------+---------+---------");

    let mut count = 0;
    for i in 0..names.len() {
        if departures[i] == "CANCELLED" {
            continue;
        }
        println!("{:<14} | {:<7} | {}", names[i], departures[i], platforms[i]);
        count += 1;
    }
    println!();
    println!("{count} trains scheduled");
}
