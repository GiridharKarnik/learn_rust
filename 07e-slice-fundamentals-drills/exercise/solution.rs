// Solution 07e: Slice Fundamentals Drills

fn sum(xs: &[u32]) -> u32 {
    let mut total = 0;
    for x in xs {
        total += x;
    }
    total
}

fn describe(xs: &[u32]) -> String {
    format!("len={} first={:?} last={:?}", xs.len(), xs.first(), xs.last())
}

fn max_speed(xs: &[u32]) -> Option<u32> {
    xs.iter().copied().max()
}

fn middle(xs: &[u32]) -> &[u32] {
    if xs.len() <= 2 {
        &[]
    } else {
        &xs[1..xs.len() - 1]
    }
}

fn safe_slice(xs: &[u32], start: usize, end: usize) -> Option<&[u32]> {
    xs.get(start..end)
}

fn take_front(xs: &[u32], n: usize) -> &[u32] {
    &xs[..n.min(xs.len())]
}

fn scale(xs: &mut [u32], factor: u32) {
    for x in xs.iter_mut() {
        *x *= factor;
    }
}

fn cap_speeds(xs: &mut [u32], limit: u32) -> usize {
    let mut changed = 0;
    for x in xs.iter_mut() {
        if *x > limit {
            *x = limit;
            changed += 1;
        }
    }
    changed
}

fn swap_ends(xs: &mut [u32]) {
    if xs.len() >= 2 {
        let last = xs.len() - 1;
        xs.swap(0, last);
    }
}

fn zero_range(xs: &mut [u32], from: usize, to: usize) {
    xs[from..to].fill(0);
}

fn overwrite_prefix(dst: &mut [u32], src: &[u32]) -> bool {
    if src.len() > dst.len() {
        return false;
    }
    dst[..src.len()].copy_from_slice(src);
    true
}

fn route_checks(route: &[&str]) -> String {
    format!(
        "starts={} ends={} has_salem={} has_madurai={}",
        route.starts_with(&["Chennai"]),
        route.ends_with(&["Coimbatore"]),
        route.contains(&"Salem"),
        route.contains(&"Madurai"),
    )
}

fn sort_desc(xs: &mut [u32]) {
    xs.sort_by(|a, b| b.cmp(a));
}

fn find_sorted(xs: &[u32], target: u32) -> String {
    match xs.binary_search(&target) {
        Ok(i) => format!("{target} found at {i}"),
        Err(i) => format!("{target} missing, would insert at {i}"),
    }
}

fn head_sum_then_push(v: &mut Vec<u32>, extra: u32) -> u32 {
    let head_sum = sum(&v[..2]);
    v.push(extra);
    head_sum
}

fn main() {
    println!("=== SLICE FUNDAMENTALS DRILLS ===");

    println!("\n--- Reading Slices ---");
    let times = vec![0, 25, 70, 90, 150, 160];
    println!("  {}", describe(&times));
    println!("  {}", describe(&times[2..4]));
    println!("  {}", describe(&[]));
    println!("  sum={} max={:?}", sum(&times), max_speed(&times));
    println!("  max of empty: {:?}", max_speed(&[]));

    println!("\n--- Sub-slices ---");
    println!("  middle: {:?}", middle(&times));
    println!("  middle of two: {:?}", middle(&times[..2]));
    println!("  front 3: {:?}", take_front(&times, 3));
    println!("  front 99: {:?}", take_front(&times, 99));
    for (s, e) in [(1, 3), (4, 9), (5, 2)] {
        match safe_slice(&times, s, e) {
            Some(part) => println!("  {s}..{e} -> {part:?}"),
            None => println!("  {s}..{e} -> invalid"),
        }
    }

    println!("\n--- Mutating Slices ---");
    let mut speeds = vec![60, 90, 120, 150, 110];
    scale(&mut speeds, 2);
    println!("  scaled: {speeds:?}");
    let changed = cap_speeds(&mut speeds, 200);
    println!("  capped {changed}: {speeds:?}");
    swap_ends(&mut speeds);
    println!("  swapped: {speeds:?}");
    scale(&mut speeds[1..3], 10);
    println!("  scaled middle: {speeds:?}");
    zero_range(&mut speeds, 3, 5);
    println!("  zeroed tail: {speeds:?}");
    let mut tiny = [7];
    swap_ends(&mut tiny);
    println!("  tiny: {tiny:?}");

    println!("\n--- copy_from_slice ---");
    let mut buffer = [0u32; 6];
    println!("  fit: {}", overwrite_prefix(&mut buffer, &[5, 6, 7]));
    println!("  buffer: {buffer:?}");
    println!("  fit: {}", overwrite_prefix(&mut buffer[..2], &[1, 2, 3]));
    println!("  buffer: {buffer:?}");

    println!("\n--- Searching Routes ---");
    let route = ["Chennai", "Arakkonam", "Katpadi", "Salem", "Coimbatore"];
    println!("  {}", route_checks(&route));
    println!("  {}", route_checks(&route[1..4]));

    println!("\n--- Sorting and Binary Search ---");
    let mut delays = vec![15, 3, 42, 8, 27, 3];
    delays.sort();
    println!("  sorted: {delays:?}");
    for t in [27, 10, 99] {
        println!("  {}", find_sorted(&delays, t));
    }
    sort_desc(&mut delays[1..]);
    println!("  tail desc: {delays:?}");

    println!("\n--- Borrow Trap ---");
    let mut v = vec![10, 20, 30];
    let head = head_sum_then_push(&mut v, 99);
    println!("  head sum {head}, vec now {v:?}");
}
