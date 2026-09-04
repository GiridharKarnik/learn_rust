// =============================================
// EXERCISE 04: Railway Station Management System
// =============================================
//
// Build a small railway system using structs and methods.
// Complete all the TODOs so the program compiles and produces the expected output.
//
// Run with: cargo run
//
// Expected output:
//
//   === RAILWAY STATION MANAGEMENT ===
//
//   --- Creating Trains ---
//   Created: Rajdhani Express (#12001) [Express]
//   Created: Shatabdi Express (#12002) [Express]
//   Created: Chennai Local (#43210) [Regular]
//
//   --- Station Info ---
//   Chennai Central (MAS) — 17 platforms
//   Bangalore City (SBC) — 10 platforms
//
//   --- Train Operations ---
//   Rajdhani Express: 0 km/h → accelerating...
//   Rajdhani Express: now at 80 km/h
//   Rajdhani Express: now at 130 km/h
//   Rajdhani Express is fast: true
//   Chennai Local is fast: false
//
//   --- Arrivals ---
//   📢 MAS: Rajdhani Express (#12001) arriving on platform 1
//   📢 MAS: Shatabdi Express (#12002) arriving on platform 3
//   📢 SBC: Chennai Local (#43210) arriving on platform 5
//
//   --- Passenger Manifest ---
//   Passenger: Giridhar Mohan (giridhar@email.com)
//     Ticket #1001: Rajdhani Express | MAS → SBC | ₹1250.00 | Confirmed
//     Ticket #1002: Shatabdi Express | SBC → MAS | ₹875.50 | Waitlisted
//
//   --- Ticket Operations ---
//   Ticket #1002 confirmed!
//     Ticket #1002: Shatabdi Express | SBC → MAS | ₹875.50 | Confirmed
//   Ticket #1001 cancelled. Refund: ₹1062.50
//     Ticket #1001: Rajdhani Express | MAS → SBC | ₹1062.50 | Cancelled (refund pending)
//
//   --- Summary ---
//   Total tickets: 2
//   Total spent: ₹1937.50 (before cancellations)

// =============================================
// TODO 1: Define the `Train` struct
// =============================================
//
// Fields:
//   - name: String
//   - number: u32
//   - speed_kmh: f64
//   - is_express: bool
//
// Derive: Debug, Clone

// =============================================
// TODO 2: Implement methods for `Train`
// =============================================
//
// Associated functions (constructors):
//   - new(name: &str, number: u32) -> Self
//     Creates a regular train with speed 0.0 and is_express false
//
//   - new_express(name: &str, number: u32) -> Self
//     Creates an express train with speed 0.0 and is_express true
//
// Methods:
//   - display(&self)
//     Prints: "Created: {name} (#{number}) [Express]" or "[Regular]"
//
//   - accelerate(&mut self, amount: f64)
//     Adds amount to speed_kmh, caps at 200.0
//     Prints: "{name}: now at {speed_kmh} km/h"
//
//   - is_fast(&self) -> bool
//     Returns true if speed_kmh > 100.0

//
// =============================================
// TODO 3: Define the `Station` struct
// =============================================
//
// Fields:
//   - name: String
//   - code: String
//   - platforms: u8
//
// Derive: Debug

// =============================================
// TODO 4: Implement methods for `Station`
// =============================================
//
// Associated function:
//   - new(name: &str, code: &str, platforms: u8) -> Self
//
// Methods:
//   - info(&self)
//     Prints: "{name} ({code}) — {platforms} platforms"
//
//   - announce_arrival(&self, train: &Train, platform: u8)
//     Prints: "📢 {code}: {train.name} (#{train.number}) arriving on platform {platform}"

// =============================================
// TODO 5: Define the `Ticket` struct
// =============================================
//
// Fields:
//   - id: u32
//   - train_name: String
//   - from_code: String
//   - to_code: String
//   - price: f64
//   - status: String       (will be "Confirmed", "Waitlisted", or "Cancelled (refund pending)")
//
// Derive: Debug, Clone

