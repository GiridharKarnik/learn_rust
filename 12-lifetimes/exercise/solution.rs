// Lesson 12: Lifetimes — Railway Demo (SOLUTION)

// ---------------------------------------------------------------------------
// STEP 1: longest
// ---------------------------------------------------------------------------

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// ---------------------------------------------------------------------------
// STEP 2: Announcement struct + display method
// ---------------------------------------------------------------------------

struct Announcement<'a> {
    message: &'a str,
    platform: u8,
}

impl<'a> Announcement<'a> {
    fn display(&self) {
        println!("  Platform {}: {}", self.platform, self.message);
    }
}

// ---------------------------------------------------------------------------
// STEP 3: first_departure
// ---------------------------------------------------------------------------

fn first_departure<'a>(departures: &'a [&str]) -> Option<&'a str> {
    departures.first().copied()
}

// ---------------------------------------------------------------------------
// STEP 4: main
// ---------------------------------------------------------------------------

fn main() {
    println!("=== LIFETIME DEMO ===");

    println!();
    println!("--- longest ---");
    let name1 = String::from("Chennai Central");
    {
        let name2 = String::from("Thiruvananthapuram Central");
        let result = longest(name1.as_str(), name2.as_str());
        println!("  Longer station name: {result}");
    }

    println!();
    println!("--- Announcement ---");
    let msg = String::from("Rajdhani Express departing in 5 minutes");
    let ann = Announcement { message: &msg, platform: 3 };
    ann.display();

    println!();
    println!("--- first_departure ---");
    let departures = vec!["Chennai → Mumbai", "Delhi → Agra", "Kolkata → Darjeeling"];
    if let Some(first) = first_departure(&departures) {
        println!("  First on board: {first}");
    }

    println!();
    println!("=== DONE ===");
}
