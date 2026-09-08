// =============================================
// EXAMPLES 07: Vec and Slices
// =============================================
//
// Run with: cargo run
//
// These examples demonstrate every major Vec and slice operation
// using a railway theme. Read through them, then try the exercise!

#[derive(Debug, Clone)]
struct Train {
    name: String,
    number: u32,
    speed_kmh: u32,
    is_active: bool,
}

impl Train {
    fn new(name: &str, number: u32, speed_kmh: u32, is_active: bool) -> Self {
        Train {
            name: name.to_string(),
            number,
            speed_kmh,
            is_active,
        }
    }
}

impl std::fmt::Display for Train {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "#{} {} ({} km/h, {})",
            self.number,
            self.name,
            self.speed_kmh,
            if self.is_active { "active" } else { "inactive" }
        )
    }
}

fn main() {
    // =========================================
    // 1. CREATING VECS
    // =========================================
    println!("=== 1. Creating Vecs ===\n");

    // Empty Vec with explicit type
    let mut trains: Vec<Train> = Vec::new();
    println!("Empty roster: {:?} (len={})", trains, trains.len());

    // Using vec![] macro — most common way
    let platforms = vec![1, 2, 3, 4, 5];
    println!("Platforms: {:?}", platforms);

    // With pre-allocated capacity (performance hint)
    let mut schedule = Vec::with_capacity(10);
    println!(
        "Schedule: len={}, capacity={} (allocated but empty)",
        schedule.len(),
        schedule.capacity()
    );
    schedule.push("06:00 Rajdhani Express");
    schedule.push("07:30 Shatabdi Express");
    println!(
        "Schedule: len={}, capacity={} (still room to grow without reallocation)",
        schedule.len(),
        schedule.capacity()
    );

    // =========================================
    // 2. PUSH, POP, INSERT, REMOVE
    // =========================================
    println!("\n=== 2. Push, Pop, Insert, Remove ===\n");

    // Push adds to the end
    trains.push(Train::new("Shatabdi Express", 12007, 150, true));
    trains.push(Train::new("Rajdhani Express", 12001, 130, true));
    trains.push(Train::new("Duronto Express", 12245, 120, true));
    println!("After 3 pushes: {} trains", trains.len());
    for t in &trains {
        println!("  {}", t);
    }

    // Insert at a specific index (shifts elements right)
    trains.insert(0, Train::new("Vande Bharat Express", 22436, 180, true));
    println!("\nAfter insert at index 0:");
    for t in &trains {
        println!("  {}", t);
    }

    // Pop removes the last element — returns Option<T>!
    let popped = trains.pop();
    println!("\nPopped: {:?}", popped.map(|t| t.name));

    // Pop on empty Vec returns None, no panic
    let empty_vec: Vec<Train> = Vec::new();
    let nothing = empty_vec.into_iter().last(); // can also show pop on empty
    println!("Pop from empty: {:?}", nothing);

    // Remove at index (shifts elements left, panics if out of bounds)
    let removed = trains.remove(1); // removes "Shatabdi Express"
    println!("Removed index 1: {}", removed.name);
    println!("Remaining: {} trains", trains.len());

    // Retain — keep only elements matching a condition (in place!)
    trains.push(Train::new("Kovai Express", 12675, 110, false));
    trains.push(Train::new("Island Express", 16525, 100, false));
    println!("\nBefore retain ({} trains):", trains.len());
    for t in &trains {
        println!("  {}", t);
    }
    trains.retain(|t| t.is_active);
    println!("After retain (active only): {} trains", trains.len());
    for t in &trains {
        println!("  {}", t);
    }

    // =========================================
    // 3. ACCESSING SAFELY — .get() vs []
    // =========================================
    println!("\n=== 3. Accessing Safely ===\n");

    let stations = vec!["Chennai", "Katpadi", "Jolarpettai", "Bangalore"];

    // Direct indexing — works when you KNOW the index is valid
    println!("First station: {}", stations[0]);
    println!("Last station:  {}", stations[stations.len() - 1]);

    // .get() returns Option<&T> — safe!
    match stations.get(2) {
        Some(station) => println!("Station at index 2: {}", station),
        None => println!("No station at index 2"),
    }
    match stations.get(99) {
        Some(station) => println!("Station at index 99: {}", station),
        None => println!("No station at index 99 (None, not a panic!)"),
    }

    // .first() and .last() — also return Option
    println!("First: {:?}", stations.first());
    println!("Last:  {:?}", stations.last());

    // .contains() — check if a value exists
    println!("Has 'Katpadi'?    {}", stations.contains(&"Katpadi"));
    println!("Has 'Vijayawada'? {}", stations.contains(&"Vijayawada"));

    // .is_empty() and .len()
    println!("Stations count: {}", stations.len());
    println!("Is empty? {}", stations.is_empty());

    // =========================================
    // 4. ITERATING — borrow, mut borrow, consume
    // =========================================
    println!("\n=== 4. Iterating ===\n");

    let trains = vec![
        Train::new("Shatabdi Express", 12007, 150, true),
        Train::new("Rajdhani Express", 12001, 130, true),
        Train::new("Duronto Express", 12245, 120, true),
    ];

    // Borrow iteration — most common, Vec still usable after
    println!("--- Borrow (&trains) ---");
    for train in &trains {
        println!("  {} — {} km/h", train.name, train.speed_kmh);
    }
    println!("  (trains still has {} items)\n", trains.len());

    // Mutable borrow — modify in place
    println!("--- Mut borrow (&mut trains) ---");
    let mut trains = trains; // rebind as mutable
    for train in &mut trains {
        train.speed_kmh += 10; // speed boost!
    }
    for train in &trains {
        println!("  {} — now {} km/h", train.name, train.speed_kmh);
    }
    println!();

    // With index using .enumerate()
    println!("--- Enumerate ---");
    for (i, train) in trains.iter().enumerate() {
        println!("  Platform {}: {}", i + 1, train.name);
    }
    println!();

    // Consuming iteration — Vec is GONE after this
    println!("--- Consume (into trains) ---");
    let train_names: Vec<String> = trains.into_iter().map(|t| t.name).collect();
    println!("  Collected names: {:?}", train_names);
    // println!("{:?}", trains);  // ❌ Would error — trains was consumed!

    // =========================================
    // 5. SLICES
    // =========================================
    println!("\n=== 5. Slices ===\n");

    let stops = vec![
        "Chennai",
        "Katpadi",
        "Jolarpettai",
        "Salem",
        "Erode",
        "Coimbatore",
    ];

    // Slice syntax: &vec[start..end] (end is exclusive)
    let first_three = &stops[0..3];
    let middle = &stops[2..5];
    let last_two = &stops[4..];
    let everything = &stops[..];

    println!("All stops:   {:?}", everything);
    println!("First three: {:?}", first_three);
    println!("Middle:      {:?}", middle);
    println!("Last two:    {:?}", last_two);

    // Slices as function parameters — the idiomatic way
    fn count_stops(stops: &[&str]) -> usize {
        stops.len()
    }

    // Works with full Vec (auto-coerces to slice)
    println!("\nFull route: {} stops", count_stops(&stops));
    // Works with a sub-slice too!
    println!("Partial route: {} stops", count_stops(&stops[1..4]));

    // A more realistic example: function taking &[Train]
    fn fastest_in_group(trains: &[Train]) -> Option<&Train> {
        let mut fastest: Option<&Train> = None;
        for train in trains {
            match fastest {
                None => fastest = Some(train),
                Some(f) if train.speed_kmh > f.speed_kmh => fastest = Some(train),
                _ => {}
            }
        }
        fastest
    }

    let fleet = vec![
        Train::new("Shatabdi Express", 12007, 150, true),
        Train::new("Rajdhani Express", 12001, 130, true),
        Train::new("Vande Bharat", 22436, 180, true),
        Train::new("Kovai Express", 12675, 110, true),
    ];

    // Pass the full Vec
    if let Some(f) = fastest_in_group(&fleet) {
        println!("Fastest overall: {} ({} km/h)", f.name, f.speed_kmh);
    }
    // Pass just a slice
    if let Some(f) = fastest_in_group(&fleet[0..2]) {
        println!("Fastest in first two: {} ({} km/h)", f.name, f.speed_kmh);
    }

    // =========================================
    // 6. SORTING
    // =========================================
    println!("\n=== 6. Sorting ===\n");

    let mut speeds = vec![160, 110, 200, 130, 150];
    println!("Before sort: {:?}", speeds);

    // sort() — ascending, in place
    speeds.sort();
    println!("Ascending:   {:?}", speeds);

    // sort_by() — custom comparator
    speeds.sort_by(|a, b| b.cmp(a));
    println!("Descending:  {:?}", speeds);

    // sort_by_key() — sort structs by a field
    let mut fleet = vec![
        Train::new("Shatabdi Express", 12007, 150, true),
        Train::new("Rajdhani Express", 12001, 130, true),
        Train::new("Vande Bharat", 22436, 180, true),
        Train::new("Kovai Express", 12675, 110, true),
    ];

    fleet.sort_by_key(|t| t.speed_kmh);
    println!("\nSorted by speed (ascending):");
    for t in &fleet {
        println!("  {} — {} km/h", t.name, t.speed_kmh);
    }

    // Sort descending by speed using sort_by
    fleet.sort_by(|a, b| b.speed_kmh.cmp(&a.speed_kmh));
    println!("\nSorted by speed (descending):");
    for t in &fleet {
        println!("  {} — {} km/h", t.name, t.speed_kmh);
    }

    // =========================================
    // 7. USEFUL METHODS — retain, dedup, etc.
    // =========================================
    println!("\n=== 7. Useful Methods ===\n");

    // --- retain ---
    let mut fleet = vec![
        Train::new("Shatabdi Express", 12007, 150, true),
        Train::new("Old Steam", 1001, 60, false),
        Train::new("Rajdhani Express", 12001, 130, true),
        Train::new("Heritage Special", 1002, 40, false),
    ];
    println!("Before retain: {} trains", fleet.len());
    fleet.retain(|t| t.is_active);
    println!("After retain (active only): {} trains", fleet.len());
    for t in &fleet {
        println!("  {}", t);
    }

    // --- dedup (removes consecutive duplicates — sort first for all dupes!) ---
    println!();
    let mut stops = vec![
        "Chennai", "Chennai", "Katpadi", "Salem", "Salem", "Salem", "Erode",
    ];
    println!("Before dedup: {:?}", stops);
    stops.dedup();
    println!("After dedup:  {:?}", stops);

    // --- contains ---
    println!("\nDoes route include 'Salem'? {}", stops.contains(&"Salem"));
    println!("Does route include 'Mumbai'? {}", stops.contains(&"Mumbai"));

    // --- reverse ---
    stops.reverse();
    println!("\nReversed route: {:?}", stops);

    // --- truncate ---
    stops.truncate(3);
    println!("First 3 stops: {:?}", stops);

    // --- split_at ---
    let platforms = vec![1, 2, 3, 4, 5, 6];
    let (domestic, international) = platforms.split_at(4);
    println!("\nDomestic platforms: {:?}", domestic);
    println!("International platforms: {:?}", international);

    // --- windows and chunks ---
    println!("\nSliding windows of 3:");
    for window in platforms.windows(3) {
        println!("  {:?}", window);
    }

    println!("\nChunks of 2:");
    for chunk in platforms.chunks(2) {
        println!("  {:?}", chunk);
    }

    // =========================================
    // SUMMARY
    // =========================================
    println!("\n=== Summary ===");
    println!("Vec<T>: growable, heap-allocated, owns its elements.");
    println!("&[T]:   a borrowed view into a Vec (or array). Zero-copy.");
    println!("Use .get() for safe access, &[T] in function params,");
    println!("and remember: for x in &vec borrows, for x in vec consumes!");
}
