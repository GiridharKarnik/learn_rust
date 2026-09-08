// =============================================
// EXERCISE 03: Ownership & Borrowing
// =============================================
//
// Write ALL functions AND main() yourself.
// This exercise builds functions that demonstrate ownership patterns.
// Your program should produce EXACTLY this output:
//
// === OWNERSHIP & BORROWING ===
//
// --- Borrowing (immutable) ---
// 📢 Now arriving: Shatabdi Express
// Train is still accessible: Shatabdi Express
//
// --- Mutable Borrowing ---
// Before rename: Rajdhani Express
// After rename: Vande Bharat Express
//
// --- Borrow In, Own Out ---
// Ticket 1: Shatabdi Express - Seat 42
// Ticket 2: Duronto Express - Seat 15
//
// --- Clone Before Move ---
// Backup copy: Garib Rath Express
// Moved value: Garib Rath Express
//
// --- Borrowed Slice → Owned String ---
// Route: Chennai → Katpadi → Jolarpettai → Bangalore
// Stations array still accessible: 4 stops
//
// =============================================
// STEP 1: Write fn announce_train(name: &str)
//   - Takes an immutable borrow of a string
//   - Prints "📢 Now arriving: {name}"
//   - The caller keeps ownership — this function only reads
//
// STEP 2: Write fn rename_train(name: &mut String, new_name: &str)
//   - Takes a mutable borrow of a String and an immutable borrow of the new name
//   - Clears the original string and pushes the new name into it
//   - Hint: name.clear() then name.push_str(new_name)
//
// STEP 3: Write fn create_ticket(train: &str, seat: u32) -> String
//   - Borrows the train name and takes a seat number
//   - Returns a new owned String: "{train} - Seat {seat}"
//   - This demonstrates "borrow in, own out" — inputs are borrowed,
//     but the function creates and returns a brand new String
//
// STEP 4: Write fn format_route(stations: &[&str]) -> String
//   - Takes a borrowed slice of string slices
//   - Returns an owned String joining all stations with " → "
//   - Hint: you can use stations.join(" → ") or build it manually with a loop
//
// STEP 5: Write main() that demonstrates each ownership concept:
//
//   Print "=== OWNERSHIP & BORROWING ==="
//
//   a) Immutable Borrowing:
//      - Print header "--- Borrowing (immutable) ---"
//      - Create a String "Shatabdi Express"
//      - Pass &reference to announce_train (the function borrows it)
//      - Print the train name again to prove you still own it
//
//   b) Mutable Borrowing:
//      - Print header "--- Mutable Borrowing ---"
//      - Create a mutable String "Rajdhani Express"
//      - Print "Before rename: {train}"
//      - Pass &mut reference to rename_train with new name "Vande Bharat Express"
//      - Print "After rename: {train}" to see the change
//
//   c) Borrow In, Own Out:
//      - Print header "--- Borrow In, Own Out ---"
//      - Call create_ticket("Shatabdi Express", 42) → ticket1
//      - Call create_ticket("Duronto Express", 15) → ticket2
//      - Print both tickets
//
//   d) Clone Before Move:
//      - Print header "--- Clone Before Move ---"
//      - Create a String "Garib Rath Express"
//      - Clone it to create a backup copy
//      - Move the original to a new variable (let moved = original)
//      - Print both the backup and the moved value
//      - Note: after the move, `original` is no longer valid — only backup and moved are
//
//   e) Borrowed Slice → Owned String:
//      - Print header "--- Borrowed Slice → Owned String ---"
//      - Create an array of station names: ["Chennai", "Katpadi", "Jolarpettai", "Bangalore"]
//      - Pass &stations to format_route (borrows the array as a slice)
//      - Print "Route: {route}"
//      - Print the stations array length to prove it's still accessible

fn main() {
    // Your code here
}
