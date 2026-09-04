fn main() {
    println!("=== LOOPS PRACTICE ===\n");

    println!("--- Challenge 1: Platform Announcements ---");
    platform_announcements();

    println!("\n--- Challenge 2: Speed Control ---");
    speed_control();

    println!("\n--- Challenge 3: Passenger Boarding ---");
    passenger_boarding();

    println!("\n--- Challenge 4: Station Finder ---");
    let found = station_finder();
    println!("Result: {found}");

    println!("\n--- Challenge 5: Ticket Queue ---");
    ticket_queue();

    println!("\n--- Challenge 6: Train Timetable ---");
    timetable();
}

// =============================================
// CHALLENGE 1: Platform Announcements (for loop)
// =============================================
//
// Print announcements for platforms 1 through 8.
// But platforms 4 and 7 are closed for maintenance — skip them.
//
// Expected output:
//   Platform 1: Now boarding
//   Platform 2: Now boarding
//   Platform 3: Now boarding
//   Platform 5: Now boarding
//   Platform 6: Now boarding
//   Platform 8: Now boarding

fn platform_announcements() {
    // TODO: for loop over 1..=8, skip 4 and 7 using `continue`
}

// =============================================
// CHALLENGE 2: Speed Control (while loop)
// =============================================
//
// A train starts at 0 km/h and accelerates by 15 km/h each step.
// Print the speed at each step.
// Once it reaches or exceeds 120, print a cruising message and stop.
// If it hits exactly 60, print a speed check message (but keep going).
//
// Expected output:
//   Speed: 15 km/h
//   Speed: 30 km/h
//   Speed: 45 km/h
//   Speed: 60 km/h ← speed check passed
//   Speed: 75 km/h
//   Speed: 90 km/h
//   Speed: 105 km/h
//   Speed: 120 km/h ← cruising speed reached!

fn speed_control() {
    // TODO: Use a while loop. Increment speed by 15 each iteration.
    //       Print each speed. Add the special messages for 60 and >= 120.
}

// =============================================
// CHALLENGE 3: Passenger Boarding (countdown with while)
// =============================================
//
// 50 passengers need to board. They board in groups of 8.
// After each group, print how many are left.
// When fewer than 8 remain, board the rest and stop.
//
// Expected output:
//   Boarded 8 passengers, 42 remaining
//   Boarded 8 passengers, 34 remaining
//   Boarded 8 passengers, 26 remaining
//   Boarded 8 passengers, 18 remaining
//   Boarded 8 passengers, 10 remaining
//   Boarded 8 passengers, 2 remaining
//   Final group: boarded last 2 passengers
//   All 50 passengers aboard!

fn passenger_boarding() {
    // TODO: Start with 50 passengers remaining.
    //       Use a loop. Each iteration, board 8 (or fewer if less than 8 remain).
    //       Print the appropriate message each time.
}

// =============================================
// CHALLENGE 4: Station Finder (loop with break returning a value)
// =============================================
//
// Search through a list of stations for "Katpadi".
// Return the message "Katpadi found at stop 3" (1-indexed).
// If not found, return "Station not found".
//
// Expected output:
//   Result: Katpadi found at stop 3

fn station_finder() -> String {
    let stations = ["Chennai", "Perambur", "Katpadi", "Jolarpettai", "Bangalore"];

    // TODO: Use a for loop with .iter().enumerate() to search for "Katpadi".
    //       When found, return a formatted String (use `format!` macro — it works
    //       just like `println!` but returns a String instead of printing).
    //       Example: format!("hello {name}") returns the String "hello <name>"
    //
    //       If the loop ends without finding it, return "Station not found".

    String::from("Station not found")
}

// =============================================
// CHALLENGE 5: Ticket Queue (nested loops)
// =============================================
//
// There are 3 ticket counters. Each counter serves 4 passengers.
// But passenger 2 at counter 2 has an invalid ticket — skip them.
//
// Expected output:
//   Counter 1: served passenger 1
//   Counter 1: served passenger 2
//   Counter 1: served passenger 3
//   Counter 1: served passenger 4
//   Counter 2: served passenger 1
//   Counter 2: ⚠ passenger 2 invalid ticket — skipped
//   Counter 2: served passenger 3
//   Counter 2: served passenger 4
//   Counter 3: served passenger 1
//   Counter 3: served passenger 2
//   Counter 3: served passenger 3
//   Counter 3: served passenger 4

fn ticket_queue() {
    // TODO: Nested for loops — outer for counters 1..=3, inner for passengers 1..=4.
    //       Skip (continue) when counter == 2 AND passenger == 2.
}

// =============================================
// CHALLENGE 6: Train Timetable (for loop + formatted output)
// =============================================
//
// Given arrays of train data, print a formatted timetable.
// Only print trains that are NOT cancelled (use continue to skip cancelled ones).
//
// Expected output:
//   Train          | Departs | Platform
//   ---------------+---------+---------
//   Rajdhani       | 06:00   | 1
//   Shatabdi       | 07:30   | 2
//   Duronto        | 10:15   | 3
//   Garib Rath     | 22:00   | 5
//
//   4 trains scheduled

fn timetable() {
    let names = ["Rajdhani", "Shatabdi", "Kovai Exp", "Duronto", "Garib Rath"];
    let departures = ["06:00", "07:30", "CANCELLED", "10:15", "22:00"];
    let platforms = [1, 2, 4, 3, 5];

    // TODO: Print the header lines, then loop through the arrays using index.
    //       Skip cancelled trains (where departure is "CANCELLED") with `continue`.
    //       Count how many trains you actually printed.
    //       After the loop, print the count.
    //
    //       For aligned columns, use format width specifiers:
    //         {:<14}  means "left-align, pad to 14 characters"
    //         Example: println!("{:<14} | {:<7} | {}", name, time, plat);
}
