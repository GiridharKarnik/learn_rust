// ============================================================================
// LESSON 05 EXERCISE — SOLUTION: Railway Event Processing System
// ============================================================================

// TODO 1: Define the `CoachClass` enum
#[derive(Debug, Clone, PartialEq)]
enum CoachClass {
    FirstAC,
    SecondAC,
    ThirdAC,
    Sleeper,
    General,
}

// TODO 2: Implement methods on `CoachClass`
impl CoachClass {
    fn price_per_km(&self) -> f64 {
        match self {
            CoachClass::FirstAC => 4.50,
            CoachClass::SecondAC => 2.75,
            CoachClass::ThirdAC => 1.80,
            CoachClass::Sleeper => 0.75,
            CoachClass::General => 0.45,
        }
    }

    fn display_name(&self) -> &str {
        match self {
            CoachClass::FirstAC => "First AC",
            CoachClass::SecondAC => "Second AC",
            CoachClass::ThirdAC => "Third AC",
            CoachClass::Sleeper => "Sleeper",
            CoachClass::General => "General",
        }
    }
}

// TODO 3: Define the `BookingStatus` enum (with associated data!)
#[derive(Debug, Clone)]
enum BookingStatus {
    Confirmed { coach: String, seat: u32 },
    Waitlisted(u32),
    Cancelled { refund_amount: f64 },
    RAC(u32),
}

// TODO 4: Implement methods on `BookingStatus`
impl BookingStatus {
    fn description(&self) -> String {
        match self {
            BookingStatus::Confirmed { coach, seat } => {
                format!("Confirmed \u{2014} Coach {}, Seat {}", coach, seat)
            }
            BookingStatus::Waitlisted(pos) => {
                format!("Waitlisted \u{2014} Position {}", pos)
            }
            BookingStatus::Cancelled { refund_amount } => {
                format!("Cancelled \u{2014} Refund \u{20b9}{:.2}", refund_amount)
            }
            BookingStatus::RAC(seat) => {
                format!("RAC \u{2014} Seat {}", seat)
            }
        }
    }

    fn is_confirmed(&self) -> bool {
        matches!(self, BookingStatus::Confirmed { .. })
    }

    fn emoji(&self) -> &str {
        match self {
            BookingStatus::Confirmed { .. } => "\u{2705}",
            BookingStatus::Waitlisted(_) => "\u{23f3}",
            BookingStatus::Cancelled { .. } => "\u{274c}",
            BookingStatus::RAC(_) => "\u{1f504}",
        }
    }
}

// TODO 5: Define the `Booking` struct
#[derive(Debug)]
struct Booking {
    id: u32,
    passenger: String,
    train_name: String,
    from: String,
    to: String,
    distance_km: f64,
    class: CoachClass,
    status: BookingStatus,
}

// TODO 6: Implement methods on `Booking`
impl Booking {
    fn new(
        id: u32,
        passenger: &str,
        train_name: &str,
        from: &str,
        to: &str,
        distance_km: f64,
        class: CoachClass,
        status: BookingStatus,
    ) -> Self {
        Booking {
            id,
            passenger: passenger.to_string(),
            train_name: train_name.to_string(),
            from: from.to_string(),
            to: to.to_string(),
            distance_km,
            class,
            status,
        }
    }

    fn fare(&self) -> f64 {
        self.distance_km * self.class.price_per_km()
    }

    fn display(&self) {
        println!(
            "[{}] Booking #{}: {}",
            self.status.emoji(),
            self.id,
            self.passenger
        );
        println!(
            "     {} | {} \u{2192} {} | {} km",
            self.train_name, self.from, self.to, self.distance_km as u64
        );
        println!(
            "     Class: {} | Fare: \u{20b9}{:.2}",
            self.class.display_name(),
            self.fare()
        );
        println!("     Status: {}", self.status.description());
    }

    fn cancel(&mut self) {
        let refund = self.fare() * 0.85;
        self.status = BookingStatus::Cancelled {
            refund_amount: refund,
        };
    }

    fn confirm(&mut self, coach: &str, seat: u32) {
        self.status = BookingStatus::Confirmed {
            coach: coach.to_string(),
            seat,
        };
    }
}

// TODO 7: Implement the `find_booking` function
fn find_booking(bookings: &[Booking], id: u32) -> Option<&Booking> {
    bookings.iter().find(|b| b.id == id)
}

