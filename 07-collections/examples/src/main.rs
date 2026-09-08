use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Train {
    name: String,
    number: u32,
    speed_kmh: f64,
    route: String,
}

fn main() {
    // =========================================================================
    // 1. 🚂 Vec Basics
    // =========================================================================
    println!("=== 1. 🚂 Vec Basics ===\n");

    // Create with vec! macro
    let mut platforms: Vec<u32> = vec![1, 2, 3, 4, 5];
    println!("Platforms: {:?}", platforms);

    // push — add to the end
    platforms.push(6);
    println!("After push(6): {:?}", platforms);

    // pop — remove from the end, returns Option<T>
    let popped: Option<u32> = platforms.pop();
    println!("Popped: {:?} (it's an Option!)", popped);
    println!("After pop: {:?}", platforms);

    // insert — insert at index, shifting elements right
    platforms.insert(0, 0); // insert platform 0 at the front
    println!("After insert(0, 0): {:?}", platforms);

    // remove — remove at index, shifting elements left
    let removed = platforms.remove(0);
    println!("Removed index 0: {} → {:?}", removed, platforms);

    // len and is_empty
    println!("Number of platforms: {}", platforms.len());
    println!("Is empty? {}", platforms.is_empty());

    let empty_vec: Vec<u32> = Vec::new();
    println!("Empty vec is_empty? {}", empty_vec.is_empty());

    // =========================================================================
    // 2. 🔍 Accessing Elements
    // =========================================================================
    println!("\n=== 2. 🔍 Accessing Elements ===\n");

    let coaches = vec!["S1", "S2", "S3", "A1", "B1", "B2"];

    // Direct indexing — panics if out of bounds!
    println!("Coach at index 0: {}", coaches[0]);
    println!("Coach at index 3: {}", coaches[3]);

    // .get() — returns Option<&T>, safe for unknown indices
    match coaches.get(2) {
        Some(coach) => println!(".get(2) = Some(\"{}\")", coach),
        None => println!(".get(2) = None"),
    }
    match coaches.get(99) {
        Some(coach) => println!(".get(99) = Some(\"{}\")", coach),
        None => println!(".get(99) = None — no panic!"),
    }

    // .first() and .last() — also return Option<&T>
    println!("First coach: {:?}", coaches.first());
    println!("Last coach: {:?}", coaches.last());

    // Slices — borrow a portion of the vec
    let sleeper_coaches = &coaches[0..3]; // S1, S2, S3
    println!("Sleeper coaches (slice [0..3]): {:?}", sleeper_coaches);
    let ac_coaches = &coaches[3..];
    println!("AC coaches (slice [3..]): {:?}", ac_coaches);

    // =========================================================================
    // 3. 🔄 Vec Iteration
    // =========================================================================
    println!("\n=== 3. 🔄 Vec Iteration ===\n");

    // Immutable borrow — read-only access
    let stations = vec![
        String::from("Chennai Central"),
        String::from("Katpadi Junction"),
        String::from("Bangalore City"),
    ];

    print!("Stations (borrowed): ");
    for station in &stations {
        print!("[{}] ", station);
    }
    println!();
    // stations is still usable here because we only borrowed
    println!("Vec still accessible, len = {}", stations.len());

    // Mutable borrow — modify elements in place
    let mut speeds = vec![80.0, 110.0, 130.0, 95.0];
    println!("\nOriginal speeds: {:?}", speeds);
    for speed in &mut speeds {
        *speed *= 1.1; // 10% speed boost!
    }
    println!("After 10% boost: {:?}", speeds);

    // Consuming iteration — vec is moved, cannot be used after
    let trains_to_announce = vec!["Rajdhani Express", "Shatabdi Express", "Duronto Express"];
    print!("\nAnnouncing (consuming): ");
    for train in trains_to_announce {
        // train is an owned String here, not a reference
        print!("📢 {} | ", train);
    }
    println!();
    // trains_to_announce is no longer accessible here!

    // =========================================================================
    // 4. 📖 HashMap Basics
    // =========================================================================
    println!("\n=== 4. 📖 HashMap Basics ===\n");

    let mut train_numbers: HashMap<String, u32> = HashMap::new();

    // insert
    train_numbers.insert(String::from("Rajdhani Express"), 12301);
    train_numbers.insert(String::from("Shatabdi Express"), 12001);
    train_numbers.insert(String::from("Duronto Express"), 12213);
    train_numbers.insert(String::from("Garib Rath"), 12203);

    println!("Train numbers map: {:?}", train_numbers);

    // get — returns Option<&V>
    let key = String::from("Rajdhani Express");
    match train_numbers.get(&key) {
        Some(number) => println!("Rajdhani Express → #{}", number),
        None => println!("Rajdhani Express not found"),
    }

    match train_numbers.get("Vande Bharat") {
        Some(number) => println!("Vande Bharat → #{}", number),
        None => println!("Vande Bharat → not found"),
    }

    // contains_key
    println!(
        "Has Shatabdi? {}",
        train_numbers.contains_key("Shatabdi Express")
    );
    println!(
        "Has Vande Bharat? {}",
        train_numbers.contains_key("Vande Bharat")
    );

    // remove — returns Option<V>
    let removed = train_numbers.remove("Garib Rath");
    println!("Removed Garib Rath: {:?}", removed);
    println!("Map after removal: {:?}", train_numbers);

    // iterate over key-value pairs
    println!("\nAll trains:");
    for (name, number) in &train_numbers {
        println!("  🚆 {} (Train #{})", name, number);
    }

    // =========================================================================
    // 5. 🔑 HashMap Entry API
    // =========================================================================
    println!("\n=== 5. 🔑 HashMap Entry API ===\n");

    // Count station occurrences in a route
    let stations = vec!["Chennai", "Katpadi", "Chennai", "Bangalore", "Chennai"];
    let mut counts: HashMap<&str, u32> = HashMap::new();

    for station in &stations {
        // entry() returns an Entry enum — if key is missing, or_insert sets the default
        // The returned mutable reference lets us increment in place
        *counts.entry(*station).or_insert(0) += 1;
    }

    println!("Station visit counts: {:?}", counts);
    println!("Chennai was visited {} times", counts["Chennai"]);

    // or_insert_with — use a closure to compute the default value
    let mut schedules: HashMap<&str, Vec<&str>> = HashMap::new();
    let departures = vec![
        ("Platform 1", "Rajdhani"),
        ("Platform 2", "Shatabdi"),
        ("Platform 1", "Duronto"),
        ("Platform 2", "Garib Rath"),
    ];

    for (platform, train) in &departures {
        schedules
            .entry(platform)
            .or_insert_with(Vec::new)
            .push(train);
    }

    println!("\nPlatform schedules:");
    for (platform, trains) in &schedules {
        println!("  {} → {:?}", platform, trains);
    }

    // =========================================================================
    // 6. ⚙️ Iterator Adaptors
    // =========================================================================
    println!("\n=== 6. ⚙️ Iterator Adaptors ===\n");

    let speeds_kmh = vec![80, 110, 130, 95, 160, 75, 140];

    // .map() — transform each element
    let speeds_mph: Vec<f64> = speeds_kmh
        .iter()
        .map(|&kmh| kmh as f64 * 0.621371)
        .collect();
    println!(
        "Speeds in km/h: {:?}\nSpeeds in mph:   {:?}",
        speeds_kmh, speeds_mph
    );

    // .filter() — keep only elements matching a predicate
    let fast_trains: Vec<&i32> = speeds_kmh.iter().filter(|&&s| s > 100).collect();
    println!("Fast trains (>100 km/h): {:?}", fast_trains);

    // .enumerate() — get (index, value) pairs
    println!("\nEnumerated speeds:");
    for (i, speed) in speeds_kmh.iter().enumerate() {
        println!("  Train #{}: {} km/h", i + 1, speed);
    }

    // .zip() — pair up elements from two iterators
    let train_names = vec!["Rajdhani", "Shatabdi", "Duronto", "Garib Rath"];
    let train_speeds = vec![130, 150, 140, 110];
    let paired: Vec<(&&str, &i32)> = train_names.iter().zip(train_speeds.iter()).collect();
    println!("\nZipped (name, speed): {:?}", paired);

    // .take() and .skip()
    println!(
        "\nFirst 3 speeds (take): {:?}",
        speeds_kmh.iter().take(3).collect::<Vec<_>>()
    );
    println!(
        "Skip first 4 speeds:  {:?}",
        speeds_kmh.iter().skip(4).collect::<Vec<_>>()
    );

    // .chain() — concatenate two iterators
    let express_trains = vec!["Rajdhani", "Shatabdi"];
    let local_trains = vec!["EMU Local", "MEMU"];
    let all_trains: Vec<&&str> = express_trains.iter().chain(local_trains.iter()).collect();
    println!("Chained trains: {:?}", all_trains);

    // =========================================================================
    // 7. 📊 Iterator Consumers
    // =========================================================================
    println!("\n=== 7. 📊 Iterator Consumers ===\n");

    let distances_km = vec![350, 150, 560, 280, 430];

    // .collect() — already shown above, gathers into a collection

    // .count()
    println!("Number of routes: {}", distances_km.iter().count());

    // .sum()
    let total: i32 = distances_km.iter().sum();
    println!("Total distance: {} km", total);

    // .min() and .max() — return Option
    println!("Shortest route: {:?} km", distances_km.iter().min());
    println!("Longest route:  {:?} km", distances_km.iter().max());

    // .find() — returns Option of first match
    let first_long = distances_km.iter().find(|&&d| d > 400);
    println!("First route > 400 km: {:?}", first_long);

    // .any() — returns bool, true if any element matches
    let has_short = distances_km.iter().any(|&d| d < 200);
    println!("Any route < 200 km? {}", has_short);

    // .all() — returns bool, true if all elements match
    let all_positive = distances_km.iter().all(|&d| d > 0);
    println!("All routes positive? {}", all_positive);

    // .fold() — accumulate with an initial value
    let route_summary = distances_km.iter().fold(String::new(), |mut acc, &d| {
        if !acc.is_empty() {
            acc.push_str(" → ");
        }
        acc.push_str(&format!("{}km", d));
        acc
    });
    println!("Route summary: {}", route_summary);

    // =========================================================================
    // 8. 🎯 Collecting Into Different Types
    // =========================================================================
    println!("\n=== 8. 🎯 Collecting Into Different Types ===\n");

    // Into a Vec
    let doubled: Vec<i32> = distances_km.iter().map(|&d| d * 2).collect();
    println!("Doubled distances (Vec): {:?}", doubled);

    // Into a HashMap (from iterator of tuples)
    let names = vec!["Rajdhani", "Shatabdi", "Duronto"];
    let numbers = vec![12301, 12001, 12213];
    let train_map: HashMap<&str, i32> =
        names.iter().copied().zip(numbers.iter().copied()).collect();
    println!("Train map (HashMap): {:?}", train_map);

    // Into a String
    let coach_labels = vec!["S1", "S2", "A1", "B1"];
    let joined: String = coach_labels.join(" - ");
    println!("Coach string: {}", joined);

    // Collecting with iterators into a String
    let upper_coaches: String = coach_labels
        .iter()
        .map(|c| c.to_uppercase())
        .collect::<Vec<_>>()
        .join(", ");
    println!("Uppercased coaches: {}", upper_coaches);

    // =========================================================================
    // 9. 🔗 Chaining Iterators — Realistic Example
    // =========================================================================
    println!("\n=== 9. 🔗 Chaining Iterators — Realistic Example ===\n");

    let fleet = vec![
        Train {
            name: String::from("Rajdhani Express"),
            number: 12301,
            speed_kmh: 130.0,
            route: String::from("Delhi-Mumbai"),
        },
        Train {
            name: String::from("Shatabdi Express"),
            number: 12001,
            speed_kmh: 150.0,
            route: String::from("Delhi-Bhopal"),
        },
        Train {
            name: String::from("Passenger Local"),
            number: 51001,
            speed_kmh: 60.0,
            route: String::from("Local"),
        },
        Train {
            name: String::from("Duronto Express"),
            number: 12213,
            speed_kmh: 140.0,
            route: String::from("Delhi-Kolkata"),
        },
        Train {
            name: String::from("Garib Rath"),
            number: 12203,
            speed_kmh: 110.0,
            route: String::from("Delhi-Chennai"),
        },
        Train {
            name: String::from("MEMU Local"),
            number: 67001,
            speed_kmh: 55.0,
            route: String::from("Local"),
        },
    ];

    // Filter fast trains (>100 km/h), extract names, join into a string
    let fast_train_names: String = fleet
        .iter()
        .filter(|t| t.speed_kmh > 100.0)
        .map(|t| t.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    println!("🚄 Fast trains (>100 km/h): {}", fast_train_names);

    // Get the average speed of express trains
    let express_speeds: Vec<f64> = fleet
        .iter()
        .filter(|t| t.name.contains("Express"))
        .map(|t| t.speed_kmh)
        .collect();

    let avg_speed: f64 = express_speeds.iter().sum::<f64>() / express_speeds.len() as f64;
    println!(
        "📊 Average express train speed: {:.1} km/h (from {} trains)",
        avg_speed,
        express_speeds.len()
    );

    // Enumerate and display with rank
    println!("\n🏆 Speed ranking:");
    let mut sorted_fleet = fleet.clone();
    sorted_fleet.sort_by(|a, b| b.speed_kmh.partial_cmp(&a.speed_kmh).unwrap());
    for (rank, train) in sorted_fleet.iter().enumerate() {
        println!(
            "  #{}. {} — {:.0} km/h ({})",
            rank + 1,
            train.name,
            train.speed_kmh,
            train.route
        );
    }

    // =========================================================================
    // 10. 🧩 Common Patterns
    // =========================================================================
    println!("\n=== 10. 🧩 Common Patterns ===\n");

    // --- Pattern A: Grouping trains by route ---
    let mut trains_by_route: HashMap<&str, Vec<&Train>> = HashMap::new();
    for train in &fleet {
        trains_by_route
            .entry(train.route.as_str())
            .or_insert_with(Vec::new)
            .push(train);
    }

    println!("📂 Trains grouped by route:");
    for (route, trains) in &trains_by_route {
        let names: Vec<&str> = trains.iter().map(|t| t.name.as_str()).collect();
        println!("  {} → {:?}", route, names);
    }

    // --- Pattern B: Finding the fastest train ---
    let fastest = fleet
        .iter()
        .max_by(|a, b| a.speed_kmh.partial_cmp(&b.speed_kmh).unwrap());

    if let Some(train) = fastest {
        println!(
            "\n🏅 Fastest train: {} at {:.0} km/h",
            train.name, train.speed_kmh
        );
    }

    // --- Pattern C: Transforming data — build a summary report ---
    println!("\n📋 Fleet summary report:");
    let report: Vec<String> = fleet
        .iter()
        .map(|t| {
            let category = if t.speed_kmh > 120.0 {
                "Superfast"
            } else if t.speed_kmh > 80.0 {
                "Express"
            } else {
                "Local"
            };
            format!(
                "  #{:5} | {:<20} | {:>6.0} km/h | {}",
                t.number, t.name, t.speed_kmh, category
            )
        })
        .collect();

    for line in &report {
        println!("{}", line);
    }

    // --- Pattern D: Partitioning — split into two groups ---
    let (express, local): (Vec<&Train>, Vec<&Train>) =
        fleet.iter().partition(|t| t.speed_kmh > 80.0);

    println!(
        "\n🔀 Partition: {} express trains, {} local trains",
        express.len(),
        local.len()
    );
    println!(
        "   Express: {:?}",
        express.iter().map(|t| &t.name).collect::<Vec<_>>()
    );
    println!(
        "   Local:   {:?}",
        local.iter().map(|t| &t.name).collect::<Vec<_>>()
    );

    println!("\n✅ All collection examples complete!");
}
