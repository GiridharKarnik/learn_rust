// =============================================
// SOLUTION — Don't peek until you've tried!
// =============================================

use std::ops::{Range, RangeInclusive};

// STEP 1
fn range_info(r: Range<i32>) -> String {
    format!(
        "{:?} -> start={} end={} len={} empty={}",
        r,
        r.start,
        r.end,
        r.len(),
        r.is_empty()
    )
}

// STEP 2
fn inclusive_info(r: RangeInclusive<u32>) -> String {
    format!(
        "{:?} -> start={} end={} count={} empty={}",
        r,
        r.start(),
        r.end(),
        r.clone().count(),
        r.is_empty()
    )
}

// STEP 3
fn range_family() -> Vec<String> {
    vec![
        format!("{:?}", 2..6),
        format!("{:?}", 2..=6),
        format!("{:?}", 2..),
        format!("{:?}", ..6),
        format!("{:?}", ..=6),
        format!("{:?}", ..),
    ]
}

// STEP 4
fn countdown(from: u32) -> Vec<u32> {
    (1..=from).rev().collect()
}

fn platform_numbers(first: u8, last: u8, step: usize) -> Vec<u8> {
    (first..=last).step_by(step).collect()
}

// STEP 5
fn step_then_rev(n: u32, k: usize) -> (Vec<u32>, Vec<u32>) {
    let step_first: Vec<u32> = (0..n).step_by(k).rev().collect();
    let rev_first: Vec<u32> = (0..n).rev().step_by(k).collect();
    (step_first, rev_first)
}

// STEP 6
fn triangular(n: u32) -> u32 {
    (1..=n).sum()
}

fn checked_total(mut r: RangeInclusive<u8>) -> Option<u8> {
    r.try_fold(0u8, |acc, x| acc.checked_add(x))
}

// STEP 7
fn window<T>(items: &[T], r: Range<usize>) -> Option<&[T]> {
    items.get(r)
}

fn last_n<T>(items: &[T], n: usize) -> &[T] {
    &items[items.len().saturating_sub(n)..]
}

// STEP 8
fn letters(from: char, to: char) -> String {
    (from..=to).collect()
}

fn platform_labels(last: char) -> Vec<String> {
    ('A'..=last).map(|c| format!("Platform {c}")).collect()
}

// STEP 9
fn gauge_status(pressure: f64) -> &'static str {
    if !(0.0..).contains(&pressure) {
        "invalid"
    } else if (0.0..5.0).contains(&pressure) {
        "low"
    } else if (5.0..=8.0).contains(&pressure) {
        "normal"
    } else {
        "high"
    }
}

// STEP 10
#[derive(Debug, Clone, PartialEq)]
struct Shift {
    label: &'static str,
    hours: Range<u32>,
}

impl Shift {
    fn covers(&self, hour: u32) -> bool {
        self.hours.contains(&hour)
    }

    fn duration(&self) -> u32 {
        self.hours.end.saturating_sub(self.hours.start)
    }

    fn describe(&self) -> String {
        if self.hours.is_empty() {
            format!("{}: empty shift", self.label)
        } else {
            format!(
                "{}: {:02}:00-{:02}:00 ({}h)",
                self.label,
                self.hours.start,
                self.hours.end,
                self.duration()
            )
        }
    }
}

// STEP 11
fn chunk_ranges(total: usize, size: usize) -> Vec<Range<usize>> {
    (0..total)
        .step_by(size)
        .map(|start| start..(start + size).min(total))
        .collect()
}

fn span_of(values: &[i32]) -> Option<RangeInclusive<i32>> {
    let min = *values.iter().min()?;
    let max = *values.iter().max()?;
    Some(min..=max)
}

// STEP 12
fn bump_platform(current: u8, by: u8, last: u8) -> (u8, bool) {
    match current.checked_add(by) {
        Some(n) if n <= last => (n, false),
        _ => (last, true),
    }
}

