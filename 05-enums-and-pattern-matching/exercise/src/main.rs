// =============================================
// EXERCISE 05: Station Announcement System
// =============================================
//
// You're building the automated announcement system for Chennai Central station.
// Different events need different announcements with different urgency levels.
//
// Run with: cargo run
//
// Expected output:
//
//   === CHENNAI CENTRAL ANNOUNCEMENTS ===
//
//   --- All Announcements ---
//   🟢 [INFO] Rajdhani Express to New Delhi departing from platform 3
//   🟢 [INFO] Shatabdi Express from Bangalore arriving at platform 7
//   🟠 [URGENT] Duronto Express delayed by 45 min — Signal failure near Katpadi
//   🔴 [CRITICAL] CANCELLED: Nilgiri Express — Landslide on mountain track
//   🟠 [URGENT] Garib Rath: platform changed from 5 to 2
//   🟢 [INFO] Kovai Express to Coimbatore departing from platform 12
//   🔴 [CRITICAL] CANCELLED: Island Express — Flooding in Kerala
//   🟡 [MINOR] Mumbai Mail delayed by 10 min — Late arrival of incoming rake
//
//   --- Urgent & Critical Only ---
//   🟠 [URGENT] Duronto Express delayed by 45 min — Signal failure near Katpadi
//   🔴 [CRITICAL] CANCELLED: Nilgiri Express — Landslide on mountain track
//   🟠 [URGENT] Garib Rath: platform changed from 5 to 2
//   🔴 [CRITICAL] CANCELLED: Island Express — Flooding in Kerala
//
//   --- Event Summary ---
//   Departures: 2
//   Arrivals: 1
//   Delays: 2
//   Cancellations: 2
//   Platform changes: 1
//   Total: 8

// =============================================
// TODO 1: Define the `Urgency` enum and its method
// =============================================
//
// Variants: Info, Minor, Urgent, Critical
//
// Derive: Debug, Clone, PartialEq
//
// Then implement ONE method:
//
//   fn prefix(&self) -> &str
//     Returns a colored tag used at the start of each announcement:
//       Info     → "🟢 [INFO]"
//       Minor    → "🟡 [MINOR]"
//       Urgent   → "🟠 [URGENT]"
//       Critical → "🔴 [CRITICAL]"
//
// WHY: Every announcement needs a visual urgency tag.
//      This is how `match` works on simple enums — variant in, value out.
#[derive(Debug, Clone, PartialEq)]
enum Urgency {
    Info,
    Minor,
    Urgent,
    Critical,
}

impl Urgency {
    fn prefix(&self) -> &str {
        match self {
            Urgency::Info => "🟢 [INFO]",
            Urgency::Minor => "🟡 [MINOR]",
            Urgency::Urgent => "🟠 [URGENT]",
            Urgency::Critical => "🔴 [CRITICAL]",
        }
    }
}

// =============================================
// TODO 2: Define the `Event` enum (with data!)
// =============================================
//
// Each variant carries the data specific to that event type.
// Notice how different events need different information:
//
//   Departure { train: String, platform: u8, destination: String }
//   Arrival { train: String, platform: u8, origin: String }
//   Delay { train: String, minutes: u32, reason: String }
//   Cancellation { train: String, reason: String }
//   PlatformChange { train: String, old_platform: u8, new_platform: u8 }
//
// Derive: Debug, Clone
//
// WHY: This is the core of "enums with data" — each variant has a
//      different shape. A Departure has a destination but no reason.
//      A Cancellation has a reason but no platform. Rust enforces that
//      you handle each shape correctly.
#[derive(Debug, Clone)]
enum Event {
    Departure {
        train: String,
        platform: u8,
        destination: String,
    },
    Arrival {
        train: String,
        platform: u8,
        origin: String,
    },
    Delay {
        train: String,
        minutes: u32,
        reason: String,
    },
    Cancellation {
        train: String,
        reason: String,
    },
    PlatformChange {
        train: String,
        old_platform: u8,
        new_platform: u8,
    },
}

// =============================================
// TODO 3: Implement `Event` methods
// =============================================
//
// Three methods, each exercises a different match skill:
//
// (a) fn urgency(&self) -> Urgency
//     Decides how urgent this event is. Rules:
//       Cancellation                      → Critical
//       Delay where minutes > 30          → Urgent    ← match guard!
//       Delay where minutes <= 30         → Minor
//       PlatformChange                    → Urgent
//       Departure, Arrival                → Info
//
//     WHY: This exercises match GUARDS — the same variant (Delay)
//          maps to different urgencies based on the data inside it.
//
// (b) fn announce(&self) -> String
//     Generates the announcement text. Each variant formats differently:
//       Departure       → "{train} to {destination} departing from platform {platform}"
//       Arrival         → "{train} from {origin} arriving at platform {platform}"
//       Delay           → "{train} delayed by {minutes} min — {reason}"
//       Cancellation    → "CANCELLED: {train} — {reason}"
//       PlatformChange  → "{train}: platform changed from {old_platform} to {new_platform}"
//
//     WHY: This exercises DESTRUCTURING — pulling data out of each
//          variant and using it to build a string.
//
// (c) fn train_name(&self) -> &str
//     Returns the train name from any event. Every variant has a `train` field.
//       Hint: match on self, extract `train` in each arm, return train.
//             You can combine arms with | if the field name is the same.
//
//     WHY: Sometimes you need the same field from every variant.
//          This shows that you always have to match even when every
//          arm does the same thing.
impl Event {
    fn urgency(&self) -> Urgency {
        match self {
            Event::Arrival { .. } | Event::Departure { .. } => Urgency::Info,
            Event::Cancellation { .. } => Urgency::Critical,
            Event::PlatformChange { .. } => Urgency::Urgent,
            Event::Delay { minutes, .. } => {
                if *minutes > 30 {
                    return Urgency::Urgent;
                } else {
                    return Urgency::Minor;
                }
            }
        }
    }

