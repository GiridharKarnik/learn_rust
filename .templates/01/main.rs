fn main() {
    // =============================================
    // EXERCISE 01: Build a Train Ticket
    // =============================================
    //
    // Complete each TODO below. When you're done, run:
    //   cargo run
    //
    // The program should compile and print a valid ticket.
    // Expected output:
    //
    //   === TRAIN TICKET ===
    //   Passenger: <your name>
    //   Train: Shatabdi Express
    //   Train Number: 12001
    //   From: Chennai Central (MAS)
    //   To: Bangalore City (SBC)
    //   Coach: B3
    //   Seat: 42
    //   Price: ₹755.50
    //   Express: Yes
    //
    //   Upgraded to coach: A1
    //   Price after 10% discount: ₹679.95

    // TODO 1: Create an immutable variable `passenger_name` with your name (use a &str)

    // TODO 2: Create a String (owned) called `train_name` with value "Shatabdi"
    //         Then append " Express" to it using .push_str()

    // TODO 3: Create a constant called TRAIN_NUMBER of type u32 with value 12001

    // TODO 4: Create two &str variables for station codes:
    //         `from_station` = "Chennai Central (MAS)"
    //         `to_station`   = "Bangalore City (SBC)"

    // TODO 5: Create a mutable variable `coach` of type &str set to "B3"

    // TODO 6: Create a variable `seat_number` of type u8 set to 42

    // TODO 7: Create a variable `price` of type f64 set to 755.50

    // TODO 8: Create a boolean `is_express` set to true

    // Now print the ticket!
    // Uncomment the lines below once you've created all the variables:

    // println!("=== TRAIN TICKET ===");
    // println!("Passenger: {passenger_name}");
    // println!("Train: {train_name}");
    // println!("Train Number: {TRAIN_NUMBER}");
    // println!("From: {from_station}");
    // println!("To: {to_station}");
    // println!("Coach: {coach}");
    // println!("Seat: {seat_number}");
    // println!("Price: ₹{price:.2}");       // :.2 formats to 2 decimal places
    // println!("Express: {}", if is_express { "Yes" } else { "No" });

    // TODO 9: Change `coach` to "A1" (this should work — why?)

    // TODO 10: Use shadowing to create a new `price` that is 90% of the original
    //          (multiply by 0.9)

    // Uncomment these too:
    // println!();
    // println!("Upgraded to coach: {coach}");
    // println!("Price after 10% discount: ₹{price:.2}");
}