// STEP 13
fn main() {
    println!("=== RANGE TYPE DRILLS ===");

    println!("\n--- 1. Range<i32> anatomy ---");
    for r in [1..5, 3..3, 5..1, -2..2] {
        println!("  {}", range_info(r));
    }

    println!("\n--- 2. RangeInclusive<u32> anatomy ---");
    for r in [1..=5, 3..=3, 5..=1] {
        println!("  {}", inclusive_info(r));
    }

    println!("\n--- 3. The range family ---");
    for (i, text) in range_family().iter().enumerate() {
        println!("  {}. {}", i + 1, text);
    }

    println!("\n--- 4. Countdown and platform numbers ---");
    println!("  countdown(5) = {:?}", countdown(5));
    println!("  countdown(0) = {:?}", countdown(0));
    println!("  platforms(1, 12, 3) = {:?}", platform_numbers(1, 12, 3));
    println!("  platforms(250, 255, 2) = {:?}", platform_numbers(250, 255, 2));

    println!("\n--- 5. step_by vs rev order ---");
    let (a, b) = step_then_rev(11, 3);
    println!("  step_by(3).rev(): {:?}", a);
    println!("  rev().step_by(3): {:?}", b);

    println!("\n--- 6. Sums and overflow ---");
    println!("  triangular(10) = {}", triangular(10));
    println!("  triangular(0) = {}", triangular(0));
    println!("  checked_total(1..=22) = {:?}", checked_total(1..=22));
    println!("  checked_total(1..=23) = {:?}", checked_total(1..=23));

    println!("\n--- 7. Slicing with ranges ---");
    let stops = ["Chennai", "Arakkonam", "Katpadi", "Jolarpettai", "Salem"];
    println!("  window(1..3) = {:?}", window(&stops, 1..3));
    println!("  window(0..0) = {:?}", window(&stops, 0..0));
    println!("  window(3..9) = {:?}", window(&stops, 3..9));
    println!("  window(4..2) = {:?}", window(&stops, 4..2));
    println!("  last_n(2) = {:?}", last_n(&stops, 2));
    println!("  last_n(99) = {:?}", last_n(&stops, 99).len());

    println!("\n--- 8. Char ranges ---");
    println!("  letters(a, f) = {}", letters('a', 'f'));
    println!("  letters(x, c) = {:?}", letters('x', 'c'));
    for label in platform_labels('C') {
        println!("  {label}");
    }

    println!("\n--- 9. Float ranges ---");
    for p in [0.0, 4.99, 5.0, 8.0, 8.01, -1.0, f64::NAN] {
        println!("  {:>5} -> {}", p, gauge_status(p));
    }

    println!("\n--- 10. Ranges in structs ---");
    let shifts = [
        Shift { label: "Morning", hours: 6..14 },
        Shift { label: "Evening", hours: 14..22 },
        Shift { label: "Night", hours: 22..6 },
    ];
    for s in &shifts {
        println!("  {} | covers 14h? {}", s.describe(), s.covers(14));
    }
    println!("  {:?}", shifts[0]);
    println!("  morning == clone? {}", shifts[0] == shifts[0].clone());

    println!("\n--- 11. Chunking and spans ---");
    println!("  chunk_ranges(10, 4) = {:?}", chunk_ranges(10, 4));
    println!("  chunk_ranges(8, 4) = {:?}", chunk_ranges(8, 4));
    println!("  chunk_ranges(0, 4) = {:?}", chunk_ranges(0, 4));
    println!("  span_of([40, 15, 90, 62]) = {:?}", span_of(&[40, 15, 90, 62]));
    println!("  span_of([]) = {:?}", span_of(&[]));

    println!("\n--- 12. Clamped platform bumps ---");
    for (cur, by) in [(3u8, 2u8), (9, 5), (250, 10)] {
        let (p, clamped) = bump_platform(cur, by, 12);
        println!("  bump({cur}, {by}) -> platform {p}, clamped={clamped}");
    }
}
