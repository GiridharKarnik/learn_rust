// =============================================
// SOLUTION 07a - Vec Fundamentals Drills
// =============================================

// STEP 1
fn build_stations() -> Vec<String> {
    vec![
        "Chennai".to_string(),
        "Arakkonam".to_string(),
        "Katpadi".to_string(),
        "Salem".to_string(),
        "Coimbatore".to_string(),
    ]
}

// STEP 2
fn describe_len(items: &[String]) -> String {
    format!("len={} empty={}", items.len(), items.is_empty())
}

// STEP 3
fn safe_get(items: &[String], index: usize) -> String {
    match items.get(index) {
        Some(name) => format!("station {index}: {name}"),
        None => format!("station {index}: out of range"),
    }
}

// STEP 4
fn first_and_last(items: &[String]) -> Option<(&str, &str)> {
    let first = items.first()?;
    let last = items.last()?;
    Some((first.as_str(), last.as_str()))
}

// STEP 5
fn board(queue: &mut Vec<String>, name: &str) {
    queue.push(name.to_string());
}

fn depart(queue: &mut Vec<String>) -> Option<String> {
    queue.pop()
}

// STEP 6
fn priority_board(queue: &mut Vec<String>, name: &str) {
    queue.insert(0, name.to_string());
}

fn cancel(queue: &mut Vec<String>, index: usize) -> Option<String> {
    if index < queue.len() {
        Some(queue.remove(index))
    } else {
        None
    }
}

// STEP 7
fn capacity_demo() -> String {
    let mut v: Vec<u32> = Vec::with_capacity(10);
    let before = v.capacity() >= 10;
    v.push(1);
    v.push(2);
    v.push(3);
    let after = v.capacity() >= 10;
    format!("len={} reserved={before} still_reserved={after}", v.len())
}

// STEP 8
fn sum_speeds(speeds: &[u32]) -> u32 {
    let mut total = 0;
    for s in speeds {
        total += s;
    }
    total
}

fn boost(speeds: &mut [u32], percent: u32) {
    for s in speeds.iter_mut() {
        *s += *s * percent / 100;
    }
}

fn consume_names(names: Vec<String>) -> String {
    let mut out = String::new();
    for (i, name) in names.into_iter().enumerate() {
        if i > 0 {
            out.push('+');
        }
        out.push_str(&name);
    }
    out
}

// STEP 9
fn find_index(items: &[String], name: &str) -> Option<usize> {
    items.iter().position(|s| s == name)
}

// STEP 10
fn make_grid(rows: usize, cols: usize) -> Vec<Vec<bool>> {
    vec![vec![false; cols]; rows]
}

fn book_seat(grid: &mut [Vec<bool>], row: usize, col: usize) -> Result<(), String> {
    let cell = grid
        .get_mut(row)
        .and_then(|r| r.get_mut(col))
        .ok_or(format!("seat ({row},{col}) does not exist"))?;
    if *cell {
        return Err(format!("seat ({row},{col}) already booked"));
    }
    *cell = true;
    Ok(())
}

fn render_grid(grid: &[Vec<bool>]) -> Vec<String> {
    grid.iter()
        .map(|row| row.iter().map(|&b| if b { 'X' } else { '.' }).collect())
        .collect()
}

// STEP 11
fn max_gap(times: &[u32]) -> Option<u32> {
    times.windows(2).map(|w| w[1] - w[0]).max()
}

fn chunk_sums(values: &[u32], size: usize) -> Vec<u32> {
    values.chunks(size).map(|c| c.iter().sum()).collect()
}

// STEP 12
fn first_then_push(items: &mut Vec<String>, extra: &str) -> String {
    let first = items[0].clone();
    items.push(extra.to_string());
    first
}

fn main() {
    println!("=== VEC FUNDAMENTALS DRILLS ===\n");

    println!("--- Stations ---");
    let stations = build_stations();
    println!("  {}", describe_len(&stations));
    println!("  {}", safe_get(&stations, 2));
    println!("  {}", safe_get(&stations, 9));
    match first_and_last(&stations) {
        Some((a, b)) => println!("  route: {a} -> {b}"),
        None => println!("  route: none"),
    }
    match first_and_last(&[]) {
        Some((a, b)) => println!("  route: {a} -> {b}"),
        None => println!("  route: none"),
    }

    println!("\n--- Boarding Queue ---");
    let mut queue: Vec<String> = Vec::new();
    println!("  {}", describe_len(&queue));
    board(&mut queue, "Anand");
    board(&mut queue, "Bala");
    board(&mut queue, "Chitra");
    priority_board(&mut queue, "Dr. Devi");
    println!("  queue: {queue:?}");
    println!("  departed: {:?}", depart(&mut queue));
    println!("  cancelled: {:?}", cancel(&mut queue, 1));
    println!("  cancelled: {:?}", cancel(&mut queue, 7));
    println!("  queue: {queue:?}");
    depart(&mut queue);
    depart(&mut queue);
    println!("  departed: {:?}", depart(&mut queue));

    println!("\n--- Capacity ---");
    println!("  {}", capacity_demo());

    println!("\n--- Three Ways to Iterate ---");
    let mut speeds = vec![100, 120, 150];
    println!("  sum: {}", sum_speeds(&speeds));
    boost(&mut speeds, 10);
    println!("  boosted: {speeds:?}");
    println!("  sum: {}", sum_speeds(&speeds));
    println!("  consumed: {}", consume_names(stations.clone()));
    println!("  still have {} stations", stations.len());

    println!("\n--- Searching ---");
    for name in ["Salem", "Madurai"] {
        match find_index(&stations, name) {
            Some(i) => println!("  {name} is at index {i}"),
            None => println!("  {name} not on this route"),
        }
    }

    println!("\n--- Seat Grid ---");
    let mut grid = make_grid(3, 4);
    for (r, c) in [(0, 0), (1, 2), (2, 3), (1, 2), (3, 0), (0, 9)] {
        match book_seat(&mut grid, r, c) {
            Ok(()) => println!("  booked ({r},{c})"),
            Err(e) => println!("  error: {e}"),
        }
    }
    for line in render_grid(&grid) {
        println!("  {line}");
    }

    println!("\n--- Slices ---");
    let times = [0, 25, 70, 90, 150, 160];
    println!("  max gap: {:?}", max_gap(&times));
    println!("  max gap (one stop): {:?}", max_gap(&times[..1]));
    println!("  chunk sums: {:?}", chunk_sums(&[1, 2, 3, 4, 5, 6, 7], 3));

    println!("\n--- Borrow Trap ---");
    let mut route = build_stations();
    let first = first_then_push(&mut route, "Madurai");
    println!("  first was {first}, route now has {} stops", route.len());
    println!("  last: {:?}", route.last());
}
