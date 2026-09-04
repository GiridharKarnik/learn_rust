// =============================================
// SOLUTION — Don't peek until you've tried!
// =============================================

fn main() {
    println!("=== PART A: Fix the bugs ===");

    // FIX 1: Clone so both variables own their own copy
    let train_a = String::from("Shatabdi Express");
    let train_b = train_a.clone();
    println!("fix1: {train_a} and {train_b}");

    // FIX 2: Pass a reference so caller keeps ownership
    let train = String::from("Rajdhani Express");
    print_name(&train);
    println!("fix2: {train}");

    // FIX 3: Function borrows instead of taking ownership
    let my_train = String::from("Duronto Express");
    display_train(&my_train);
    println!("fix3: Still mine: {my_train}");

    // FIX 4: Need mut on the variable AND &mut in the function
    let mut platform_info = String::from("Platform 3");
    assign_train(&mut platform_info, "Rajdhani Express");
    println!("fix4: Updated: {platform_info}");

    // FIX 5: Function borrows instead of taking ownership
    let station = String::from("Chennai Central");
    let len = get_length(&station);
    println!("fix5: {station} ({len} chars)");
    println!("fix5: {station}");

    // FIX 6: Clone to avoid moving — both swapped values are independent
    let first = String::from("Rajdhani");
    let second = String::from("Shatabdi");
    let temp = first.clone();
    let first = second;
    let second = temp;
    println!("fix6: First: {first}, Second: {second}");

    // FIX 7: Capture the returned String into a new binding
    let name = String::from("Vande Bharat");
    let name = make_express(name);
    println!("fix7: {name}");

    println!();
    println!("=== PART B: Build functions ===");

    let ticket = format_ticket("Giridhar", "Shatabdi Express", "Chennai", "Bangalore");
    println!("{ticket}");

    let stops = ["Chennai", "Katpadi", "Jolarpettai", "Bangalore"];
    let description = describe_route(&stops);
    println!("{description}");

    let mut train = String::from("Rajdhani");
    upgrade_name(&mut train);
    println!("Upgraded: {train}");
}

// Part A — fixed signatures

fn print_name(name: &str) {
    println!("fix2: {name}");
}

fn display_train(name: &str) {
    println!("fix3: Train: {name}");
}

fn assign_train(info: &mut String, train: &str) {
    info.push_str(" \u{2014} ");
    info.push_str(train);
}

fn get_length(s: &str) -> usize {
    s.len()
}

fn make_express(mut name: String) -> String {
    name.push_str(" Express");
    name
}

// Part B — implementations

fn format_ticket(passenger: &str, train: &str, from: &str, to: &str) -> String {
    format!("Ticket: {passenger} | {train} | {from} \u{2192} {to}")
}

fn describe_route(stations: &[&str]) -> String {
    let mut route = String::from("Route: ");
    for (i, station) in stations.iter().enumerate() {
        if i > 0 {
            route.push_str(" \u{2192} ");
        }
        route.push_str(station);
    }
    route.push_str(&format!(" ({} stops)", stations.len()));
    route
}

fn upgrade_name(name: &mut String) {
    let upper = name.to_uppercase();
    name.clear();
    name.push_str(&upper);
    name.push_str(" EXPRESS");
}
