use std::collections::HashMap;

// A simple train struct we'll use throughout
#[derive(Debug, Clone)]
struct Train {
    name: String,
    number: u32,
    speed_kmh: f64,
    fare: f64,
    from: String,
    to: String,
    coaches: u32,
}

impl Train {
    fn new(
        name: &str,
        number: u32,
        speed_kmh: f64,
        fare: f64,
        from: &str,
        to: &str,
        coaches: u32,
    ) -> Self {
        Train {
            name: name.to_string(),
            number,
            speed_kmh,
            fare,
            from: from.to_string(),
            to: to.to_string(),
            coaches,
        }
    }
}

fn build_trains() -> Vec<Train> {
    vec![
        Train::new(
            "Rajdhani Express",
            12001,
            130.0,
            4250.00,
            "Chennai",
            "New Delhi",
            20,
        ),
        Train::new(
            "Shatabdi Express",
            12007,
            150.0,
            755.50,
            "Chennai",
            "Bangalore",
            12,
        ),
        Train::new(
            "Duronto Express",
            12245,
            120.0,
            2100.00,
            "Chennai",
            "Mumbai",
            18,
        ),
        Train::new(
            "Kovai Express",
            12675,
            80.0,
            350.00,
            "Chennai",
            "Coimbatore",
            16,
        ),
        Train::new(
            "Island Express",
            16525,
            75.0,
            625.00,
            "Bangalore",
            "Kanyakumari",
            14,
        ),
        Train::new(
            "Thiruvananthapuram Mail",
            12625,
            90.0,
            780.00,
            "Chennai",
            "Thiruvananthapuram",
            18,
        ),
        Train::new(
            "Tejas Express",
            22501,
            160.0,
            1200.00,
            "Chennai",
            "Madurai",
            10,
        ),
        Train::new(
            "Vande Bharat Express",
            20601,
            180.0,
            1850.00,
            "Chennai",
            "Bangalore",
            16,
        ),
    ]
}

