// =============================================
// EXERCISE 08: Platform Scheduler
// =============================================
//
// Build a complete platform scheduler from scratch.
// There is NO code provided — you write everything, including main().
//
// This exercises: tuples, destructuring, ranges, range match patterns, slicing.
//
// Run with: cargo run
//
// Expected output:
//
//   === PLATFORM SCHEDULER ===
//
//   --- Full Schedule (6 stops) ---
//     Rajdhani Express (platform 1) — limit 130 km/h [fast]
//     Shatabdi Express (platform 2) — limit 150 km/h [fast]
//     Duronto Express (platform 3) — limit 120 km/h [moderate]
//     Vande Bharat Express (platform 4) — limit 180 km/h [fast]
//     Kovai Express (platform 5) — limit 110 km/h [moderate]
//     Island Express (platform 6) — limit 100 km/h [moderate]
//
//   --- Fastest Stop ---
//     Fastest: Vande Bharat Express (platform 4) — limit 180 km/h
//
//   --- Stops in Speed Range (100-130 km/h) ---
//     Rajdhani Express
//     Duronto Express
//     Kovai Express
//     Island Express
//
//   --- Platforms Window (index 1..4) ---
//     Shatabdi Express (platform 2) — limit 150 km/h [fast]
//     Duronto Express (platform 3) — limit 120 km/h [moderate]
//     Vande Bharat Express (platform 4) — limit 180 km/h [fast]

// =============================================
// STEP 1: Implement `describe_stop`
// =============================================
// fn describe_stop((name, platform, speed): (&str, u8, u32)) -> String
//
// Destructure the tuple parameter directly in the signature.
// Returns: "{name} (platform {platform}) — limit {speed} km/h"

// =============================================
// STEP 2: Implement `build_schedule() -> Vec<(&'static str, u8, u32)>`
// =============================================
// Return a vec! of (name, platform, speed_kmh) tuples:
//   ("Rajdhani Express",      1, 130)
//   ("Shatabdi Express",      2, 150)
//   ("Duronto Express",       3, 120)
//   ("Vande Bharat Express",  4, 180)
//   ("Kovai Express",         5, 110)
//   ("Island Express",       6, 100)

// =============================================
// STEP 3: Implement `fastest_stop`
// =============================================
// fn fastest_stop<'a>(schedule: &[(&'a str, u8, u32)]) -> (&'a str, u8, u32)
//
// Walk the slice and return the tuple with the highest speed (field .2).
// Start with schedule[0] as the running fastest.
// Note the lifetime: the returned tuple borrows &str data from the input slice.

// =============================================
// STEP 4: Implement `stops_in_speed_range`
// =============================================
// fn stops_in_speed_range<'a>(
//     schedule: &[(&'a str, u8, u32)],
//     range: std::ops::RangeInclusive<u32>,
// ) -> Vec<&'a str>
//
// Return the names of every stop whose speed falls inside `range`.
// Hint: range.contains(&speed)

// =============================================
// STEP 5: Implement `classify_speed`
// =============================================
// fn classify_speed(speed_kmh: u32) -> &'static str
//
// Use a match with inclusive range patterns:
//   0..=59    => "slow"
//   60..=120  => "moderate"
//   121..=200 => "fast"
//   _         => "very fast"

// =============================================
// STEP 6: Implement `platforms_window`
// =============================================
// fn platforms_window<'a>(
//     schedule: &'a [(&'a str, u8, u32)],
//     range: std::ops::Range<usize>,
// ) -> &'a [(&'a str, u8, u32)]
//
// Return the sub-slice at the given index range: &schedule[range]

// =============================================
// STEP 7: Write `fn main()`
// =============================================
// Build the schedule, then:
//
// 1. Print "=== PLATFORM SCHEDULER ==="
//
// 2. Print "--- Full Schedule ({count} stops) ---"
//    Loop through the schedule, print each using describe_stop(),
//    with its classify_speed() result in brackets: "  {describe} [{class}]"
//
// 3. Print "--- Fastest Stop ---"
//    Call fastest_stop(), print "  Fastest: {describe_stop(...)}"
//
// 4. Print "--- Stops in Speed Range (100-130 km/h) ---"
//    Call stops_in_speed_range(&schedule, 100..=130), print each name
//
// 5. Print "--- Platforms Window (index 1..4) ---"
//    Call platforms_window(&schedule, 1..4), print each stop with its
//    classification, same format as step 2

fn main() {
    // Your code here
}
