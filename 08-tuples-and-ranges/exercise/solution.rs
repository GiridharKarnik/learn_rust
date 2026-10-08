// =============================================
// SOLUTION — Don't peek until you've tried!
// =============================================

use std::ops::{Range, RangeInclusive};

// TODO 1
fn describe_stop((name, platform, speed): (&str, u8, u32)) -> String {
    format!("{name} (platform {platform}) \u{2014} limit {speed} km/h")
}

// TODO 2
fn build_schedule() -> Vec<(&'static str, u8, u32)> {
    vec![
        ("Rajdhani Express", 1, 130),
        ("Shatabdi Express", 2, 150),
        ("Duronto Express", 3, 120),
        ("Vande Bharat Express", 4, 180),
        ("Kovai Express", 5, 110),
        ("Island Express", 6, 100),
    ]
}

// TODO 3
fn fastest_stop<'a>(schedule: &[(&'a str, u8, u32)]) -> (&'a str, u8, u32) {
    let mut fastest = schedule[0];
    for &stop in &schedule[1..] {
        if stop.2 > fastest.2 {
            fastest = stop;
        }
    }
    fastest
}

// TODO 4
fn stops_in_speed_range<'a>(
    schedule: &[(&'a str, u8, u32)],
    range: RangeInclusive<u32>,
) -> Vec<&'a str> {
    let mut result = Vec::new();
    for &(name, _platform, speed) in schedule {
        if range.contains(&speed) {
            result.push(name);
        }
    }
    result
}

// TODO 5
fn classify_speed(speed_kmh: u32) -> &'static str {
    match speed_kmh {
        0..=59 => "slow",
        60..=120 => "moderate",
        121..=200 => "fast",
        _ => "very fast",
    }
}

// TODO 6
fn platforms_window<'a>(
    schedule: &'a [(&'a str, u8, u32)],
    range: Range<usize>,
) -> &'a [(&'a str, u8, u32)] {
    &schedule[range]
}

fn main() {
    println!("=== PLATFORM SCHEDULER ===");

    let schedule = build_schedule();

    println!("\n--- Full Schedule ({} stops) ---", schedule.len());
    for &stop in &schedule {
        let classification = classify_speed(stop.2);
        println!("  {} [{}]", describe_stop(stop), classification);
    }

    println!("\n--- Fastest Stop ---");
    let fastest = fastest_stop(&schedule);
    println!("  Fastest: {}", describe_stop(fastest));

    println!("\n--- Stops in Speed Range (100-130 km/h) ---");
    for name in stops_in_speed_range(&schedule, 100..=130) {
        println!("  {name}");
    }

    println!("\n--- Platforms Window (index 1..4) ---");
    for &stop in platforms_window(&schedule, 1..4) {
        let classification = classify_speed(stop.2);
        println!("  {} [{}]", describe_stop(stop), classification);
    }
}
