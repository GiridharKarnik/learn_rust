fn main() {
    // =============================================
    // LESSON 03: Ownership & Borrowing — Examples
    // =============================================

    // --- 1. Ownership and scope ---
    println!("--- 1. Scope ---");
    {
        let station = String::from("Chennai Central");
        println!("Inside block: {station}");
    } // station is dropped here — memory freed

    // println!("{station}"); // ❌ would fail — station is gone

    // --- 2. Move semantics ---
    println!("\n--- 2. Move ---");
    let train_a = String::from("Shatabdi Express");
    let train_b = train_a; // ownership MOVES to train_b

    // println!("{train_a}"); // ❌ would fail — train_a is invalid
    println!("train_b owns it: {train_b}");

    // --- 3. Clone — explicit deep copy ---
    println!("\n--- 3. Clone ---");
    let original = String::from("Rajdhani Express");
    let copy = original.clone(); // deep copy — both are valid

    println!("Original: {original}");
    println!("Copy:     {copy}");

    // --- 4. Copy types (stack values) ---
    println!("\n--- 4. Copy types ---");
    let platform = 5;
    let other_platform = platform; // COPIED, not moved

    println!("Both work: {platform} and {other_platform}");

    let is_express = true;
    let also_express = is_express; // also copied
    println!("Both work: {is_express} and {also_express}");

    // --- 5. Ownership and functions ---
    println!("\n--- 5. Functions take ownership ---");
    let train = String::from("Duronto Express");
    consume_train(train); // ownership moved into the function
                          // println!("{train}"); // ❌ would fail — train was consumed

    // --- 6. Borrowing with & ---
    println!("\n--- 6. Immutable borrowing ---");
    let train = String::from("Garib Rath");
    print_train_info(&train); // we LEND it
    println!("Still ours: {train}"); // ✅ we still own it

    // Multiple immutable borrows — totally fine
    let r1 = &train;
    let r2 = &train;
    println!("Two borrows: {r1} and {r2}");

    // --- 7. &str vs &String ---
    println!("\n--- 7. &str in function params ---");
    let owned = String::from("Shatabdi Express");
    let literal = "Rajdhani Express";

    display_train(&owned); // &String → auto-converts to &str
    display_train(literal); // &str → works directly

    // --- 8. Mutable borrowing ---
    println!("\n--- 8. Mutable borrowing ---");
    let mut train = String::from("Rajdhani");
    println!("Before: {train}");

    upgrade_train(&mut train);
    println!("After:  {train}");

    // --- 9. The borrow rules in action ---
    println!("\n--- 9. Borrow rules ---");
    let mut data = String::from("Platform 1");

    // Multiple immutable borrows — OK
    let r1 = &data;
    let r2 = &data;
    println!("Reading: {r1} and {r2}");
    // r1 and r2 are no longer used after this point,
    // so their borrows end here (Non-Lexical Lifetimes)

    // Now we can mutably borrow — the immutable borrows are done
    let r3 = &mut data;
    r3.push_str(" (updated)");
    println!("Modified: {r3}");

    // --- 10. Returning ownership ---
    println!("\n--- 10. Returning ownership ---");
    let new_train = create_train("Vande Bharat", 22436);
    println!("Created: {new_train}");

    // --- 11. String slices ---
    println!("\n--- 11. String slices ---");
    let full_name = String::from("Chennai Central");
    let city = &full_name[0..7]; // borrow a slice
    let station_type = &full_name[8..]; // from index 8 to end
    println!("City: {city}");
    println!("Type: {station_type}");
    println!("Full: {full_name}"); // original still valid

    // --- 12. Practical pattern: borrow in, own out ---
    println!("\n--- 12. Borrow in, own out ---");
    let stations = ["Chennai", "Katpadi", "Bangalore"];
    let announcement = build_announcement(&stations);
    println!("{announcement}");
}

// Takes ownership — caller loses access
fn consume_train(name: String) {
    println!("Consumed: {name}");
} // name is dropped here

// Borrows — caller keeps ownership
fn print_train_info(name: &String) {
    println!("Train info: {name} ({} chars)", name.len());
}

// Idiomatic: accepts &str (works with both &String and &str)
fn display_train(name: &str) {
    println!("🚂 {name}");
}

// Mutable borrow — can modify the original
fn upgrade_train(name: &mut String) {
    name.push_str(" Express");
}

// Returns an owned String — caller gets ownership
fn create_train(name: &str, number: u32) -> String {
    format!("{name} (#{number})")
}

// Borrows an array of &str, returns an owned String
fn build_announcement(stations: &[&str]) -> String {
    let mut result = String::from("Stopping at: ");
    for (i, station) in stations.iter().enumerate() {
        if i > 0 {
            result.push_str(", ");
        }
        result.push_str(station);
    }
    result
}
