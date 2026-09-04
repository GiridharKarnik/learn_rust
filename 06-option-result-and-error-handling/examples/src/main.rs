// =============================================================================
// Lesson 06: Option, Result & Error Handling 🚂
// =============================================================================
// This file demonstrates all major Option/Result concepts in Rust.
// Run with: cargo run
// =============================================================================

// ---------------------------------------------------------------------------
// Common types used throughout the examples
// ---------------------------------------------------------------------------

/// A train on the railway network.
#[derive(Debug, Clone)]
struct Train {
    name: String,
    number: u32,
    platform: Option<u32>, // platform might not be assigned yet
}

impl Train {
    fn new(name: &str, number: u32, platform: Option<u32>) -> Self {
        Train {
            name: name.to_string(),
            number,
            platform,
        }
    }
}

fn main() {
    section_1_creating_and_matching_option();
    section_2_option_methods();
    section_3_if_let_with_option();
    section_4_creating_and_matching_result();
    section_5_result_methods();
    section_6_question_mark_operator();
    section_7_while_let();
    section_8_converting_option_and_result();
    section_9_chaining_with_and_then();
}

// =============================================================================
// Helper: build a sample list of trains
// =============================================================================

fn sample_trains() -> Vec<Train> {
    vec![
        Train::new("Rajdhani Express", 12049, Some(3)),
        Train::new("Shatabdi Express", 12007, Some(1)),
        Train::new("Duronto Express", 12213, None), // no platform yet
        Train::new("Garib Rath", 12578, Some(5)),
        Train::new("Chennai Express", 12163, None),
    ]
}

/// Find a train by its number. Returns Some(&Train) or None.
fn find_train(trains: &[Train], number: u32) -> Option<&Train> {
    for train in trains {
        if train.number == number {
            return Some(train);
        }
    }
    None
}

// =============================================================================
// 1. Creating and Matching Option
// =============================================================================

fn section_1_creating_and_matching_option() {
    println!("\n🚂 ═══════════════════════════════════════════════════════");
    println!("   1. CREATING AND MATCHING OPTION");
    println!("   ═══════════════════════════════════════════════════════\n");

    let trains = sample_trains();

    // --- match on Option ---
    println!("--- Searching for train #12049 ---");
    match find_train(&trains, 12049) {
        Some(train) => println!("Found: {} (Platform {:?})", train.name, train.platform),
        None => println!("No train found"),
    }

    println!("--- Searching for train #99999 ---");
    match find_train(&trains, 99999) {
        Some(train) => println!("Found: {}", train.name),
        None => println!("No train found with number 99999"),
    }

    // --- Creating Option values directly ---
    let some_number: Option<u32> = Some(42);
    let no_number: Option<u32> = None;
    println!("\nsome_number = {:?}", some_number);
    println!("no_number   = {:?}", no_number);
}

// =============================================================================
// 2. Option Methods
// =============================================================================

fn section_2_option_methods() {
    println!("\n🚂 ═══════════════════════════════════════════════════════");
    println!("   2. OPTION METHODS");
    println!("   ═══════════════════════════════════════════════════════\n");

    let trains = sample_trains();

    // --- unwrap_or ---
    println!("--- unwrap_or ---");
    let platform = find_train(&trains, 12049)
        .and_then(|t| t.platform)
        .unwrap_or(0);
    println!("Rajdhani platform (or 0): {platform}");

    let platform = find_train(&trains, 99999)
        .and_then(|t| t.platform)
        .unwrap_or(0);
    println!("Unknown train platform (or 0): {platform}");

    // --- map ---
    println!("\n--- map ---");
    let name: Option<String> = find_train(&trains, 12049).map(|train| train.name.clone());
    println!("Train name: {:?}", name); // Some("Rajdhani Express")

    let name: Option<String> = find_train(&trains, 99999).map(|train| train.name.clone());
    println!("Unknown train name: {:?}", name); // None

    // --- is_some / is_none ---
    println!("\n--- is_some / is_none ---");
    let found = find_train(&trains, 12049);
    println!("12049 exists? {}", found.is_some()); // true
    println!("12049 missing? {}", found.is_none()); // false

    let not_found = find_train(&trains, 99999);
    println!("99999 exists? {}", not_found.is_some()); // false
    println!("99999 missing? {}", not_found.is_none()); // true

    // --- unwrap_or_else ---
    println!("\n--- unwrap_or_else ---");
    let name = find_train(&trains, 99999)
        .map(|t| t.name.clone())
        .unwrap_or_else(|| String::from("Unknown Train"));
    println!("Train name: {name}"); // "Unknown Train"

    // --- or ---
    println!("\n--- or (fallback to another Option) ---");
    let primary = find_train(&trains, 99999); // None
    let backup = find_train(&trains, 12049); // Some(Rajdhani)
    let chosen = primary.or(backup);
    println!(
        "Chosen train: {}",
        chosen.map(|t| t.name.as_str()).unwrap_or("None")
    );
}

