// =============================================
// SOLUTION — Don't peek until you've tried!
// =============================================

// TODO 1
#[derive(Debug, Clone)]
struct Train {
    name: String,
    number: u32,
    speed_kmh: f64,
    is_express: bool,
}

// TODO 2
impl Train {
    fn new(name: &str, number: u32) -> Self {
        Self {
            name: String::from(name),
            number,
            speed_kmh: 0.0,
            is_express: false,
        }
    }

    fn new_express(name: &str, number: u32) -> Self {
        Self {
            name: String::from(name),
            number,
            speed_kmh: 0.0,
            is_express: true,
        }
    }

    fn display(&self) {
        let tag = if self.is_express {
            "Express"
        } else {
            "Regular"
        };
        println!("Created: {} (#{}) [{tag}]", self.name, self.number);
    }

    fn accelerate(&mut self, amount: f64) {
        self.speed_kmh += amount;
        if self.speed_kmh > 200.0 {
            self.speed_kmh = 200.0;
        }
        println!("{}: now at {} km/h", self.name, self.speed_kmh);
    }

    fn is_fast(&self) -> bool {
        self.speed_kmh > 100.0
    }
}

// TODO 3
#[derive(Debug)]
struct Station {
    name: String,
    code: String,
    platforms: u8,
}

// TODO 4
impl Station {
    fn new(name: &str, code: &str, platforms: u8) -> Self {
        Self {
            name: String::from(name),
            code: String::from(code),
            platforms,
        }
    }

    fn info(&self) {
        println!(
            "{} ({}) \u{2014} {} platforms",
            self.name, self.code, self.platforms
        );
    }

    fn announce_arrival(&self, train: &Train, platform: u8) {
        println!(
            "\u{1f4e2} {}: {} (#{}) arriving on platform {}",
            self.code, train.name, train.number, platform
        );
    }
}

// TODO 5
#[derive(Debug, Clone)]
struct Ticket {
    id: u32,
    train_name: String,
    from_code: String,
    to_code: String,
    price: f64,
    status: String,
}

// TODO 6
impl Ticket {
    fn new(id: u32, train_name: &str, from: &str, to: &str, price: f64, status: &str) -> Self {
        Self {
            id,
            train_name: String::from(train_name),
            from_code: String::from(from),
            to_code: String::from(to),
            price,
            status: String::from(status),
        }
    }

    fn display(&self) {
        println!(
            "  Ticket #{}: {} | {} \u{2192} {} | \u{20b9}{:.2} | {}",
            self.id, self.train_name, self.from_code, self.to_code, self.price, self.status
        );
    }

    fn confirm(&mut self) {
        self.status = String::from("Confirmed");
        println!("Ticket #{} confirmed!", self.id);
    }

    fn cancel(&mut self) {
        self.status = String::from("Cancelled (refund pending)");
        self.price = self.price * 0.85;
        println!(
            "Ticket #{} cancelled. Refund: \u{20b9}{:.2}",
            self.id, self.price
        );
    }
}

// TODO 7
#[derive(Debug)]
struct Passenger {
    name: String,
    email: String,
    tickets: Vec<Ticket>,
}

// TODO 8
impl Passenger {
    fn new(name: &str, email: &str) -> Self {
        Self {
            name: String::from(name),
            email: String::from(email),
            tickets: Vec::new(),
        }
    }

    fn display(&self) {
        println!("Passenger: {} ({})", self.name, self.email);
    }

    fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }

    fn get_ticket(&mut self, index: usize) -> &mut Ticket {
        &mut self.tickets[index]
    }

    fn total_spent(&self) -> f64 {
        let mut total = 0.0;
        for ticket in &self.tickets {
            total += ticket.price;
        }
        total
    }

    fn ticket_count(&self) -> usize {
        self.tickets.len()
    }
}

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
        "Rajdhani Express: {} km/h \u{2192} accelerating...",
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
        "Total spent: \u{20b9}{:.2} (before cancellations)",
        passenger.total_spent()
    );
}