// =============================================
// TODO 6: Implement methods for `Ticket`
// =============================================
//
// Associated function:
//   - new(id: u32, train_name: &str, from: &str, to: &str, price: f64, status: &str) -> Self
//
// Methods:
//   - display(&self)
//     Prints: "  Ticket #{id}: {train_name} | {from_code} → {to_code} | ₹{price:.2} | {status}"
//
//   - confirm(&mut self)
//     Sets status to "Confirmed"
//     Prints: "Ticket #{id} confirmed!"
//
//   - cancel(&mut self)
//     Sets status to "Cancelled (refund pending)"
//     Calculates 85% refund: price * 0.85
//     Updates price to the refund amount
//     Prints: "Ticket #{id} cancelled. Refund: ₹{refund:.2}"
//       (where refund is the NEW price after 85% calculation)

// =============================================
// TODO 7: Define the `Passenger` struct
// =============================================
//
// Fields:
//   - name: String
//   - email: String
//   - tickets: Vec<Ticket>
//
// Derive: Debug
//
// Vec<Ticket> is a growable list of Tickets. Think of it like an array
// that can change size. You create an empty one with Vec::new() and
// add items with .push(). We'll cover Vec in detail in Lesson 06.

// =============================================
// TODO 8: Implement methods for `Passenger`
// =============================================
//
// Associated function:
//   - new(name: &str, email: &str) -> Self
//     Creates a passenger with an empty tickets Vec
//
// Methods:
//   - display(&self)
//     Prints: "Passenger: {name} ({email})"
//
//   - add_ticket(&mut self, ticket: Ticket)
//     Pushes a ticket onto self.tickets
//     Hint: self.tickets.push(ticket);
//
//   - get_ticket(&mut self, index: usize) -> &mut Ticket
//     Returns a mutable reference to the ticket at the given index
//     Hint: &mut self.tickets[index]
//
//   - total_spent(&self) -> f64
//     Returns the sum of all ticket prices
//     Hint: use a for loop over &self.tickets and sum the prices
//
//   - ticket_count(&self) -> usize
//     Returns the number of tickets
//     Hint: self.tickets.len()

fn main() {
    println!("=== RAILWAY STATION MANAGEMENT ===");

    // --- Creating Trains ---
    println!("\n--- Creating Trains ---");
    let mut rajdhani = Train::new_express("Rajdhani Express", 12001);
    let shatabdi = Train::new_express("Shatabdi Express", 12002);
    let local = Train::new("Chennai Local", 43210);

    rajdhani.display();
    shatabdi.display();
    local.display();

    // --- Station Info ---
    println!("\n--- Station Info ---");
    let chennai = Station::new("Chennai Central", "MAS", 17);
    let bangalore = Station::new("Bangalore City", "SBC", 10);

    chennai.info();
    bangalore.info();

    // --- Train Operations ---
    println!("\n--- Train Operations ---");
    println!(
        "Rajdhani Express: {} km/h → accelerating...",
        rajdhani.speed_kmh
    );
    rajdhani.accelerate(80.0);
    rajdhani.accelerate(50.0);
    println!("Rajdhani Express is fast: {}", rajdhani.is_fast());
    println!("Chennai Local is fast: {}", local.is_fast());

    // --- Arrivals ---
    println!("\n--- Arrivals ---");
    chennai.announce_arrival(&rajdhani, 1);
    chennai.announce_arrival(&shatabdi, 3);
    bangalore.announce_arrival(&local, 5);

    // --- Passenger & Tickets ---
    println!("\n--- Passenger Manifest ---");
    let mut passenger = Passenger::new("Giridhar Mohan", "giridhar@email.com");

    let ticket1 = Ticket::new(1001, "Rajdhani Express", "MAS", "SBC", 1250.00, "Confirmed");
    let ticket2 = Ticket::new(1002, "Shatabdi Express", "SBC", "MAS", 875.50, "Waitlisted");

    passenger.add_ticket(ticket1);
    passenger.add_ticket(ticket2);

    passenger.display();
    passenger.get_ticket(0).display();
    passenger.get_ticket(1).display();

    // --- Ticket Operations ---
    println!("\n--- Ticket Operations ---");
    passenger.get_ticket(1).confirm();
    passenger.get_ticket(1).display();

    passenger.get_ticket(0).cancel();
    passenger.get_ticket(0).display();

    // --- Summary ---
    println!("\n--- Summary ---");
    println!("Total tickets: {}", passenger.ticket_count());
    println!(
        "Total spent: ₹{:.2} (before cancellations)",
        passenger.total_spent()
    );
}
