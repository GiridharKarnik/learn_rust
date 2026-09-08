// =============================================
// Lesson 08 — HashMap Examples (Railway Edition)
// =============================================
//
// Run with: cargo run
//
// These examples demonstrate every HashMap concept from lesson.md.

use std::collections::HashMap;

fn main() {
    // =========================================================================
    // 1. 🚂 Creating and Inserting
    // =========================================================================
    println!("=== 1. 🚂 Creating and Inserting ===\n");

    // Create an empty HashMap — type annotation needed if Rust can't infer
    let mut platforms: HashMap<String, u32> = HashMap::new();

    // .insert(key, value) — add entries
    platforms.insert("MAS".to_string(), 12); // Chennai Central
    platforms.insert("SBC".to_string(), 10); // Bangalore City
    platforms.insert("NDLS".to_string(), 16); // New Delhi
    platforms.insert("BCT".to_string(), 9); // Mumbai Central
    platforms.insert("HWH".to_string(), 23); // Howrah

    println!("Station platforms: {:?}", platforms);
    println!("Total stations: {}\n", platforms.len());

    // .insert() returns Option<V> — the OLD value if key existed
    let old = platforms.insert("MAS".to_string(), 14);
    println!("Updated MAS platforms. Old value: {:?}", old); // Some(12)

    let old = platforms.insert("KGP".to_string(), 8);
    println!("Inserted KGP. Old value: {:?}", old); // None (new key)

    // Creating from a Vec of tuples using .collect()
    let station_names: HashMap<String, String> = vec![
        ("MAS".to_string(), "Chennai Central".to_string()),
        ("SBC".to_string(), "Bangalore City".to_string()),
        ("NDLS".to_string(), "New Delhi".to_string()),
    ]
    .into_iter()
    .collect();

    println!("\nFrom tuples: {:?}", station_names);

    // =========================================================================
    // 2. 🔍 Accessing Values
    // =========================================================================
    println!("\n=== 2. 🔍 Accessing Values ===\n");

    // map[&key] — PANICS if key missing! Use with caution.
    println!("NDLS platforms (indexing): {}", platforms["NDLS"]);

    // .get(&key) — returns Option<&V>, the SAFE way
    match platforms.get("MAS") {
        Some(p) => println!("MAS has {} platforms", p),
        None => println!("MAS not found"),
    }

    match platforms.get("XYZ") {
        Some(p) => println!("XYZ has {} platforms", p),
        None => println!("XYZ not found — .get() returned None, no panic!"),
    }

    // .contains_key()
    let code = "HWH";
    if platforms.contains_key(code) {
        println!("{} is in the directory ✓", code);
    }

    // .keys() and .values()
    print!("All station codes: ");
    let mut codes: Vec<&String> = platforms.keys().collect();
    codes.sort(); // Sort for consistent output
    for code in &codes {
        print!("{} ", code);
    }
    println!();

    let total: u32 = platforms.values().sum();
    println!("Total platforms across all stations: {}", total);

    // .is_empty()
    let empty: HashMap<String, u32> = HashMap::new();
    println!("Empty map is_empty? {}", empty.is_empty());
    println!("platforms is_empty? {}", platforms.is_empty());

    // =========================================================================
    // 3. 🔄 Iterating
    // =========================================================================
    println!("\n=== 3. 🔄 Iterating ===\n");

    let mut routes: HashMap<&str, &str> = HashMap::new();
    routes.insert("12001", "Chennai → New Delhi");
    routes.insert("12007", "Chennai → Bangalore");
    routes.insert("16525", "Bangalore → Kanyakumari");

    // Borrow iteration — for (key, value) in &map
    println!("All routes:");
    let mut sorted_routes: Vec<(&&str, &&str)> = routes.iter().collect();
    sorted_routes.sort_by_key(|(k, _)| **k);
    for (train_no, route) in &sorted_routes {
        println!("  Train {} → {}", train_no, route);
    }

    // Mutable iteration — modify values in place
    let mut delays: HashMap<String, i32> = HashMap::new();
    delays.insert("12001".to_string(), 15);
    delays.insert("12007".to_string(), 0);
    delays.insert("16525".to_string(), 45);

    println!("\nDelays before maintenance:");
    let mut sorted: Vec<_> = delays.iter().collect();
    sorted.sort_by_key(|(k, _)| k.to_string());
    for (train, delay) in &sorted {
        println!("  {}: {} min", train, delay);
    }

    // Add 5 minutes to every delay
    for (_train, delay) in &mut delays {
        *delay += 5;
    }

    println!("Delays after +5 min maintenance:");
    let mut sorted: Vec<_> = delays.iter().collect();
    sorted.sort_by_key(|(k, _)| k.to_string());
    for (train, delay) in &sorted {
        println!("  {}: {} min", train, delay);
    }

    // ⚠️ Order is NOT guaranteed — use sorted keys for deterministic output
    println!("\nStations sorted by code:");
    let mut codes: Vec<&String> = platforms.keys().collect();
    codes.sort();
    for code in &codes {
        println!("  {}: {} platforms", code, platforms[code.as_str()]);
    }

    // =========================================================================
    // 4. 🗑️ Removing
    // =========================================================================
    println!("\n=== 4. 🗑️ Removing ===\n");

    let mut directory: HashMap<String, String> = HashMap::new();
    directory.insert("MAS".to_string(), "Chennai Central".to_string());
    directory.insert("SBC".to_string(), "Bangalore City".to_string());
    directory.insert("OLD".to_string(), "Decommissioned Station".to_string());
    directory.insert("TST".to_string(), "Test".to_string());

    println!("Before removal: {} entries", directory.len());

    // .remove() returns Option<V>
    if let Some(name) = directory.remove("OLD") {
        println!("Removed: {} (was \"{}\")", "OLD", name);
    }

    // .retain() — keep only entries matching a condition
    directory.retain(|_code, name| name.len() > 4);
    println!("After retain (name > 4 chars): {:?}", directory);

    // =========================================================================
    // 5. 🔑 The Entry API — The Killer Feature
    // =========================================================================
    println!("\n=== 5. 🔑 The Entry API ===\n");

    // --- Counting pattern ---
    println!("--- Counting Pattern ---");

    let train_origins = vec![
        "Chennai",
        "Chennai",
        "Bangalore",
        "Chennai",
        "New Delhi",
        "Bangalore",
        "Chennai",
        "Mumbai",
    ];

    let mut departure_counts: HashMap<&str, u32> = HashMap::new();
    for city in &train_origins {
        *departure_counts.entry(city).or_insert(0) += 1;
    }

    let mut sorted_counts: Vec<_> = departure_counts.iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(a.1));
    for (city, count) in &sorted_counts {
        println!("  {}: {} departures", city, count);
    }

    // --- Grouping pattern ---
    println!("\n--- Grouping Pattern ---");

    let trains = vec![
        ("Express", "Rajdhani Express"),
        ("Express", "Shatabdi Express"),
        ("Mail", "Thiruvananthapuram Mail"),
        ("Express", "Duronto Express"),
        ("Mail", "Howrah Mail"),
    ];

    let mut by_type: HashMap<&str, Vec<&str>> = HashMap::new();
    for (train_type, name) in &trains {
        by_type
            .entry(train_type)
            .or_insert_with(Vec::new)
            .push(name);
    }

    let mut sorted_types: Vec<_> = by_type.iter().collect();
    sorted_types.sort_by_key(|(k, _)| *k);
    for (train_type, names) in &sorted_types {
        println!("  {}: {:?}", train_type, names);
    }

    // --- .or_default() — shortest syntax ---
    println!("\n--- or_default() ---");

    let mut counts: HashMap<&str, u32> = HashMap::new();
    let cities = vec!["Chennai", "Mumbai", "Chennai", "Delhi"];
    for city in &cities {
        *counts.entry(city).or_default() += 1;
    }
    println!("  Using or_default(): {:?}", counts);

    // =========================================================================
    // 6. 🏗️ Ownership
    // =========================================================================
    println!("\n=== 6. 🏗️ Ownership ===\n");

    // HashMap takes ownership of String keys and values
    let code = String::from("MAS");
    let name = String::from("Chennai Central");

    let mut owned_map: HashMap<String, String> = HashMap::new();
    owned_map.insert(code.clone(), name.clone()); // clone to keep originals
    println!("After insert with clone — code still usable: {}", code);
    println!("After insert with clone — name still usable: {}", name);

    // Without clone, the values would be moved:
    let code2 = String::from("SBC");
    let name2 = String::from("Bangalore City");
    owned_map.insert(code2, name2);
    // println!("{}", code2);  // ❌ Would fail — code2 was moved!

    // Copy types (like u32) are copied, not moved
    let platform_num: u32 = 5;
    let mut num_map: HashMap<String, u32> = HashMap::new();
    num_map.insert("MAS".to_string(), platform_num);
    println!("platform_num still usable: {}", platform_num); // ✅ u32 is Copy

    // =========================================================================
    // 7. 📋 Common Patterns
    // =========================================================================
    println!("\n=== 7. 📋 Common Patterns ===\n");

    // Pattern 1: Word counting
    println!("--- Word Counting ---");
    let announcement = "the next train is the Rajdhani Express the fastest train";
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    for word in announcement.split_whitespace() {
        *word_counts.entry(word).or_insert(0) += 1;
    }
    let mut sorted_words: Vec<_> = word_counts.iter().collect();
    sorted_words.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
    for (word, count) in &sorted_words {
        println!("  \"{}\": {}", word, count);
    }

    // Pattern 2: Grouping
    println!("\n--- Grouping Stations by State ---");
    let stations = vec![
        ("Chennai Central", "Tamil Nadu"),
        ("Bangalore City", "Karnataka"),
        ("Coimbatore", "Tamil Nadu"),
        ("Mysuru", "Karnataka"),
        ("Madurai", "Tamil Nadu"),
    ];

    let mut by_state: HashMap<&str, Vec<&str>> = HashMap::new();
    for (station, state) in &stations {
        by_state.entry(state).or_insert_with(Vec::new).push(station);
    }

    let mut sorted_states: Vec<_> = by_state.iter().collect();
    sorted_states.sort_by_key(|(state, _)| *state);
    for (state, stations) in &sorted_states {
        println!("  {}: {:?}", state, stations);
    }

    // Pattern 3: Lookup table from tuples
    println!("\n--- Lookup Table ---");
    let station_list = vec![
        ("MAS", "Chennai Central"),
        ("SBC", "Bangalore City"),
        ("NDLS", "New Delhi"),
        ("BCT", "Mumbai Central"),
        ("HWH", "Howrah"),
    ];

    let lookup: HashMap<&str, &str> = station_list.into_iter().collect();
    for code in &["MAS", "SBC", "XYZ"] {
        match lookup.get(code) {
            Some(name) => println!("  {} → {}", code, name),
            None => println!("  {} → Not found!", code),
        }
    }

    // Pattern 4: Inverting a map
    println!("\n--- Inverted Lookup (name → code) ---");
    let name_to_code: HashMap<&str, &str> = lookup.iter().map(|(k, v)| (*v, *k)).collect();

    for name in &["Chennai Central", "Howrah", "Unknown Station"] {
        match name_to_code.get(name) {
            Some(code) => println!("  {} → {}", name, code),
            None => println!("  {} → No code found!", name),
        }
    }

    // =========================================================================
    // 8. 🆚 Quick Comparison Recap
    // =========================================================================
    println!("\n=== 8. 🆚 Vec vs HashMap ===\n");

    // Vec: ordered, access by index
    let platforms_vec = vec!["Platform 1", "Platform 2", "Platform 3"];
    println!("Vec — access by index: {}", platforms_vec[0]);

    // HashMap: unordered, access by key
    let mut platform_map: HashMap<&str, &str> = HashMap::new();
    platform_map.insert("12001", "Platform 3");
    platform_map.insert("12007", "Platform 1");
    println!(
        "HashMap — access by key: Train 12001 departs from {}",
        platform_map["12001"]
    );

    println!("\n🚂 All examples complete! Run the exercise next: cd ../exercise && cargo run");
}
