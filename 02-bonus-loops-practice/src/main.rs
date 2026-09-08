// =============================================
// EXERCISE 02-BONUS: Loops Practice
// =============================================
//
// Write ALL 6 challenge functions AND main() yourself.
// Your program should produce EXACTLY this output:
//
// === LOOPS PRACTICE ===
//
// --- Challenge 1: Platform Announcements ---
// Platform 1: Now boarding
// Platform 2: Now boarding
// Platform 3: Now boarding
// Platform 5: Now boarding
// Platform 6: Now boarding
// Platform 8: Now boarding
//
// --- Challenge 2: Speed Control ---
// Speed: 15 km/h
// Speed: 30 km/h
// Speed: 45 km/h
// Speed: 60 km/h ← speed check passed
// Speed: 75 km/h
// Speed: 90 km/h
// Speed: 105 km/h
// Speed: 120 km/h ← cruising speed reached!
//
// --- Challenge 3: Passenger Boarding ---
// Boarded 8 passengers, 42 remaining
// Boarded 8 passengers, 34 remaining
// Boarded 8 passengers, 26 remaining
// Boarded 8 passengers, 18 remaining
// Boarded 8 passengers, 10 remaining
// Boarded 8 passengers, 2 remaining
// Final group: boarded last 2 passengers
// All 50 passengers aboard!
//
// --- Challenge 4: Station Finder ---
// Result: Katpadi found at stop 3
//
// --- Challenge 5: Ticket Queue ---
// Counter 1: served passenger 1
// Counter 1: served passenger 2
// Counter 1: served passenger 3
// Counter 1: served passenger 4
// Counter 2: served passenger 1
// Counter 2: ⚠ passenger 2 invalid ticket — skipped
// Counter 2: served passenger 3
// Counter 2: served passenger 4
// Counter 3: served passenger 1
// Counter 3: served passenger 2
// Counter 3: served passenger 3
// Counter 3: served passenger 4
//
// --- Challenge 6: Train Timetable ---
// Train          | Departs | Platform
// ---------------+---------+---------
// Rajdhani       | 06:00   | 1
// Shatabdi       | 07:30   | 2
// Duronto        | 10:15   | 3
// Garib Rath     | 22:00   | 5
//
// 4 trains scheduled
//
// =============================================
// STEP 1: Implement platform_announcements()
//   - Use a for loop over platforms 1..=8
//   - Skip platforms 4 and 7 (use continue)
//   - Print "Platform {n}: Now boarding" for all others
//
// STEP 2: Implement speed_control()
//   - Start with speed = 0 (u32)
//   - Use a while loop that runs while speed < 120
//   - Each iteration: add 15 to speed, then print:
//     - At exactly 60:  "Speed: {speed} km/h ← speed check passed"
//     - At 120 or above: "Speed: {speed} km/h ← cruising speed reached!"
//     - Otherwise:       "Speed: {speed} km/h"
//
// STEP 3: Implement passenger_boarding()
//   - Total passengers = 50, board 8 at a time
//   - Use a loop (infinite loop with break)
//   - If remaining >= 8: subtract 8, print "Boarded 8 passengers, {remaining} remaining"
//   - Otherwise: print "Final group: boarded last {remaining} passengers", set remaining to 0
//   - Break when remaining == 0
//   - After the loop: print "All {total} passengers aboard!"
//
// STEP 4: Implement station_finder() -> String
//   - Create an array: ["Chennai", "Perambur", "Katpadi", "Jolarpettai", "Bangalore"]
//   - Use a for loop with .iter().enumerate() to get (index, station)
//   - When you find "Katpadi", return format!("{station} found at stop {}", index + 1)
//   - If not found (after the loop), return String::from("Station not found")
//   - Hint: compare with *station == "Katpadi" (dereference the iterator item)
//
// STEP 5: Implement ticket_queue()
//   - Use nested for loops: counters 1..=3 (outer), passengers 1..=4 (inner)
//   - At counter 2, passenger 2: print "Counter {c}: ⚠ passenger {p} invalid ticket — skipped"
//     and continue
//   - All others: print "Counter {c}: served passenger {p}"
//   - Hint: the — is an em dash (Unicode \u{2014})
//
// STEP 6: Implement timetable()
//   - Create three parallel arrays:
//     names:      ["Rajdhani", "Shatabdi", "Kovai Exp", "Duronto", "Garib Rath"]
//     departures: ["06:00", "07:30", "CANCELLED", "10:15", "22:00"]
//     platforms:  [1, 2, 4, 3, 5]
//   - Print a table header: "{:<14} | {:<7} | {}" with "Train", "Departs", "Platform"
//   - Print a separator: "---------------+---------+---------"
//   - Loop through indices 0..names.len()
//   - Skip any train where departure is "CANCELLED" (use continue)
//   - Print each active train formatted like the header
//   - Track count of active trains, print "{count} trains scheduled" at the end
//
// STEP 7: Write main() that:
//   - Prints "=== LOOPS PRACTICE ===" followed by an empty line
//   - Calls each challenge function with its header:
//     "--- Challenge 1: Platform Announcements ---"
//     "--- Challenge 2: Speed Control ---"
//     "--- Challenge 3: Passenger Boarding ---"
//     "--- Challenge 4: Station Finder ---"       (capture return value, print "Result: {found}")
//     "--- Challenge 5: Ticket Queue ---"
//     "--- Challenge 6: Train Timetable ---"
//   - Separate each section with \n before the header (use "\n--- Challenge..." in println!)

fn main() {
    // Your code here
}
