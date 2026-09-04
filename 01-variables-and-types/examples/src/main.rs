fn main() {
    // =============================================
    // LESSON 01: Variables & Types — Live Examples
    // =============================================
    // Run this with: cargo run
    // Read along with lesson.md as you go!

    // --- 1. Immutable vs Mutable ---
    let station = "Chennai Central";
    println!("Station: {station}");

    let mut platform = 5;
    println!("Originally assigned to platform {platform}");
    platform = 3;
    println!("Reassigned to platform {platform}");

    // --- 2. Integer types ---
    let passengers: u32 = 487;        // unsigned — can't be negative
    let temperature: i32 = -2;        // signed — can be negative
    let coach_count: u8 = 24;         // small unsigned — max 255
    println!("Passengers: {passengers}, Temp: {temperature}°C, Coaches: {coach_count}");

    // --- 3. Floats ---
    let speed_kmh: f64 = 130.5;       // 64-bit float (default)
    let delay_hours: f32 = 0.25;      // 32-bit float
    println!("Speed: {speed_kmh} km/h, Delay: {delay_hours} hours");

    // --- 4. Booleans ---
    let is_express = true;
    let is_cancelled: bool = false;
    println!("Express? {is_express}, Cancelled? {is_cancelled}");

    // --- 5. Characters (Unicode!) ---
    let train_emoji: char = '🚂';
    let check_mark: char = '✓';
    println!("{train_emoji} All aboard! {check_mark}");

    // --- 6. The two kinds of strings ---
    let code: &str = "MAS";                            // string slice — fixed
    let mut train_name = String::from("Shatabdi");     // owned String — growable
    println!("Code: {code}, Train: {train_name}");

    train_name.push_str(" Express");                   // modify the owned String
    println!("Full name: {train_name}");

    // --- 7. Constants ---
    const MAX_SPEED_KMH: u32 = 200;
    const STATION_CODE: &str = "MAS";
    println!("Max speed: {MAX_SPEED_KMH} km/h at {STATION_CODE}");

    // --- 8. Shadowing ---
    let ticket = "SL-42";             // ticket is a &str
    println!("Ticket code: {ticket}");

    let ticket = 42;                  // shadowed — now ticket is an i32!
    println!("Ticket number: {ticket}");

    let ticket = ticket * 2;          // shadowed again — transformed
    println!("Doubled: {ticket}");

    // --- 9. Type inference in action ---
    let a = 10;          // i32
    let b = 3.14;        // f64
    let c = true;        // bool
    let d = 'Z';         // char
    println!("Inferred types → a: {a}, b: {b}, c: {c}, d: {d}");

    // --- 10. Formatted printing ---
    let train = "Rajdhani Express";
    let departs = "06:00";
    let plat = 1;
    println!();
    println!("╔══════════════════════════════════╗");
    println!("║   🚂 DEPARTURE BOARD             ║");
    println!("╠══════════════════════════════════╣");
    println!("║  Train:    {train:<20}  ║");
    println!("║  Departs:  {departs:<20}  ║");
    println!("║  Platform: {plat:<20}  ║");
    println!("╚══════════════════════════════════╝");
}