// =============================================================================
// 3. if let with Option
// =============================================================================

fn section_3_if_let_with_option() {
    println!("\n🚂 ═══════════════════════════════════════════════════════");
    println!("   3. IF LET WITH OPTION");
    println!("   ═══════════════════════════════════════════════════════\n");

    let trains = sample_trains();

    // --- if let Some ---
    if let Some(train) = find_train(&trains, 12049) {
        println!(
            "Found Rajdhani: number {}, platform {:?}",
            train.number, train.platform
        );
    }

    // --- if let with else ---
    if let Some(train) = find_train(&trains, 99999) {
        println!("Found: {}", train.name);
    } else {
        println!("Train #99999 not found — using if let with else");
    }

    // --- Nested if let for optional platform ---
    if let Some(train) = find_train(&trains, 12213) {
        println!("\nFound {} (number {})", train.name, train.number);
        if let Some(platform) = train.platform {
            println!("  Platform: {platform}");
        } else {
            println!("  Platform: not yet assigned");
        }
    }
}

// =============================================================================
// 4. Creating and Matching Result
// =============================================================================

fn section_4_creating_and_matching_result() {
    println!("\n🚂 ═══════════════════════════════════════════════════════");
    println!("   4. CREATING AND MATCHING RESULT");
    println!("   ═══════════════════════════════════════════════════════\n");

    // --- Parsing strings to numbers ---
    println!("--- Parsing strings ---");
    let inputs = ["12049", "hello", "42", "", "99x"];

    for input in inputs {
        let result: Result<u32, _> = input.parse();
        match result {
            Ok(n) => println!("  \"{input}\" → Ok({n})"),
            Err(e) => println!("  \"{input}\" → Err({e})"),
        }
    }

    // --- Custom validation ---
    println!("\n--- Validate passenger names ---");
    let names = ["Giridhar", "", "X", "Priya"];

    for name in names {
        match validate_passenger(name) {
            Ok(valid) => println!("  \"{name}\" → Ok(\"{valid}\")"),
            Err(e) => println!("  \"{name}\" → Err(\"{e}\")"),
        }
    }
}

fn validate_passenger(name: &str) -> Result<String, String> {
    if name.is_empty() {
        Err(String::from("Name cannot be empty"))
    } else if name.len() < 2 {
        Err(format!("Name '{}' is too short (min 2 characters)", name))
    } else {
        Ok(name.to_string())
    }
}

// =============================================================================
// 5. Result Methods
// =============================================================================

fn section_5_result_methods() {
    println!("\n🚂 ═══════════════════════════════════════════════════════");
    println!("   5. RESULT METHODS");
    println!("   ═══════════════════════════════════════════════════════\n");

    // --- unwrap_or ---
    println!("--- unwrap_or ---");
    let n: u32 = "12049".parse().unwrap_or(0);
    println!("Parsed \"12049\": {n}");

    let n: u32 = "hello".parse().unwrap_or(0);
    println!("Parsed \"hello\" (or 0): {n}");

    // --- map ---
    println!("\n--- map (transform Ok value) ---");
    let doubled: Result<u32, _> = "21".parse::<u32>().map(|n| n * 2);
    println!("\"21\" parsed and doubled: {:?}", doubled); // Ok(42)

    let doubled: Result<u32, _> = "abc".parse::<u32>().map(|n| n * 2);
    println!("\"abc\" parsed and doubled: {:?}", doubled); // Err(...)

    // --- map_err ---
    println!("\n--- map_err (transform Err value) ---");
    let result: Result<u32, String> = "abc"
        .parse::<u32>()
        .map_err(|e| format!("Failed to parse train number: {e}"));
    println!("Better error: {:?}", result);

    // --- is_ok / is_err ---
    println!("\n--- is_ok / is_err ---");
    let good: Result<u32, _> = "42".parse::<u32>();
    let bad: Result<u32, _> = "nope".parse::<u32>();
    println!("\"42\" is_ok: {}, is_err: {}", good.is_ok(), good.is_err());
    println!("\"nope\" is_ok: {}, is_err: {}", bad.is_ok(), bad.is_err());

    // --- if let Ok ---
    println!("\n--- if let Ok ---");
    if let Ok(number) = "12049".parse::<u32>() {
        println!("Successfully parsed train number: {number}");
    }
    if let Err(e) = "xyz".parse::<u32>() {
        println!("Failed to parse: {e}");
    }
}

