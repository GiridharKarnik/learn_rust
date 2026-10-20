// =============================================
// SOLUTION 07b - Vec Mutation and Reordering
// =============================================

// STEP 1
fn remove_ordered(v: &mut Vec<String>, index: usize) -> Option<String> {
    if index < v.len() {
        Some(v.remove(index))
    } else {
        None
    }
}

fn remove_fast(v: &mut Vec<String>, index: usize) -> Option<String> {
    if index < v.len() {
        Some(v.swap_remove(index))
    } else {
        None
    }
}

// STEP 2
fn keep_first(v: &mut Vec<u32>, n: usize) {
    v.truncate(n);
}

fn split_route(v: &mut Vec<String>, at: usize) -> Vec<String> {
    v.split_off(at.min(v.len()))
}

// STEP 3
fn merge(a: &mut Vec<String>, b: &mut Vec<String>) {
    a.append(b);
}

fn extend_from(a: &mut Vec<String>, extra: &[&str]) {
    a.extend(extra.iter().map(|s| s.to_string()));
}

// STEP 4
fn drop_slow(speeds: &mut Vec<u32>, min: u32) -> usize {
    let before = speeds.len();
    speeds.retain(|&s| s >= min);
    before - speeds.len()
}

// STEP 5
fn take_range(v: &mut Vec<String>, start: usize, end: usize) -> Vec<String> {
    v.drain(start..end).collect()
}

fn take_all(v: &mut Vec<String>) -> Vec<String> {
    v.drain(..).collect()
}

// STEP 6
fn dedup_consecutive(v: &mut Vec<String>) {
    v.dedup();
}

fn unique_sorted(v: &mut Vec<u32>) {
    v.sort();
    v.dedup();
}

// STEP 7
fn sort_desc(v: &mut [u32]) {
    v.sort_by(|a, b| b.cmp(a));
}

fn sort_by_len_then_alpha(v: &mut [String]) {
    v.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
}

fn sort_by_last_char(v: &mut [String]) {
    v.sort_by_key(|s| s.chars().last());
}

// STEP 8
fn sort_floats_desc(v: &mut [f64]) {
    v.sort_by(|a, b| b.partial_cmp(a).unwrap());
}

// STEP 9
fn locate(sorted: &[u32], target: u32) -> String {
    match sorted.binary_search(&target) {
        Ok(i) => format!("{target} found at {i}"),
        Err(i) => format!("{target} missing, would insert at {i}"),
    }
}

fn insert_sorted(sorted: &mut Vec<u32>, value: u32) {
    let pos = sorted.binary_search(&value).unwrap_or_else(|i| i);
    sorted.insert(pos, value);
}

// STEP 10
fn rotate_and_reverse(v: &mut [u32]) {
    v.rotate_left(2);
    v.reverse();
}

fn swap_ends(v: &mut [u32]) {
    let last = v.len() - 1;
    v.swap(0, last);
}

// STEP 11
fn remove_while_iterating(v: &mut Vec<u32>) -> Vec<u32> {
    let mut removed = Vec::new();
    let mut i = 0;
    while i < v.len() {
        if v[i] % 2 == 0 {
            removed.push(v.remove(i));
        } else {
            i += 1;
        }
    }
    removed
}

// STEP 12
fn double_in_place(v: &mut [u32]) {
    for x in v.iter_mut() {
        *x *= 2;
    }
}

fn pairs_sum_to(v: &[u32], target: u32) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for i in 0..v.len() {
        for j in (i + 1)..v.len() {
            if v[i] + v[j] == target {
                out.push((i, j));
            }
        }
    }
    out
}

