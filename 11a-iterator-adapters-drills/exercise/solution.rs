// =============================================
// SOLUTION 11a - Iterator Adapters Drills
// =============================================

// STEP 1
fn first_over(speeds: &[u32], limit: u32) -> Option<usize> {
    let mut i = 0;
    loop {
        if i >= speeds.len() {
            break None;
        }
        if speeds[i] > limit {
            break Some(i);
        }
        i += 1;
    }
}

// STEP 2
fn find_free_seat(grid: &[Vec<bool>]) -> Option<(usize, usize)> {
    'search: {
        for (r, row) in grid.iter().enumerate() {
            for (c, &taken) in row.iter().enumerate() {
                if !taken {
                    break 'search Some((r, c));
                }
            }
        }
        None
    }
}

fn count_empty_rows(grid: &[Vec<bool>]) -> usize {
    let mut count = 0;
    'rows: for row in grid {
        for &taken in row {
            if taken {
                continue 'rows;
            }
        }
        count += 1;
    }
    count
}

// STEP 3
fn trips_needed(mut distance: u32, per_trip: u32) -> u32 {
    let mut trips = 0;
    while distance > 0 {
        distance = distance.saturating_sub(per_trip);
        trips += 1;
    }
    trips
}

// STEP 4
fn double_in_place(values: &mut [u32]) {
    for v in values.iter_mut() {
        *v *= 2;
    }
}

fn total_chars(names: &[String]) -> usize {
    names.iter().map(|n| n.len()).sum()
}

fn shout_all(names: Vec<String>) -> Vec<String> {
    names.into_iter().map(|n| n.to_uppercase()).collect()
}

// STEP 5
fn odd_numbered_stops(stops: &[&str]) -> Vec<String> {
    stops
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(i, s)| format!("{}:{}", i + 1, s))
        .collect()
}

// STEP 6
fn parse_delays(raw: &[&str]) -> Vec<u32> {
    raw.iter().filter_map(|s| s.parse().ok()).collect()
}

fn all_stops<'a>(routes: &[Vec<&'a str>]) -> Vec<&'a str> {
    routes.iter().flatten().copied().collect()
}

fn letters_of(codes: &[&str]) -> Vec<char> {
    codes.iter().flat_map(|c| c.chars()).collect()
}

fn present_only(slots: &[Option<u32>]) -> Vec<u32> {
    slots.iter().flatten().copied().collect()
}

// STEP 7
fn gaps(times: &[u32]) -> Vec<u32> {
    times.iter().zip(times.iter().skip(1)).map(|(a, b)| b - a).collect()
}

fn split_pairs(pairs: &[(String, u32)]) -> (Vec<String>, Vec<u32>) {
    pairs.iter().cloned().unzip()
}

// STEP 8
fn head_and_tail(v: &[u32]) -> Vec<u32> {
    v.iter().take(2).chain(v.iter().skip(v.len().saturating_sub(2))).copied().collect()
}

fn every_third(v: &[u32]) -> Vec<u32> {
    v.iter().step_by(3).copied().collect()
}

fn countdown(from: u32) -> Vec<u32> {
    (1..=from).rev().collect()
}

// STEP 9
fn leading_on_time(delays: &[u32]) -> Vec<u32> {
    delays.iter().copied().take_while(|&d| d == 0).collect()
}

fn after_leading_on_time(delays: &[u32]) -> Vec<u32> {
    delays.iter().copied().skip_while(|&d| d == 0).collect()
}

// STEP 10
fn run_lengths(v: &[u32]) -> Vec<(u32, usize)> {
    let mut out = Vec::new();
    let mut it = v.iter().peekable();
    while let Some(&x) = it.next() {
        let mut n = 1;
        while it.peek() == Some(&&x) {
            it.next();
            n += 1;
        }
        out.push((x, n));
    }
    out
}

// STEP 11
fn running_totals(v: &[u32]) -> Vec<u32> {
    v.iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect()
}

fn trace_evens(v: &[u32], log: &mut Vec<String>) -> Vec<u32> {
    v.iter()
        .copied()
        .inspect(|x| log.push(format!("saw {x}")))
        .filter(|x| x % 2 == 0)
        .collect()
}

// STEP 12
fn long_names(names: &[String], min: usize) -> impl Iterator<Item = &str> + '_ {
    names.iter().map(|s| s.as_str()).filter(move |s| s.len() >= min)
}

