// =============================================
// EXERCISE 01: Build a Train Ticket
// =============================================
//
// Write ALL the code yourself inside fn main().
// Your program should produce EXACTLY this output:
//
// === TRAIN TICKET ===
// Passenger: Giridhar
// Train: Shatabdi Express
// Train Number: 12001
// From: Chennai Central (MAS)
// To: Bangalore City (SBC)
// Coach: B3
// Seat: 42
// Price: ₹755.50
// Express: Yes
//
// Upgraded to coach: A1
// Price after 10% discount: ₹679.95
//
// STEP 1:  Create an immutable variable `passenger_name` of type &str with value "Giridhar"
// STEP 2:  Create a mutable String called `train_name` with value "Shatabdi", then append " Express"
//          Hint: String::from("...") and .push_str("...")
// STEP 3:  Create a constant TRAIN_NUMBER of type u32 with value 12001
// STEP 4:  Create two &str variables: `from_station` = "Chennai Central (MAS)"
//          and `to_station` = "Bangalore City (SBC)"
// STEP 5:  Create a mutable variable `coach` of type &str with value "B3"
// STEP 6:  Create a variable `seat_number` of type u8 with value 42
// STEP 7:  Create a variable `price` of type f64 with value 755.50
// STEP 8:  Create a variable `is_express` of type bool with value true
// STEP 9:  Print the full ticket using println! matching the expected output above
//          Hint: Use {variable_name} for inline formatting
//          Hint: Use {price:.2} to format floats to 2 decimal places
//          Hint: For Express line, use: if is_express { "Yes" } else { "No" }
// STEP 10: Change `coach` to "A1" (this works because coach is mutable)
// STEP 11: Shadow `price` with 90% of its original value (price * 0.9)
//          Hint: Shadowing means using `let price = ...` again
// STEP 12: Print the upgrade info (empty line, then the two upgrade lines)

fn main() {
    // Your code here
}