// =============================================================================
// 6. The ? Operator
// =============================================================================

fn section_6_question_mark_operator() {
    println!("\n🚂 ═══════════════════════════════════════════════════════");
    println!("   6. THE ? OPERATOR");
    println!("   ═══════════════════════════════════════════════════════\n");

    let trains = sample_trains();

    // Try several scenarios
    let cases = [
        ("12049", "Giridhar"),
        ("hello", "Priya"), // bad number
        ("99999", "Arjun"), // train not found
        ("12213", "Meera"), // train found but no platform
    ];

    for (input, passenger) in cases {
        println!("Booking for {passenger} on train \"{input}\":");
        match book_with_question_mark(&trains, input, passenger) {
            Ok(msg) => println!("  ✅ {msg}"),
            Err(e) => println!("  ❌ {e}"),
        }
    }
}

/// A function that uses ? to propagate errors cleanly.
fn book_with_question_mark(
    trains: &[Train],
    input: &str,
    passenger: &str,
) -> Result<String, String> {
    // Step 1: parse the train number (Result → use map_err + ?)
    let number: u32 = input
        .parse()
        .map_err(|e| format!("Invalid train number \"{input}\": {e}"))?;

    // Step 2: find the train (Option → convert to Result with ok_or + ?)
    let train =
        find_train(trains, number).ok_or(format!("Train #{number} not found in schedule"))?;

    // Step 3: check platform (Option → convert to Result with ok_or + ?)
    let platform = train
        .platform
        .ok_or(format!("Train {} has no platform assigned yet", train.name))?;

    // If we get here, all three steps succeeded!
    Ok(format!(
        "{} booked on {} (#{}) — Platform {}",
        passenger, train.name, train.number, platform
    ))
}

// =============================================================================
// 7. while let with Vec::pop()
// =============================================================================

fn section_7_while_let() {
    println!("\n🚂 ═══════════════════════════════════════════════════════");
    println!("   7. WHILE LET");
    println!("   ═══════════════════════════════════════════════════════\n");

    // Simulate a queue of stations to announce
    let mut announcement_queue = vec![
        String::from("Chennai Central"),
        String::from("Katpadi Junction"),
        String::from("Bangalore City"),
        String::from("Mysore Junction"),
    ];

    println!("Processing announcement queue:");
    // pop() returns Option<T> — Some(item) or None when empty
    while let Some(station) = announcement_queue.pop() {
        println!("  📢 Now arriving at: {station}");
    }
    println!("Queue empty — all stations announced.\n");

    // Another example: processing optional values from a list
    let maybe_platforms: Vec<Option<u32>> = vec![Some(1), None, Some(3), None, Some(5)];
    println!("Assigned platforms:");
    for (i, maybe_platform) in maybe_platforms.iter().enumerate() {
        if let Some(platform) = maybe_platform {
            println!("  Train {}: Platform {platform}", i + 1);
        } else {
            println!("  Train {}: no platform yet", i + 1);
        }
    }
}

// =============================================================================
// 8. Converting Option ↔ Result
// =============================================================================