fn multiples_of(n: u32, count: usize) -> impl Iterator<Item = u32> {
    (1..).map(move |i| i * n).take(count)
}

// STEP 13
fn weighted_sum(v: &[u32]) -> u32 {
    v.iter().enumerate().map(|(i, &x)| i as u32 * x).sum()
}

fn dot(a: &[u32], b: &[u32]) -> u32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

fn first_long_index(names: &[String], min: usize) -> Option<usize> {
    names.iter().position(|n| n.len() >= min)
}

// STEP 14
fn main() {
    println!("=== ITERATOR ADAPTERS DRILLS ===");

    println!("\n--- Loops ---");
    let speeds = [80, 95, 120, 60, 140];
    println!("  first over 100: {:?}", first_over(&speeds, 100));
    println!("  first over 200: {:?}", first_over(&speeds, 200));
    let grid = vec![
        vec![true, true, true],
        vec![true, false, true],
        vec![false, false, false],
    ];
    println!("  free seat: {:?}", find_free_seat(&grid));
    println!("  free seat (full): {:?}", find_free_seat(&[vec![true, true]]));
    println!("  empty rows: {}", count_empty_rows(&grid));
    println!("  trips for 1000 km at 300: {}", trips_needed(1000, 300));

    println!("\n--- Three Iterators ---");
    let mut fares = vec![10, 20, 35];
    double_in_place(&mut fares);
    println!("  doubled: {fares:?}");
    let names: Vec<String> = ["Salem", "Erode", "Karur", "Trichy"].iter().map(|s| s.to_string()).collect();
    println!("  total chars: {}", total_chars(&names));
    println!("  shouted: {:?}", shout_all(names.clone()));

    println!("\n--- Map / Filter / Enumerate ---");
    let stops = ["Chennai", "Arakkonam", "Katpadi", "Jolarpet", "Salem"];
    println!("  {:?}", odd_numbered_stops(&stops));

    println!("\n--- Flattening ---");
    println!("  delays: {:?}", parse_delays(&["5", "x", "12", "", "0", "-3"]));
    let routes = vec![vec!["A", "B"], vec![], vec!["C"]];
    println!("  stops: {:?}", all_stops(&routes));
    println!("  letters: {:?}", letters_of(&["MAS", "SBC"]));
    println!("  present: {:?}", present_only(&[Some(1), None, Some(3)]));

    println!("\n--- Zip / Unzip ---");
    println!("  gaps: {:?}", gaps(&[0, 25, 70, 90]));
    println!("  gaps (one): {:?}", gaps(&[5]));
    let pairs = vec![("MAS".to_string(), 10), ("SBC".to_string(), 20)];
    let (codes, platforms) = split_pairs(&pairs);
    println!("  codes={codes:?} platforms={platforms:?}");

    println!("\n--- Slicing Streams ---");
    let nums = [1, 2, 3, 4, 5, 6, 7];
    println!("  head+tail: {:?}", head_and_tail(&nums));
    println!("  head+tail (short): {:?}", head_and_tail(&[9]));
    println!("  every third: {:?}", every_third(&nums));
    println!("  countdown: {:?}", countdown(4));

    println!("\n--- While Conditions ---");
    let delays = [0, 0, 7, 0, 3];
    println!("  leading on time: {:?}", leading_on_time(&delays));
    println!("  after: {:?}", after_leading_on_time(&delays));

    println!("\n--- Peekable ---");
    println!("  runs: {:?}", run_lengths(&[1, 1, 2, 3, 3, 3, 1]));

    println!("\n--- Scan / Inspect ---");
    println!("  totals: {:?}", running_totals(&[4, 1, 5, 2]));
    let mut log = Vec::new();
    let evens = trace_evens(&[1, 2, 3, 4], &mut log);
    println!("  evens: {evens:?}");
    println!("  log: {}", log.join(", "));

    println!("\n--- Returning Iterators ---");
    let long: Vec<&str> = long_names(&names, 5).collect();
    println!("  long names: {long:?}");
    println!("  multiples: {:?}", multiples_of(7, 4).collect::<Vec<_>>());

    println!("\n--- Index Loops Replaced ---");
    println!("  weighted: {}", weighted_sum(&[5, 10, 15]));
    println!("  dot: {}", dot(&[1, 2, 3], &[4, 5, 6]));
    println!("  first long: {:?}", first_long_index(&names, 6));
    println!("  first huge: {:?}", first_long_index(&names, 60));
}
