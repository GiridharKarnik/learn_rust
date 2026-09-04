// =============================================
// EXERCISE 03: Ownership & Borrowing
// =============================================
//
// This exercise has two parts:
//   Part A — Fix the broken code (7 compiler errors to fix)
//   Part B — Build functions using the right ownership patterns
//
// Run with: cargo run
//
// Expected output when everything is fixed:
//
//   === PART A: Fix the bugs ===
//   fix1: Shatabdi Express and Shatabdi Express
//   fix2: Rajdhani Express
//   fix3: Train: Duronto Express
//   fix3: Still mine: Duronto Express
//   fix4: Updated: Platform 3 — Rajdhani Express
//   fix5: Chennai Central (15 chars)
//   fix5: Chennai Central
//   fix6: First: Rajdhani, Second: Shatabdi
//   fix7: Vande Bharat Express
//
//   === PART B: Build functions ===
//   Ticket: Giridhar | Shatabdi Express | Chennai → Bangalore
//   Route: Chennai → Katpadi → Jolarpettai → Bangalore (4 stops)
//   Upgraded: RAJDHANI EXPRESS

fn main() {
    println!("=== PART A: Fix the bugs ===");

    // FIX 1: This fails because ownership moves. Make both prints work
    //        WITHOUT changing the first let or removing train_b.
    //        Hint: you need both variables to have their own copy.
    // let train_a = String::from("Shatabdi Express");
    // let train_b = train_a;
    // println!("fix1: {train_a} and {train_b}");

    // FIX 2: This fails because the function takes ownership.
    //        Fix the function signature and call so the caller keeps the String.
    // let train = String::from("Rajdhani Express");
    // print_name(train);
    // println!("fix2: {train}");

    // FIX 3: This function takes a String but it should borrow instead.
    //        Fix the function AND the call site.
    // let my_train = String::from("Duronto Express");
    // display_train(my_train);
    // println!("fix3: Still mine: {my_train}");

    // FIX 4: This function needs to modify the string, but it's not set up correctly.
    //        Fix everything needed to make the mutation work.
    // let platform_info = String::from("Platform 3");
    // assign_train(platform_info, "Rajdhani Express");
    // println!("fix4: Updated: {platform_info}");

    // FIX 5: This function returns the length, but the borrow isn't right.
    //        Fix the function signature.
    // let station = String::from("Chennai Central");
    // let len = get_length(station);
    // println!("fix5: {station} ({len} chars)");
    // println!("fix5: {station}");

    // FIX 6: Two variables try to own the same String.
    //        Make this work so both names are usable.
    // let first = String::from("Rajdhani");
    // let second = String::from("Shatabdi");
    // let first = second;
    // let second = first;
    // println!("fix6: First: {first}, Second: {second}");

    // FIX 7: The function modifies a String and returns it, but the caller
    //        doesn't capture the return value properly.
    // let name = String::from("Vande Bharat");
    // make_express(name);
    // println!("fix7: {name}");

    println!();
    println!("=== PART B: Build functions ===");

    // =============================================
    // Part B — implement these from scratch
    // =============================================

    // TODO 1: Implement `format_ticket`
    // Takes passenger name (&str), train name (&str), from (&str), to (&str)
    // Returns an owned String: "Ticket: {passenger} | {train} | {from} → {to}"
    //
    // let ticket = format_ticket("Giridhar", "Shatabdi Express", "Chennai", "Bangalore");
    // println!("{ticket}");

    // TODO 2: Implement `describe_route`
    // Takes a borrowed slice of station names (&[&str])
    // Returns a String: "Route: A → B → C → D (N stops)"
    //
    // let stops = ["Chennai", "Katpadi", "Jolarpettai", "Bangalore"];
    // let description = describe_route(&stops);
    // println!("{description}");

    // TODO 3: Implement `upgrade_name`
    // Takes a mutable reference to a String (&mut String)
    // Converts the string to uppercase and appends " EXPRESS"
    // Returns nothing (modifies in place)
    //
    // Hint: Use .to_uppercase() which returns a new String, then use
    //       .clear() to empty the original and .push_str() to fill it.
    //
    // let mut train = String::from("Rajdhani");
    // upgrade_name(&mut train);
    // println!("Upgraded: {train}");
}

// =============================================
// Part A function stubs — fix these signatures
// =============================================

fn print_name(name: String) {
    println!("fix2: {name}");
}

fn display_train(name: String) {
    println!("fix3: Train: {name}");
}

fn assign_train(info: String, train: &str) {}

fn get_length(s: String) -> usize {
    s.len()
}

fn make_express(mut name: String) -> String {
    name.push_str(" Express");
    name
}

// =============================================
// Part B function stubs — implement these
// =============================================

// TODO 1: fn format_ticket(...) -> String { }

// TODO 2: fn describe_route(...) -> String { }

// TODO 3: fn upgrade_name(...) { }