fn names(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!("=== VEC MUTATION AND REORDERING ===\n");

    println!("--- Remove: ordered vs fast ---");
    let mut a = names(&["A", "B", "C", "D", "E"]);
    let mut b = a.clone();
    println!("  remove_ordered(1) -> {:?}, left {a:?}", remove_ordered(&mut a, 1));
    println!("  remove_fast(1)    -> {:?}, left {b:?}", remove_fast(&mut b, 1));
    println!("  remove_ordered(9) -> {:?}", remove_ordered(&mut a, 9));
    println!("  remove_fast(9)    -> {:?}", remove_fast(&mut b, 9));

    println!("\n--- Truncate and split_off ---");
    let mut times = vec![10, 20, 30, 40, 50];
    keep_first(&mut times, 3);
    println!("  kept: {times:?}");
    keep_first(&mut times, 10);
    println!("  kept (n too big): {times:?}");
    let mut route = names(&["Chennai", "Katpadi", "Salem", "Erode", "Coimbatore"]);
    let tail = split_route(&mut route, 3);
    println!("  head: {route:?}");
    println!("  tail: {tail:?}");
    let empty = split_route(&mut route, 99);
    println!("  split past end: {empty:?}, head len {}", route.len());

    println!("\n--- Append and extend ---");
    let mut first_half = names(&["Chennai", "Katpadi"]);
    let mut second_half = names(&["Salem", "Erode"]);
    merge(&mut first_half, &mut second_half);
    println!("  merged: {first_half:?}");
    println!("  donor empty: {} (len {})", second_half.is_empty(), second_half.len());
    extend_from(&mut first_half, &["Tiruppur", "Coimbatore"]);
    println!("  extended: {first_half:?}");

    println!("\n--- Retain ---");
    let mut speeds = vec![40, 120, 60, 150, 30, 110];
    let dropped = drop_slow(&mut speeds, 100);
    println!("  dropped {dropped}, kept {speeds:?}");
    println!("  dropped again: {}", drop_slow(&mut speeds, 100));

    println!("\n--- Drain ---");
    let mut stops = names(&["S1", "S2", "S3", "S4", "S5", "S6"]);
    let mid = take_range(&mut stops, 1, 4);
    println!("  drained: {mid:?}");
    println!("  left: {stops:?}");
    let rest = take_all(&mut stops);
    println!("  take_all: {rest:?}, left {} items", stops.len());

    println!("\n--- Dedup ---");
    let mut log = names(&["Salem", "Salem", "Erode", "Salem", "Salem", "Salem"]);
    dedup_consecutive(&mut log);
    println!("  consecutive only: {log:?}");
    let mut platforms = vec![3, 1, 3, 2, 1, 3, 2];
    unique_sorted(&mut platforms);
    println!("  unique_sorted: {platforms:?}");

    println!("\n--- Sorting ---");
    let mut nums = vec![130, 90, 150, 110, 180];
    sort_desc(&mut nums);
    println!("  desc: {nums:?}");
    let mut words = names(&["Kovai", "Duronto", "Rajdhani", "Mail", "Jan", "Goa"]);
    sort_by_len_then_alpha(&mut words);
    println!("  len then alpha: {words:?}");
    sort_by_last_char(&mut words);
    println!("  by last char (stable): {words:?}");
    let mut fares = vec![250.5, 99.0, 1200.25, 99.0, 15.75];
    sort_floats_desc(&mut fares);
    println!("  floats desc: {fares:?}");

    println!("\n--- Binary search ---");
    let mut sorted = vec![10, 20, 30, 40, 50];
    for t in [30, 10, 50, 35, 5, 99] {
        println!("  {}", locate(&sorted, t));
    }
    insert_sorted(&mut sorted, 35);
    insert_sorted(&mut sorted, 5);
    insert_sorted(&mut sorted, 99);
    insert_sorted(&mut sorted, 30);
    println!("  after inserts: {sorted:?}");

    println!("\n--- Rotate, reverse, swap ---");
    let mut r = vec![1, 2, 3, 4, 5, 6];
    rotate_and_reverse(&mut r);
    println!("  rotate_left(2) then reverse: {r:?}");
    swap_ends(&mut r);
    println!("  swap ends: {r:?}");

    println!("\n--- Removing while iterating ---");
    let mut mixed = vec![2, 4, 5, 6, 7, 8, 8, 9];
    let evens = remove_while_iterating(&mut mixed);
    println!("  removed: {evens:?}");
    println!("  remaining: {mixed:?}");

    println!("\n--- In-place edits ---");
    let mut loads = vec![1, 2, 3, 4, 5];
    double_in_place(&mut loads);
    println!("  doubled: {loads:?}");
    println!("  pairs summing to 12: {:?}", pairs_sum_to(&loads, 12));
    println!("  pairs summing to 1: {:?}", pairs_sum_to(&loads, 1));
}