fn main() {
    let trains = build_trains();

    // =========================================================================
    // 1. 🚂 The Iterator Trait — .iter(), .iter_mut(), .into_iter()
    // =========================================================================
    println!("=== 1. 🚂 The Iterator Trait ===\n");

    // .iter() — borrows each element as &T
    println!("All trains (borrowed with .iter()):");
    for train in trains.iter() {
        println!("  #{} {}", train.number, train.name);
    }
    // trains is still usable — we only borrowed
    println!("  (trains Vec still has {} entries)\n", trains.len());

    // .iter_mut() — mutably borrows each element as &mut T
    let mut speeds = vec![100, 120, 95, 80, 150];
    println!("Original speeds: {:?}", speeds);
    for speed in speeds.iter_mut() {
        *speed += 10; // boost each speed by 10
    }
    println!("After +10 boost: {:?}", speeds);

    // .into_iter() — takes ownership (consumes the collection)
    let temp_names = vec!["Rajdhani", "Shatabdi", "Duronto"];
    let owned: Vec<String> = temp_names.into_iter().map(|s| s.to_string()).collect();
    println!("Owned strings: {:?}", owned);
    // temp_names is gone here — consumed by into_iter()

    // Manually calling .next()
    println!("\nManually calling .next():");
    let platforms = vec![1, 3, 5, 7];
    let mut iter = platforms.iter();
    println!("  next() = {:?}", iter.next()); // Some(&1)
    println!("  next() = {:?}", iter.next()); // Some(&3)
    println!("  next() = {:?}", iter.next()); // Some(&5)
    println!("  next() = {:?}", iter.next()); // Some(&7)
    println!("  next() = {:?}", iter.next()); // None — exhausted

    // =========================================================================
    // 2. 🔧 Iterator Adaptors (Lazy)
    // =========================================================================
    println!("\n=== 2. 🔧 Iterator Adaptors (Lazy) ===\n");

    // .map() — transform each element
    let train_names: Vec<&str> = trains.iter().map(|t| t.name.as_str()).collect();
    println!(".map() — train names: {:?}", train_names);

    // .filter() — keep elements matching a condition
    let fast_trains: Vec<&Train> = trains.iter().filter(|t| t.speed_kmh > 100.0).collect();
    println!("\n.filter() — trains faster than 100 km/h:");
    for t in &fast_trains {
        println!("  {} ({} km/h)", t.name, t.speed_kmh);
    }

    // .enumerate() — attach an index
    println!("\n.enumerate() — numbered departure board:");
    for (i, t) in trains.iter().enumerate() {
        println!("  {}. #{} {}", i + 1, t.number, t.name);
    }

    // .zip() — pair two iterators
    let platforms = vec![1, 3, 5, 7, 2, 4, 6, 8];
    println!("\n.zip() — assign trains to platforms:");
    for (train, platform) in trains.iter().zip(platforms.iter()) {
        println!("  Platform {}: {}", platform, train.name);
    }

    // .take(n) / .skip(n)
    println!("\n.take(3) — first 3 trains:");
    for t in trains.iter().take(3) {
        println!("  {}", t.name);
    }
    println!(".skip(5) — skip first 5:");
    for t in trains.iter().skip(5) {
        println!("  {}", t.name);
    }

    // .chain() — concatenate two iterators
    let express = vec!["Rajdhani", "Shatabdi"];
    let local = vec!["EMU Local", "MEMU"];
    let all: Vec<&&str> = express.iter().chain(local.iter()).collect();
    println!("\n.chain() — express + local: {:?}", all);

    // .flatten() — flatten nested iterators
    let routes: Vec<Vec<&str>> = vec![
        vec!["Chennai", "Katpadi", "Bangalore"],
        vec!["Mumbai", "Pune", "Solapur"],
    ];
    let all_stops: Vec<&&str> = routes.iter().flatten().collect();
    println!("\n.flatten() — all stops: {:?}", all_stops);

    // .inspect() — peek for debugging
    println!("\n.inspect() — debugging a pipeline:");
    let result: Vec<f64> = trains
        .iter()
        .inspect(|t| println!("  [inspect] considering: {}", t.name))
        .filter(|t| t.speed_kmh >= 130.0)
        .inspect(|t| println!("  [inspect] passed filter: {}", t.name))
        .map(|t| t.speed_kmh)
        .collect();
    println!("  Fast speeds: {:?}", result);

    // KEY: Laziness demo
    println!("\n🔴 KEY INSIGHT — Laziness:");
    println!("  trains.iter().map(|t| t.speed_kmh * 2.0)  ← does NOTHING by itself!");
    let _lazy = trains.iter().map(|t| t.speed_kmh * 2.0); // No .collect() — nothing happens
    println!("  You need a consumer like .collect() to execute the chain.");

    // =========================================================================
    // 3. 📊 Iterator Consumers
    // =========================================================================
    println!("\n=== 3. 📊 Iterator Consumers ===\n");

    // .collect() — already shown above

    // .count()
    let fast_count = trains.iter().filter(|t| t.speed_kmh > 100.0).count();
    println!(".count() — trains over 100 km/h: {}", fast_count);

    // .sum()
    let total_coaches: u32 = trains.iter().map(|t| t.coaches).sum();
    println!(".sum() — total coaches: {}", total_coaches);

    // .product()
    let small_nums = vec![2u32, 3, 4];
    let product: u32 = small_nums.iter().product();
    println!(".product() — 2 × 3 × 4 = {}", product);

    // .min() / .max()
    println!("\n.min()/.max() on integers:");
    let coach_counts: Vec<u32> = trains.iter().map(|t| t.coaches).collect();
    println!("  Coach counts: {:?}", coach_counts);
    println!("  Min coaches: {:?}", coach_counts.iter().min());
    println!("  Max coaches: {:?}", coach_counts.iter().max());

    // .min_by_key() / .max_by_key()
    let slowest = trains.iter().min_by_key(|t| t.speed_kmh as u32).unwrap();
    println!(
        "\n.min_by_key() — slowest train: {} ({} km/h)",
        slowest.name, slowest.speed_kmh
    );
    let fastest = trains.iter().max_by_key(|t| t.speed_kmh as u32).unwrap();
    println!(
        ".max_by_key() — fastest train: {} ({} km/h)",
        fastest.name, fastest.speed_kmh
    );

    // .min_by() / .max_by() — needed for f64 (NaN doesn't have total ordering)
    let cheapest = trains
        .iter()
        .min_by(|a, b| a.fare.partial_cmp(&b.fare).unwrap())
        .unwrap();
    println!(
        "\n.min_by() — cheapest: {} (₹{:.2})",
        cheapest.name, cheapest.fare
    );
    let priciest = trains
        .iter()
        .max_by(|a, b| a.fare.partial_cmp(&b.fare).unwrap())
        .unwrap();
    println!(
        ".max_by() — priciest: {} (₹{:.2})",
        priciest.name, priciest.fare
    );

    // .find()
    let found = trains.iter().find(|t| t.name.contains("Vande"));
    println!(
        "\n.find() — searching for 'Vande': {:?}",
        found.map(|t| &t.name)
    );

    let not_found = trains.iter().find(|t| t.name.contains("Bullet"));
    println!(
        ".find() — searching for 'Bullet': {:?}",
        not_found.map(|t| &t.name)
    );

    // .any() / .all()
    let any_fast = trains.iter().any(|t| t.speed_kmh > 170.0);
    println!("\n.any() — any train over 170 km/h? {}", any_fast);
    let all_have_coaches = trains.iter().all(|t| t.coaches > 0);
    println!(".all() — all trains have coaches? {}", all_have_coaches);

    // .fold() — the most powerful consumer
    let total_fare = trains.iter().fold(0.0, |acc, t| acc + t.fare);
    println!(
        "\n.fold() — total fare across all trains: ₹{:.2}",
        total_fare
    );

    // fold to build a summary string
    let summary = trains.iter().fold(String::new(), |mut acc, t| {
        if !acc.is_empty() {
            acc.push_str(", ");
        }
        acc.push_str(&t.name);
        acc
    });
    println!(".fold() — all names: {}", summary);

    // .for_each() — like a for loop at the end of a chain
    println!("\n.for_each() — announcing fast trains:");
    trains
        .iter()
        .filter(|t| t.speed_kmh >= 150.0)
        .for_each(|t| println!("  🚄 {} now departing!", t.name));

    // =========================================================================
    // 4. 🎯 Collecting into Different Types
    // =========================================================================
    println!("\n=== 4. 🎯 Collecting into Different Types ===\n");

    // Into Vec (with turbofish)
    let names = trains.iter().map(|t| t.name.clone()).collect::<Vec<_>>();
    println!("Collected into Vec<String>: {:?}\n", &names[..3]);

    // Into HashMap (from tuples)
    let fare_map: HashMap<&str, f64> = trains.iter().map(|t| (t.name.as_str(), t.fare)).collect();
    println!("Collected into HashMap:");
    for (name, fare) in &fare_map {
        println!("  {} → ₹{:.2}", name, fare);
    }

    // Into String (from chars)
    let code: String = vec!['I', 'R', 'C', 'T', 'C'].into_iter().collect();
    println!("\nCollected chars into String: {}", code);

    // Into String (from &str slices)
    let parts = vec!["Train", " #12001", " departing"];
    let announcement: String = parts.into_iter().collect();
    println!("Collected &str slices into String: {}", announcement);

    // Type annotation instead of turbofish
    let doubled_speeds: Vec<f64> = trains.iter().map(|t| t.speed_kmh * 2.0).collect(); // No turbofish — type annotation does the job
    println!(
        "\nType annotation instead of turbofish: {:?}",
        &doubled_speeds[..3]
    );

    // =========================================================================
    // 5. 🔗 Chaining — Where It All Comes Together
    // =========================================================================
    println!("\n=== 5. 🔗 Chaining Examples ===\n");

    // Example 1: Filter → map → collect → join
    let fast_train_list: String = trains
        .iter()
        .filter(|t| t.speed_kmh > 100.0)
        .map(|t| format!("{} ({} km/h)", t.name, t.speed_kmh))
        .collect::<Vec<_>>()
        .join(", ");
    println!("Fast trains: {}", fast_train_list);

    // Example 2: Enumerate → filter → map → collect
    let board: String = trains
        .iter()
        .enumerate()
        .filter(|(_, t)| t.fare > 1000.0)
        .map(|(i, t)| format!("  {}. #{} {} — ₹{:.2}", i + 1, t.number, t.name, t.fare))
        .collect::<Vec<_>>()
        .join("\n");
    println!("\nPremium departure board:\n{}", board);

    // Example 3: Group by destination using fold
    let trains_by_dest: HashMap<&str, Vec<&str>> =
        trains.iter().fold(HashMap::new(), |mut acc, t| {
            acc.entry(t.to.as_str())
                .or_insert_with(Vec::new)
                .push(t.name.as_str());
            acc
        });
    println!("\nTrains grouped by destination:");
    let mut destinations: Vec<_> = trains_by_dest.iter().collect();
    destinations.sort_by_key(|(dest, _)| dest.to_string());
    for (dest, names) in &destinations {
        println!("  {} ← {}", dest, names.join(", "));
    }

    // Example 4: Complex pipeline — top 3 most expensive, formatted
    let mut sorted_trains: Vec<&Train> = trains.iter().collect();
    sorted_trains.sort_by(|a, b| b.fare.partial_cmp(&a.fare).unwrap());
    let top3: String = sorted_trains
        .iter()
        .take(3)
        .enumerate()
        .map(|(i, t)| format!("  {}. {} (₹{:.2})", i + 1, t.name, t.fare))
        .collect::<Vec<_>>()
        .join("\n");
    println!("\nTop 3 most expensive trains:\n{}", top3);

    // Example 5: Zip to pair names with formatted info
    let origins: Vec<&str> = trains.iter().map(|t| t.from.as_str()).collect();
    let dests: Vec<&str> = trains.iter().map(|t| t.to.as_str()).collect();
    let route_list: Vec<String> = origins
        .iter()
        .zip(dests.iter())
        .zip(trains.iter().map(|t| t.name.as_str()))
        .map(|((from, to), name)| format!("{}: {} → {}", name, from, to))
        .collect();
    println!("\nAll routes (via zip):");
    for r in route_list.iter().take(4) {
        println!("  {}", r);
    }

    // =========================================================================
    // 6. ⚡ for Loop vs Iterator Chain
    // =========================================================================
    println!("\n=== 6. ⚡ for Loop vs Iterator Chain ===\n");

    // for loop version
    let mut fast_names_loop = Vec::new();
    for train in &trains {
        if train.speed_kmh > 100.0 {
            fast_names_loop.push(&train.name);
        }
    }
    println!("for loop result: {:?}", fast_names_loop);

    // Iterator chain version — same result, same performance
    let fast_names_iter: Vec<&String> = trains
        .iter()
        .filter(|t| t.speed_kmh > 100.0)
        .map(|t| &t.name)
        .collect();
    println!("Iterator result: {:?}", fast_names_iter);

    println!("\nBoth produce identical results and identical machine code!");
    println!("Iterators are a zero-cost abstraction in Rust. 🎉");

    // =========================================================================
    // 7. 🌐 TypeScript Comparison
    // =========================================================================
    println!("\n=== 7. 🌐 TypeScript Comparison ===\n");

    println!("JS:   arr.map(fn)            →  Rust: arr.iter().map(fn).collect()");
    println!("JS:   arr.filter(fn)         →  Rust: arr.iter().filter(fn).collect()");
    println!("JS:   arr.reduce(fn, init)   →  Rust: arr.iter().fold(init, fn)");
    println!("JS:   arr.find(fn)           →  Rust: arr.iter().find(fn)  // returns Option");
    println!("JS:   arr.some(fn)           →  Rust: arr.iter().any(fn)");
    println!("JS:   arr.every(fn)          →  Rust: arr.iter().all(fn)");
    println!();
    println!("Key difference: JS arrays are eager, Rust iterators are lazy.");
    println!("JS:   [1,2,3].map(x => x*2)  // immediately returns [2,4,6]");
    println!("Rust: vec.iter().map(|x| x*2) // returns a lazy Map adapter — nothing computed!");
    println!("      Must call .collect() to materialize the result.");
}
