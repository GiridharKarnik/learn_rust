// =============================================
// EXERCISE 12: Lifetimes
// =============================================
//
// Practice the three core lifetime patterns:
//   - Annotated functions that return a reference derived from their inputs
//   - Structs that hold references
//   - Functions over slices of references
//
// You know: variables, functions, ownership, structs, enums, Option, Result,
// Vec, HashMap, iterators, traits. Now add lifetimes to your toolkit.
//
// Run with: cargo run
//
// Expected output:
//
//   === LIFETIME DEMO ===
//
//   --- longest ---
//     Longer station name: Thiruvananthapuram Central
//
//   --- Announcement ---
//     Platform 3: Rajdhani Express departing in 5 minutes
//
//   --- first_departure ---
//     First on board: Chennai → Mumbai
//
//   === DONE ===

// =============================================
// STEP 1: Write `longest`
// =============================================
// fn longest<'a>(a: &'a str, b: &'a str) -> &'a str
//
// Return whichever string is longer (use .len()).
//
// Hint: this is the canonical lifetime annotation example.
// Both inputs share lifetime 'a; the return borrows from one of them.
// Without the annotation the compiler cannot tell which input the
// return comes from, so it rejects the function.

// TODO: implement `longest` here

// =============================================
// STEP 2: Define `Announcement` and implement `display`
// =============================================
// struct Announcement<'a> with fields:
//   message: &'a str
//   platform: u8
//
// Implement a method:
//   fn display(&self)
//   prints: "  Platform {platform}: {message}"
//
// The struct cannot outlive the string slice it borrows.

// TODO: define `Announcement` and its impl block here

// =============================================
// STEP 3: Write `first_departure`
// =============================================
// fn first_departure<'a>(departures: &'a [&str]) -> Option<&'a str>
//
// Return the first element of the slice.
// Hint: departures.first().copied()

// TODO: implement `first_departure` here

// =============================================
// STEP 4: Write main()
// =============================================
//
// 1. Print "=== LIFETIME DEMO ==="
//
// 2. Print "" (blank line)
//
// 3. Print "--- longest ---"
//    let name1 = String::from("Chennai Central");
//    let result;
//    {
//        let name2 = String::from("Thiruvananthapuram Central");
//        result = longest(name1.as_str(), name2.as_str());
//        println!("  Longer station name: {result}");
//    }
//    (Note: result is used inside the inner scope where name2 is alive.
//     Try moving the println! outside the braces and watch the compiler
//     reject it — that is the borrow checker doing its job.)
//
// 4. Print "" (blank line)
//
// 5. Print "--- Announcement ---"
//    let msg = String::from("Rajdhani Express departing in 5 minutes");
//    let ann = Announcement { message: &msg, platform: 3 };
//    ann.display();
//
// 6. Print "" (blank line)
//
// 7. Print "--- first_departure ---"
//    let departures = vec!["Chennai → Mumbai", "Delhi → Agra", "Kolkata → Darjeeling"];
//    if let Some(first) = first_departure(&departures) {
//        println!("  First on board: {first}");
//    }
//
// 8. Print "" (blank line)
//
// 9. Print "=== DONE ==="

fn main() {}