fn section_8_converting_option_and_result() {
    println!("\n🚂 ═══════════════════════════════════════════════════════");
    println!("   8. CONVERTING OPTION ↔ RESULT");
    println!("   ═══════════════════════════════════════════════════════\n");

    // --- ok_or: Option → Result ---
    println!("--- ok_or: Option → Result ---");
    let some_platform: Option<u32> = Some(3);
    let none_platform: Option<u32> = None;

    let result: Result<u32, &str> = some_platform.ok_or("No platform assigned");
    println!("Some(3).ok_or(...) = {:?}", result); // Ok(3)

    let result: Result<u32, &str> = none_platform.ok_or("No platform assigned");
    println!("None.ok_or(...)    = {:?}", result); // Err("No platform assigned")

    // --- ok: Result → Option ---
    println!("\n--- ok: Result → Option ---");
    let good: Result<u32, String> = Ok(42);
    let bad: Result<u32, String> = Err(String::from("something went wrong"));

    println!("Ok(42).ok()  = {:?}", good.ok()); // Some(42)
    println!("Err(...).ok() = {:?}", bad.ok()); // None  (error is discarded)

    // --- Practical use: ok_or with ? ---
    println!("\n--- Practical: ok_or + ? in a function ---");
    let trains = sample_trains();
    match get_train_name(&trains, 12049) {
        Ok(name) => println!("Train 12049: {name}"),
        Err(e) => println!("Error: {e}"),
    }
    match get_train_name(&trains, 99999) {
        Ok(name) => println!("Train 99999: {name}"),
        Err(e) => println!("Error: {e}"),
    }

    // --- transpose ---
    println!("\n--- transpose: Option<Result> ↔ Result<Option> ---");
    let option_result: Option<Result<u32, String>> = Some(Ok(42));
    let result_option: Result<Option<u32>, String> = option_result.transpose();
    println!("Some(Ok(42)).transpose() = {:?}", result_option); // Ok(Some(42))

    let option_result: Option<Result<u32, String>> = Some(Err(String::from("oops")));
    let result_option: Result<Option<u32>, String> = option_result.transpose();
    println!("Some(Err(\"oops\")).transpose() = {:?}", result_option); // Err("oops")

    let option_result: Option<Result<u32, String>> = None;
    let result_option: Result<Option<u32>, String> = option_result.transpose();
    println!("None.transpose() = {:?}", result_option); // Ok(None)
}

fn get_train_name(trains: &[Train], number: u32) -> Result<String, String> {
    let train = find_train(trains, number).ok_or(format!("Train #{number} not found"))?;
    Ok(train.name.clone())
}

// =============================================================================
// 9. Chaining Options with and_then
// =============================================================================

fn section_9_chaining_with_and_then() {
    println!("\n🚂 ═══════════════════════════════════════════════════════");
    println!("   9. CHAINING OPTIONS WITH AND_THEN");
    println!("   ═══════════════════════════════════════════════════════\n");

    let trains = sample_trains();

    // and_then chains operations that each return Option
    // Without and_then, you'd get Option<Option<u32>> — nested Options!

    println!("--- Find train then get platform (and_then) ---");

    // Train 12049 exists and has a platform
    let platform = find_train(&trains, 12049).and_then(|train| train.platform);
    println!("Train 12049 platform: {:?}", platform); // Some(3)

    // Train 12213 exists but has no platform
    let platform = find_train(&trains, 12213).and_then(|train| train.platform);
    println!("Train 12213 platform: {:?}", platform); // None

    // Train 99999 doesn't exist
    let platform = find_train(&trains, 99999).and_then(|train| train.platform);
    println!("Train 99999 platform: {:?}", platform); // None

    // --- Chaining multiple and_then ---
    println!("\n--- Multi-step chain ---");

    // Find train → get platform → check if it's an odd-numbered platform
    let is_odd_platform = find_train(&trains, 12049)
        .and_then(|train| train.platform)
        .map(|p| p % 2 != 0);
    println!("Train 12049 on odd platform? {:?}", is_odd_platform); // Some(true)

    // Full chain with descriptive output
    let message = find_train(&trains, 12049)
        .and_then(|train| {
            train
                .platform
                .map(|p| format!("{} departs from Platform {}", train.name, p))
        })
        .unwrap_or_else(|| String::from("Train or platform not found"));
    println!("{message}");

    // When the chain breaks in the middle
    let message = find_train(&trains, 12213)
        .and_then(|train| {
            train
                .platform
                .map(|p| format!("{} departs from Platform {}", train.name, p))
        })
        .unwrap_or_else(|| String::from("Train or platform not found"));
    println!("{message}");
}