    fn announce(&self) -> String {
        match self {
            Event::Arrival {
                train,
                platform,
                origin,
            } => {
                return format!("{train} from {origin} arriving at platform {platform}");
            }
            Event::Departure {
                train,
                platform,
                destination,
            } => {
                return format!("{train} to {destination} departing from platform {platform}");
            }
            Event::Delay {
                train,
                minutes,
                reason,
            } => {
                return format!("{train} delayed by {minutes} min — {reason}");
            }
            Event::Cancellation { train, reason } => {
                return format!("CANCELLED: {train} — {reason}");
            }
            Event::PlatformChange {
                train,
                old_platform,
                new_platform,
            } => {
                return format!("{train}: platform changed from {old_platform} to {new_platform}");
            }
        }
    }

    fn train_name(&self) -> &str {
        match self {
            Event::Arrival { train, .. } => train,
            Event::Departure { train, .. } => train,
            Event::Delay { train, .. } => train,
            Event::Cancellation { train, .. } => train,
            Event::PlatformChange { train, .. } => train,
        }
    }
}

// =============================================
// TODO 4: Define `AnnouncementBoard` struct and methods
// =============================================
//
// Fields:
//   station: String
//   events: Vec<Event>
//
// Derive: Debug
//
// Implement these methods:
//
//   fn new(station: &str) -> Self
//     Creates a board with an empty events Vec.
//
//   fn add(&mut self, event: Event)
//     Pushes an event onto the Vec.
//
//   fn display_all(&self)
//     Prints "--- All Announcements ---"
//     Then for each event, prints one line:
//       "{urgency_prefix} {announcement}"
//     Example: "🔴 [CRITICAL] CANCELLED: Nilgiri Express — Landslide"
//
//   fn display_urgent(&self)
//     Prints "--- Urgent & Critical Only ---"
//     Same format, but only prints events where urgency is Urgent or Critical.
//     Hint: compare with == (that's why Urgency derives PartialEq)
//
//   fn summary(&self)
//     Counts how many of each event type using match, then prints:
//       --- Event Summary ---
//       Departures: 2
//       Arrivals: 1
//       Delays: 2
//       Cancellations: 2
//       Platform changes: 1
//       Total: 8
//
//     Hint: use `..` to ignore fields: Event::Departure { .. } => departures += 1,
struct AnnouncementBoard {
    station: String,
    events: Vec<Event>,
}

impl AnnouncementBoard {
    fn new(station: &str) -> Self {
        return AnnouncementBoard {
            station: String::from(station),
            events: Vec::new(),
        };
    }

    fn add(&mut self, event: Event) {
        self.events.push(event);
    }

    fn display_all(&self) {
        println!("--- All Announcements ---");

        for event in &self.events {
            println!("{} {}", event.urgency().prefix(), event.announce())
        }
    }

    fn display_urgent(&self) {
        println!("--- Urgent & Critical Only ---");

        for event in &self.events {
            match event.urgency() {
                Urgency::Urgent | Urgency::Critical => {
                    println!("{} {}", event.urgency().prefix(), event.announce())
                }
                _ => {}
            }
        }
    }

    fn summary(&self) {
        let mut departures_count = 0;
        let mut arrivals_count = 0;
        let mut delays_count = 0;
        let mut cancellations_count = 0;
        let mut platform_changes_count = 0;

        for event in &self.events {
            match event {
                Event::Arrival { .. } => arrivals_count += 1,
                Event::Departure { .. } => departures_count += 1,
                Event::Delay { .. } => delays_count += 1,
                Event::Cancellation { .. } => cancellations_count += 1,
                Event::PlatformChange { .. } => platform_changes_count += 1,
            }
        }

        println!("--- Event Summary ---");
        println!("Departures: {}", departures_count);
        println!("Arrivals: {}", arrivals_count);
        println!("Delays: {}", delays_count);
        println!("Cancellations: {}", cancellations_count);
        println!("Platform changes: {}", platform_changes_count);
        println!("Total: {}", &self.events.len());
    }
}

// =============================================
// main() — DO NOT EDIT BELOW THIS LINE
// =============================================

fn main() {
    println!("=== CHENNAI CENTRAL ANNOUNCEMENTS ===");

    let mut board = AnnouncementBoard::new("Chennai Central");

    board.add(Event::Departure {
        train: String::from("Rajdhani Express"),
        platform: 3,
        destination: String::from("New Delhi"),
    });

    board.add(Event::Arrival {
        train: String::from("Shatabdi Express"),
        platform: 7,
        origin: String::from("Bangalore"),
    });

    board.add(Event::Delay {
        train: String::from("Duronto Express"),
        minutes: 45,
        reason: String::from("Signal failure near Katpadi"),
    });

    board.add(Event::Cancellation {
        train: String::from("Nilgiri Express"),
        reason: String::from("Landslide on mountain track"),
    });

    board.add(Event::PlatformChange {
        train: String::from("Garib Rath"),
        old_platform: 5,
        new_platform: 2,
    });

    board.add(Event::Departure {
        train: String::from("Kovai Express"),
        platform: 12,
        destination: String::from("Coimbatore"),
    });

    board.add(Event::Cancellation {
        train: String::from("Island Express"),
        reason: String::from("Flooding in Kerala"),
    });

    board.add(Event::Delay {
        train: String::from("Mumbai Mail"),
        minutes: 10,
        reason: String::from("Late arrival of incoming rake"),
    });

    println!();
    board.display_all();

    println!();
    board.display_urgent();

    println!();
    board.summary();
}
