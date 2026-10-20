// =============================================
// SOLUTION — Don't peek until you've tried!
// =============================================

use std::ops::{Bound, RangeBounds, RangeInclusive};

// STEP 1
fn classify_delay(minutes: i32) -> &'static str {
    match minutes {
        i32::MIN..=-1 => "early",
        0 => "on time",
        1..=5 => "minor",
        6..=30 => "late",
        31.. => "severe",
    }
}

// STEP 2
fn fare_for_km(km: u32) -> u32 {
    match km {
        0 => 0,
        1..=10 => 5,
        11..=25 => 12,
        26..=60 => 25,
        _ => 40,
    }
}

// STEP 3
fn char_kind(c: char) -> &'static str {
    match c {
        'a'..='z' => "lower",
        'A'..='Z' => "upper",
        '0'..='9' => "digit",
        ' ' | '\t' => "space",
        _ => "other",
    }
}

// STEP 4
fn coach_label(code: u8) -> String {
    match code {
        n @ 1..=3 => format!("First-{n}"),
        n @ 4..=9 => format!("Standard-{n}"),
        n @ (10 | 20 | 30) => format!("Special-{n}"),
        n @ 10..=40 => format!("Sleeper-{n}"),
        0 => "Engine".to_string(),
        n => format!("Unknown-{n}"),
    }
}

// STEP 5
const EXPRESS_LOW: u32 = 12000;
const EXPRESS_HIGH: u32 = 12999;
const PASSENGER_LOW: u32 = 50000;
const PASSENGER_HIGH: u32 = 59999;

fn train_category(no: u32) -> Option<&'static str> {
    match no {
        EXPRESS_LOW..=EXPRESS_HIGH => Some("Express"),
        PASSENGER_LOW..=PASSENGER_HIGH => Some("Passenger"),
        10000..=19999 => Some("Other mail"),
        _ => None,
    }
}

// STEP 6
fn describe_bounds<R: RangeBounds<i32>>(r: &R) -> String {
    let left = match r.start_bound() {
        Bound::Included(a) => format!("[{a}"),
        Bound::Excluded(a) => format!("({a}"),
        Bound::Unbounded => "(-inf".to_string(),
    };
    let right = match r.end_bound() {
        Bound::Included(b) => format!("{b}]"),
        Bound::Excluded(b) => format!("{b})"),
        Bound::Unbounded => "+inf)".to_string(),
    };
    format!("{left}, {right}")
}

// STEP 7
fn my_contains<R: RangeBounds<u32>>(r: &R, x: u32) -> bool {
    let after_start = match r.start_bound() {
        Bound::Included(a) => x >= *a,
        Bound::Excluded(a) => x > *a,
        Bound::Unbounded => true,
    };
    let before_end = match r.end_bound() {
        Bound::Included(b) => x <= *b,
        Bound::Excluded(b) => x < *b,
        Bound::Unbounded => true,
    };
    after_start && before_end
}

// STEP 8
fn resolve<R: RangeBounds<usize>>(r: &R, len: usize) -> std::ops::Range<usize> {
    let start = match r.start_bound() {
        Bound::Included(s) => *s,
        Bound::Excluded(s) => s.saturating_add(1),
        Bound::Unbounded => 0,
    };
    let end = match r.end_bound() {
        Bound::Included(e) => e.saturating_add(1),
        Bound::Excluded(e) => *e,
        Bound::Unbounded => len,
    };
    let end = end.min(len);
    let start = start.min(end);
    start..end
}

fn take_window<T, R: RangeBounds<usize>>(items: &[T], r: R) -> &[T] {
    &items[resolve(&r, items.len())]
}

// STEP 9
#[derive(Debug)]
struct Limit {
    name: &'static str,
    kmh: RangeInclusive<u32>,
}

fn find_limit(limits: &[Limit], speed: u32) -> Option<&Limit> {
    limits.iter().find(|l| l.kmh.contains(&speed))
}

// STEP 10
fn signal_action(speed: u32, dist_m: u32) -> &'static str {
    match (speed, dist_m) {
        (0, _) => "hold",
        (1..=30, 0..=100) => "proceed cautiously",
        (1..=30, _) => "proceed",
        (31..=80, 0..=200) => "brake",
        (31..=80, _) => "coast",
        (81.., 0..=500) => "emergency",
        (81.., _) => "slow",
    }
}

// STEP 11
fn u8_band(n: u8) -> &'static str {
    match n {
        0..=99 => "low",
        100..=199 => "mid",
        200..=255 => "high",
    }
}

fn hour_period(h: u8) -> Result<&'static str, String> {
    match h {
        0..=5 => Ok("night"),
        6..=11 => Ok("morning"),
        12..=17 => Ok("afternoon"),
        18..=23 => Ok("evening"),
        other => Err(format!("hour {other} out of range")),
    }
}

