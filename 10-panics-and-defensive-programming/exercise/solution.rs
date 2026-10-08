// =============================================
// SOLUTION 09: Railway Ticket Validator
// =============================================
// A crash-proof ticket validator. Zero panics. Zero .unwrap() calls.
// Every function handles errors gracefully via Result or Option.

// =============================================
// STEP 1: Define `TicketClass` enum
// =============================================

#[derive(Debug, Clone)]
enum TicketClass {
    FirstAC,
    SecondAC,
    Sleeper,
    General,
}

// =============================================
// STEP 2: Implement `parse_ticket_class`
// =============================================

fn parse_ticket_class(input: &str) -> Result<TicketClass, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("Ticket class cannot be empty".to_string());
    }
    let lowered = trimmed.to_lowercase();
    match &lowered[..] {
        "1a" | "first" => Ok(TicketClass::FirstAC),
        "2a" | "second" => Ok(TicketClass::SecondAC),
        "sl" | "sleeper" => Ok(TicketClass::Sleeper),
        "gn" | "general" => Ok(TicketClass::General),
        _ => Err(format!("Unknown ticket class: {}", input)),
    }
}

// =============================================
// STEP 3: Define `Ticket` struct
// =============================================

#[derive(Debug, Clone)]
struct Ticket {
    id: u32,
    passenger: String,
    from: String,
    to: String,
    class: TicketClass,
    price: f64,
}

// =============================================
// STEP 4: Implement `Ticket` methods
// =============================================

impl Ticket {
    fn new(id: u32, passenger: &str, from: &str, to: &str, class: TicketClass, price: f64) -> Self {
        Ticket {
            id,
            passenger: passenger.to_string(),
            from: from.to_string(),
            to: to.to_string(),
            class,
            price,
        }
    }

    fn display(&self) -> String {
        format!(
            "Ticket #{}: {} | {} → {} | {:?} | ₹{:.2}",
            self.id, self.passenger, self.from, self.to, self.class, self.price
        )
    }
}

// =============================================
// STEP 5: Implement `validate_ticket`
// =============================================

fn validate_ticket(ticket: &Ticket) -> Result<(), String> {
    if ticket.passenger.is_empty() {
        return Err(format!("Ticket #{}: passenger name is empty", ticket.id));
    }
    if ticket.from == ticket.to {
        return Err(format!(
            "Ticket #{}: origin and destination are the same",
            ticket.id
        ));
    }
    if ticket.price <= 0.0 {
        return Err(format!(
            "Ticket #{}: invalid price ₹{:.2}",
            ticket.id, ticket.price
        ));
    }
    if ticket.price > 10000.0 {
        return Err(format!(
            "Ticket #{}: price ₹{:.2} exceeds maximum",
            ticket.id, ticket.price
        ));
    }
    Ok(())
}

// =============================================
// STEP 6: Implement `find_ticket`
// =============================================

fn find_ticket(tickets: &[Ticket], id: u32) -> Option<&Ticket> {
    tickets.iter().find(|t| t.id == id)
}

// =============================================
// STEP 7: Implement `get_ticket_at`
// =============================================

fn get_ticket_at(tickets: &[Ticket], index: usize) -> Option<&Ticket> {
    tickets.get(index)
}

// =============================================
// STEP 8: Implement `calculate_refund`
// =============================================

fn calculate_refund(price: f64, percent: f64) -> Result<f64, String> {
    if percent < 0.0 || percent > 100.0 {
        return Err(format!("Invalid refund percentage: {}", percent));
    }
    if price < 0.0 {
        return Err(format!("Invalid price: {}", price));
    }
    Ok(price * percent / 100.0)
}

// =============================================
// STEP 9: Implement `process_booking`
// =============================================