// TODO 8: Implement the `parse_coach_class` function
fn parse_coach_class(input: &str) -> Result<CoachClass, String> {
    match input.to_lowercase().as_str() {
        "1a" | "first" => Ok(CoachClass::FirstAC),
        "2a" | "second" => Ok(CoachClass::SecondAC),
        "3a" | "third" => Ok(CoachClass::ThirdAC),
        "sl" | "sleeper" => Ok(CoachClass::Sleeper),
        "gn" | "general" => Ok(CoachClass::General),
        _ => Err(format!("Unknown coach class: {}", input)),
    }
}

// TODO 9: Implement the `summarize_bookings` function
fn summarize_bookings(bookings: &[Booking]) {
    let mut confirmed = 0;
    let mut waitlisted = 0;
    let mut rac = 0;
    let mut cancelled = 0;

    for booking in bookings {
        match &booking.status {
            BookingStatus::Confirmed { .. } => confirmed += 1,
            BookingStatus::Waitlisted(_) => waitlisted += 1,
            BookingStatus::RAC(_) => rac += 1,
            BookingStatus::Cancelled { .. } => cancelled += 1,
        }
    }

    println!("--- Booking Summary ---");
    println!("Total: {}", bookings.len());
    println!("Confirmed: {}", confirmed);
    println!("Waitlisted: {}", waitlisted);
    println!("RAC: {}", rac);
    println!("Cancelled: {}", cancelled);
}

// ============================================================================
// main() — identical to the exercise file
// ============================================================================

fn main() {
    println!("=== RAILWAY BOOKING SYSTEM ===");

    // --- Create bookings ---
    println!("\n--- All Bookings ---");
    let mut bookings = vec![
        Booking::new(
            1001,
            "Giridhar",
            "Rajdhani Express",
            "MAS",
            "SBC",
            350.0,
            CoachClass::FirstAC,
            BookingStatus::Confirmed {
                coach: String::from("A1"),
                seat: 23,
            },
        ),
        Booking::new(
            1002,
            "Priya",
            "Shatabdi Express",
            "SBC",
            "MAS",
            350.0,
            CoachClass::SecondAC,
            BookingStatus::Waitlisted(3),
        ),
        Booking::new(
            1003,
            "Arjun",
            "Duronto Express",
            "MAS",
            "NDLS",
            2200.0,
            CoachClass::ThirdAC,
            BookingStatus::RAC(42),
        ),
        Booking::new(
            1004,
            "Meera",
            "Garib Rath",
            "MAS",
            "HWH",
            1650.0,
            CoachClass::Sleeper,
            BookingStatus::Confirmed {
                coach: String::from("S4"),
                seat: 15,
            },
        ),
        Booking::new(
            1005,
            "Ravi",
            "Chennai Local",
            "MAS",
            "TBM",
            25.0,
            CoachClass::General,
            BookingStatus::Waitlisted(7),
        ),
    ];

    for booking in &bookings {
        booking.display();
        println!();
    }

    // --- Find a booking using Option ---
    println!("--- Find Booking ---");
    match find_booking(&bookings, 1003) {
        Some(b) => println!("Found: Booking #{} for {}", b.id, b.passenger),
        None => println!("Booking not found"),
    }
    match find_booking(&bookings, 9999) {
        Some(b) => println!("Found: Booking #{} for {}", b.id, b.passenger),
        None => println!("Booking #9999 not found"),
    }

    // --- if let ---
    println!("\n--- Quick Lookup (if let) ---");
    if let Some(booking) = find_booking(&bookings, 1001) {
        println!(
            "Giridhar's booking: {} class on {}",
            booking.class.display_name(),
            booking.train_name
        );
    }

    // --- Parse coach class using Result ---
    println!("\n--- Parse Coach Class ---");
    let inputs = ["1A", "SL", "general", "2A", "XYZ", "first"];
    for input in inputs {
        match parse_coach_class(input) {
            Ok(class) => println!(
                "  '{}' \u{2192} {} (\u{20b9}{:.2}/km)",
                input,
                class.display_name(),
                class.price_per_km()
            ),
            Err(e) => println!("  '{}' \u{2192} Error: {}", input, e),
        }
    }

    // --- Modify bookings ---
    println!("\n--- Booking Updates ---");

    // Confirm waitlisted booking
    bookings[1].confirm("B2", 17);
    println!("Updated booking #1002:");
    bookings[1].display();

    // Cancel a booking
    println!();
    bookings[4].cancel();
    println!("Updated booking #1005:");
    bookings[4].display();

    // --- Summary ---
    println!();
    summarize_bookings(&bookings);
}