fn is_peak(hour: u32) -> bool {
    matches!(hour, 7..=10 | 17..=20)
}

// STEP 12
fn rotate_letter(c: char, by: u8) -> char {
    match c {
        'A'..='Z' => (((c as u8 - b'A' + by) % 26) + b'A') as char,
        'a'..='z' => (((c as u8 - b'a' + by) % 26) + b'a') as char,
        _ => c,
    }
}

// STEP 13
fn main() {
    println!("=== RANGES IN PATTERNS AND BOUNDS ===");

    println!("\n--- 1. Delay classes ---");
    for m in [-4, 0, 3, 6, 30, 31, 500] {
        println!("  {:>4} min -> {}", m, classify_delay(m));
    }

    println!("\n--- 2. Fares ---");
    for km in [0, 1, 10, 11, 25, 26, 60, 61] {
        println!("  {:>2} km -> Rs {}", km, fare_for_km(km));
    }

    println!("\n--- 3. Character kinds ---");
    for c in ['q', 'Q', '7', ' ', '#', 'z', '{'] {
        println!("  {:?} -> {}", c, char_kind(c));
    }

    println!("\n--- 4. Coach labels ---");
    let labels: Vec<String> = [0u8, 2, 7, 10, 15, 30, 41].iter().map(|&c| coach_label(c)).collect();
    println!("  {}", labels.join(", "));

    println!("\n--- 5. Train categories ---");
    for no in [12627, 12000, 50123, 11999, 20000, 7] {
        println!("  {} -> {:?}", no, train_category(no));
    }

    println!("\n--- 6. Bounds as intervals ---");
    println!("  {}", describe_bounds(&(1..5)));
    println!("  {}", describe_bounds(&(1..=5)));
    println!("  {}", describe_bounds(&(3..)));
    println!("  {}", describe_bounds(&(..7)));
    println!("  {}", describe_bounds(&(..=7)));
    println!("  {}", describe_bounds(&(..)));
    println!("  {}", describe_bounds(&(Bound::Excluded(0), Bound::Included(9))));

    println!("\n--- 7. Hand-made contains ---");
    for x in [4, 5, 9, 10] {
        let a = my_contains(&(5..10), x);
        let b = my_contains(&(5..=10), x);
        let c = my_contains(&(5..), x);
        println!("  {x}: 5..10={a} 5..=10={b} 5..={c}");
    }
    println!("  agrees with std? {}", (0..15).all(|x| my_contains(&(5..10), x) == (5..10).contains(&x)));

    println!("\n--- 8. Generic windows ---");
    let stops = ["MAS", "AJJ", "KPD", "JTJ", "SA"];
    println!("  1..3   -> {:?}", take_window(&stops, 1..3));
    println!("  ..2    -> {:?}", take_window(&stops, ..2));
    println!("  3..    -> {:?}", take_window(&stops, 3..));
    println!("  ..     -> {:?}", take_window(&stops, ..).len());
    println!("  2..=3  -> {:?}", take_window(&stops, 2..=3));
    println!("  3..99  -> {:?}", take_window(&stops, 3..99));
    println!("  7..    -> {:?}", take_window(&stops, 7..));
    println!("  4..1   -> {:?}", take_window(&stops, 4..1));

    println!("\n--- 9. Ranges in a table ---");
    let limits = [
        Limit { name: "yard", kmh: 0..=15 },
        Limit { name: "branch", kmh: 16..=80 },
        Limit { name: "main", kmh: 81..=130 },
    ];
    for s in [0, 15, 16, 100, 130, 131] {
        match find_limit(&limits, s) {
            Some(l) => println!("  {:>3} km/h -> {} {:?}", s, l.name, l.kmh),
            None => println!("  {:>3} km/h -> no limit class", s),
        }
    }

    println!("\n--- 10. Tuple patterns ---");
    for (s, d) in [(0, 50), (20, 80), (20, 400), (60, 150), (60, 900), (100, 450), (100, 2000)] {
        println!("  ({:>3}, {:>4}) -> {}", s, d, signal_action(s, d));
    }

    println!("\n--- 11. Exhaustive and matches! ---");
    println!("  u8 bands: {} {} {} {}", u8_band(0), u8_band(99), u8_band(100), u8_band(255));
    println!("  hour_period(5) = {:?}", hour_period(5));
    println!("  hour_period(18) = {:?}", hour_period(18));
    println!("  hour_period(24) = {:?}", hour_period(24));
    let peaks: Vec<u32> = (0..24).filter(|&h| is_peak(h)).collect();
    println!("  peak hours: {:?}", peaks);

    println!("\n--- 12. Rotating letters ---");
    let msg = "Hello, Zebra-9!";
    let out: String = msg.chars().map(|c| rotate_letter(c, 3)).collect();
    println!("  {} -> {}", msg, out);
    let back: String = out.chars().map(|c| rotate_letter(c, 23)).collect();
    println!("  and back: {}", back);
}
