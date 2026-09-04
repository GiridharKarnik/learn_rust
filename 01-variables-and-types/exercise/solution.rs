// =============================================
// SOLUTION — Don't peek until you've tried!
// =============================================

fn main() {
    // TODO 1
    let passenger_name: &str = "Giridhar";

    // TODO 2
    let mut train_name = String::from("Shatabdi");
    train_name.push_str(" Express");

    // TODO 3
    const TRAIN_NUMBER: u32 = 12001;

    // TODO 4
    let from_station = "Chennai Central (MAS)";
    let to_station = "Bangalore City (SBC)";

    // TODO 5
    let mut coach: &str = "B3";

    // TODO 6
    let seat_number: u8 = 42;

    // TODO 7
    let price: f64 = 755.50;

    // TODO 8
    let is_express = true;

    println!("=== TRAIN TICKET ===");
    println!("Passenger: {passenger_name}");
    println!("Train: {train_name}");
    println!("Train Number: {TRAIN_NUMBER}");
    println!("From: {from_station}");
    println!("To: {to_station}");
    println!("Coach: {coach}");
    println!("Seat: {seat_number}");
    println!("Price: ₹{price:.2}");
    println!("Express: {}", if is_express { "Yes" } else { "No" });

    // TODO 9: coach is mut, so we can reassign it
    coach = "A1";

    // TODO 10: shadowing creates a brand new `price` — the old one is gone
    let price = price * 0.9;

    println!();
    println!("Upgraded to coach: {coach}");
    println!("Price after 10% discount: ₹{price:.2}");
}
