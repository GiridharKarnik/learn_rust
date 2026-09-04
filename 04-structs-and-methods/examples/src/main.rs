// =============================================
// LESSON 04: Structs & Methods — Live Examples
// =============================================

// --- Derive common traits ---
// Debug  → lets us print with {:?}
// Clone  → lets us call .clone()
#[derive(Debug, Clone)]
struct Train {
    name: String,
    number: u32,
    speed_kmh: f64,
    is_express: bool,
}

// --- Methods and associated functions ---
impl Train {
    // Associated function (constructor) — no self parameter
    // Called with Train::new(...)
    fn new(name: &str, number: u32) -> Self {
        Self {
            name: String::from(name),
            number,
            speed_kmh: 0.0,
            is_express: false,
        }
    }

    // Another constructor — for express trains
    fn new_express(name: &str, number: u32, speed: f64) -> Self {
        Self {
            name: String::from(name),
            number,
            speed_kmh: speed,
            is_express: true,
        }
    }

    // Method: borrows self immutably — just reads data
    fn display(&self) {
        let express_tag = if self.is_express { " [EXPRESS]" } else { "" };
        println!(
            "  #{} {}{} — {} km/h",
            self.number, self.name, express_tag, self.speed_kmh
        );
    }

    // Method: returns a value based on the struct's data
    fn is_fast(&self) -> bool {
        self.speed_kmh > 100.0
    }

    // Method: borrows self mutably — modifies the struct
    fn accelerate(&mut self, amount: f64) {
        self.speed_kmh += amount;
        if self.speed_kmh > 200.0 {
            self.speed_kmh = 200.0; // cap at 200
        }
        println!("  {} accelerated to {} km/h", self.name, self.speed_kmh);
    }

    // Method: borrows self mutably
    fn stop(&mut self) {
        self.speed_kmh = 0.0;
        println!("  {} has stopped", self.name);
    }

    // Method: consumes self — the Train is gone after this
    fn scrap(self) -> String {
        println!("  #{} {} has been scrapped!", self.number, self.name);
        format!("Scrapped: {}", self.name)
        // self is dropped here — the Train no longer exists
    }
}

// --- A second struct ---
#[derive(Debug)]
struct Station {
    name: String,
    code: String,
    platforms: u8,
}

impl Station {
    fn new(name: &str, code: &str, platforms: u8) -> Self {
        Self {
            name: String::from(name),
            code: String::from(code),
            platforms,
        }
    }

    fn announce_arrival(&self, train: &Train) {
        println!(
            "  📢 {} (#{}) arriving at {} [{}]",
            train.name, train.number, self.name, self.code
        );
    }
}

// --- Tuple struct ---
#[derive(Debug)]
struct Kilometers(f64);

#[derive(Debug)]
struct Rupees(f64);

fn calculate_fare(distance: &Kilometers) -> Rupees {
    Rupees(distance.0 * 0.75)
}

fn main() {
    // =============================================
    // 1. Creating structs
    // =============================================
    println!("--- 1. Creating structs ---");

    // Using the constructor
    let mut rajdhani = Train::new_express("Rajdhani Express", 12001, 130.0);
    let shatabdi = Train::new_express("Shatabdi Express", 12002, 150.0);
    let local = Train::new("Chennai Local", 43210);

    rajdhani.display();
    shatabdi.display();
    local.display();

    // =============================================
    // 2. Accessing fields
    // =============================================
    println!("\n--- 2. Accessing fields ---");
    println!("Train name: {}", rajdhani.name);
    println!("Train number: {}", rajdhani.number);
    println!("Is express? {}", rajdhani.is_express);

    // =============================================
    // 3. Mutating fields
    // =============================================
    println!("\n--- 3. Mutating fields ---");
    println!("Speed before: {} km/h", rajdhani.speed_kmh);
    rajdhani.speed_kmh = 140.0;
    println!("Speed after:  {} km/h", rajdhani.speed_kmh);

    // =============================================
    // 4. Methods that read (&self)
    // =============================================
    println!("\n--- 4. Methods that read ---");
    println!("Is Rajdhani fast? {}", rajdhani.is_fast());
    println!("Is Local fast? {}", local.is_fast());

    // =============================================
    // 5. Methods that modify (&mut self)
    // =============================================
    println!("\n--- 5. Methods that modify ---");
    rajdhani.accelerate(30.0);
    rajdhani.accelerate(50.0); // will be capped at 200
    rajdhani.stop();

    // =============================================
    // 6. Debug printing
    // =============================================
    println!("\n--- 6. Debug printing ---");
    println!("Debug:  {:?}", local);
    println!("Pretty: {:#?}", local);

    // =============================================
    // 7. Clone
    // =============================================
    println!("\n--- 7. Clone ---");
    let rajdhani_copy = rajdhani.clone();
    println!("Original: {:?}", rajdhani);
    println!("Copy:     {:?}", rajdhani_copy);

    // =============================================
    // 8. Field init shorthand
    // =============================================
    println!("\n--- 8. Field init shorthand ---");
    let name = String::from("Duronto Express");
    let number = 12003;
    let speed_kmh = 120.0;
    let is_express = true;

    let duronto = Train {
        name, // shorthand — same as name: name
        number,
        speed_kmh,
        is_express,
    };
    duronto.display();

    // =============================================
    // 9. Struct update syntax
    // =============================================
    println!("\n--- 9. Struct update syntax ---");
    let duronto_slow = Train {
        name: String::from("Duronto Slow"),
        speed_kmh: 60.0,
        ..duronto // copy remaining fields from duronto
                  // ⚠️ duronto.name was moved (but we overrode it, so it's fine)
                  // duronto.number and duronto.is_express were copied (they're Copy types)
    };
    duronto_slow.display();

    // =============================================
    // 10. Structs interacting
    // =============================================
    println!("\n--- 10. Structs interacting ---");
    let chennai = Station::new("Chennai Central", "MAS", 17);
    let bangalore = Station::new("Bangalore City", "SBC", 10);

    chennai.announce_arrival(&rajdhani);
    bangalore.announce_arrival(&shatabdi);

    println!("  {} has {} platforms", chennai.name, chennai.platforms);

    // =============================================
    // 11. Tuple structs for type safety
    // =============================================
    println!("\n--- 11. Tuple structs ---");
    let distance = Kilometers(350.0);
    let fare = calculate_fare(&distance);
    println!("Distance: {:.0} km → Fare: ₹{:.2}", distance.0, fare.0);

    // =============================================
    // 12. Consuming method (self)
    // =============================================
    println!("\n--- 12. Consuming method ---");
    let old_train = Train::new("Steam Engine", 1);
    let result = old_train.scrap(); // old_train is consumed here
    println!("  {result}");
    // println!("{}", old_train.name);  // ❌ would fail — old_train is gone
}
