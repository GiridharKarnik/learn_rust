// =============================================
// SOLUTION — Don't peek until you've tried!
// =============================================

fn announce_train(name: &str) {
    println!("📢 Now arriving: {name}");
}

fn rename_train(name: &mut String, new_name: &str) {
    name.clear();
    name.push_str(new_name);
}

fn create_ticket(train: &str, seat: u32) -> String {
    format!("{train} - Seat {seat}")
}

fn format_route(stations: &[&str]) -> String {
    stations.join(" → ")
}

fn main() {
    println!("=== OWNERSHIP & BORROWING ===");

    // --- Immutable borrowing ---
    println!();
    println!("--- Borrowing (immutable) ---");
    let train = String::from("Shatabdi Express");
    announce_train(&train);
    println!("Train is still accessible: {train}");

    // --- Mutable borrowing ---
    println!();
    println!("--- Mutable Borrowing ---");
    let mut train = String::from("Rajdhani Express");
    println!("Before rename: {train}");
    rename_train(&mut train, "Vande Bharat Express");
    println!("After rename: {train}");

    // --- Borrow in, own out ---
    println!();
    println!("--- Borrow In, Own Out ---");
    let ticket1 = create_ticket("Shatabdi Express", 42);
    let ticket2 = create_ticket("Duronto Express", 15);
    println!("Ticket 1: {ticket1}");
    println!("Ticket 2: {ticket2}");

    // --- Clone before move ---
    println!();
    println!("--- Clone Before Move ---");
    let original = String::from("Garib Rath Express");
    let backup = original.clone();
    let moved = original;
    println!("Backup copy: {backup}");
    println!("Moved value: {moved}");

    // --- Borrowed slice → owned String ---
    println!();
    println!("--- Borrowed Slice → Owned String ---");
    let stations = ["Chennai", "Katpadi", "Jolarpettai", "Bangalore"];
    let route = format_route(&stations);
    println!("Route: {route}");
    println!("Stations array still accessible: {} stops", stations.len());
}