fn process_booking(inputs: &[(&str, &str, &str, &str, f64)]) -> (Vec<Ticket>, Vec<String>) {
    let mut valid_tickets: Vec<Ticket> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    for (i, &(passenger, from, to, class_str, price)) in inputs.iter().enumerate() {
        let id = (i as u32) + 1;

        // Step 1: Parse the ticket class — if it fails, collect error and skip
        let class = match parse_ticket_class(class_str) {
            Ok(c) => c,
            Err(e) => {
                errors.push(e);
                continue;
            }
        };

        // Step 2: Create the ticket
        let ticket = Ticket::new(id, passenger, from, to, class, price);

        // Step 3: Validate — if it fails, collect error and skip
        match validate_ticket(&ticket) {
            Ok(()) => valid_tickets.push(ticket),
            Err(e) => errors.push(e),
        }
    }

    (valid_tickets, errors)
}

// =============================================
// STEP 10: Write `fn main()`
// =============================================

fn main() {
    println!("=== RAILWAY TICKET VALIDATOR ===");

    // Define booking inputs (mix of valid and invalid)
    let bookings: Vec<(&str, &str, &str, &str, f64)> = vec![
        ("Giridhar", "Chennai", "Bangalore", "1A", 1250.00), // valid
        ("Priya", "Mumbai", "Delhi", "SL", 450.00),          // valid
        ("", "Chennai", "Mumbai", "2A", 800.00),             // empty passenger
        ("Arjun", "Bangalore", "Bangalore", "GN", 200.00),   // same from/to
        ("Meera", "Delhi", "Chennai", "XYZ", 500.00),        // invalid class
        ("Ravi", "Chennai", "Kolkata", "1A", -100.00),       // negative price
        ("Suresh", "Mumbai", "Chennai", "general", 350.00),  // valid
        ("Anita", "Chennai", "Delhi", "first", 15000.00),    // price too high
    ];

    let (valid_tickets, errors) = process_booking(&bookings);

    // Print booking errors
    println!("\n--- Booking Errors ---");
    for error in &errors {
        println!("  {}", error);
    }

    // Print valid tickets
    println!("\n--- Valid Tickets ---");
    for ticket in &valid_tickets {
        println!("  {}", ticket.display());
    }

    // Safe lookups
    println!("\n--- Safe Lookups ---");

    match find_ticket(&valid_tickets, 1) {
        Some(t) => println!(
            "  Ticket #1: Found — {} | {} → {}",
            t.passenger, t.from, t.to
        ),
        None => println!("  Ticket #1: Not found"),
    }

    match find_ticket(&valid_tickets, 99) {
        Some(t) => println!(
            "  Ticket #99: Found — {} | {} → {}",
            t.passenger, t.from, t.to
        ),
        None => println!("  Ticket #99: Not found"),
    }

    match get_ticket_at(&valid_tickets, 0) {
        Some(t) => println!("  Index 0: Found — {} | {} → {}", t.passenger, t.from, t.to),
        None => println!("  Index 0: Not found (safe — no panic!)"),
    }

    match get_ticket_at(&valid_tickets, 99) {
        Some(t) => println!(
            "  Index 99: Found — {} | {} → {}",
            t.passenger, t.from, t.to
        ),
        None => println!("  Index 99: Not found (safe — no panic!)"),
    }

    // Safe refund calculations
    println!("\n--- Refund Calculations ---");

    match calculate_refund(1250.0, 85.0) {
        Ok(amount) => println!("  85% refund on ₹1250.00: ₹{:.2}", amount),
        Err(e) => println!("  85% refund on ₹1250.00: Error — {}", e),
    }

    match calculate_refund(1250.0, 150.0) {
        Ok(amount) => println!("  150% refund: ₹{:.2}", amount),
        Err(e) => println!("  150% refund: Error — {}", e),
    }

    match calculate_refund(-500.0, 50.0) {
        Ok(amount) => println!("  Refund on ₹-500.00: ₹{:.2}", amount),
        Err(e) => println!("  Refund on ₹-500.00: Error — {}", e),
    }
}
