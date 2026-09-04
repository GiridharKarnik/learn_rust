// =============================================
// SOLUTION — Don't peek until you've tried!
// =============================================

// TODO 1
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

// TODO 2
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

// TODO 3
impl Event {
    fn urgency(&self) -> Urgency {
        match self {
            Event::Cancellation { .. } => Urgency::Critical,
            Event::Delay { minutes, .. } if *minutes > 30 => Urgency::Urgent,
            Event::Delay { .. } => Urgency::Minor,
            Event::PlatformChange { .. } => Urgency::Urgent,
            Event::Departure { .. } | Event::Arrival { .. } => Urgency::Info,
        }
    }

    fn announce(&self) -> String {
        match self {
            Event::Departure {
                train,
                platform,
                destination,
            } => {
                format!("{train} to {destination} departing from platform {platform}")
            }
            Event::Arrival {
                train,
                platform,
                origin,
            } => {
                format!("{train} from {origin} arriving at platform {platform}")
            }
            Event::Delay {
                train,
                minutes,
                reason,
            } => {
                format!("{train} delayed by {minutes} min \u{2014} {reason}")
            }
            Event::Cancellation { train, reason } => {
                format!("CANCELLED: {train} \u{2014} {reason}")
            }
            Event::PlatformChange {
                train,
                old_platform,
                new_platform,
            } => {
                format!("{train}: platform changed from {old_platform} to {new_platform}")
            }
        }
    }

    fn train_name(&self) -> &str {
        match self {
            Event::Departure { train, .. }
            | Event::Arrival { train, .. }
            | Event::Delay { train, .. }
            | Event::Cancellation { train, .. }
            | Event::PlatformChange { train, .. } => train,
        }
    }
}

// TODO 4
#[derive(Debug)]
struct AnnouncementBoard {
    station: String,
    events: Vec<Event>,
}

impl AnnouncementBoard {
    fn new(station: &str) -> Self {
        AnnouncementBoard {
            station: station.to_string(),
            events: Vec::new(),
        }
    }

    fn add(&mut self, event: Event) {
        self.events.push(event);
    }

    fn display_all(&self) {
        println!("--- All Announcements ---");
        for event in &self.events {
            println!("{} {}", event.urgency().prefix(), event.announce());
        }
    }

    fn display_urgent(&self) {
        println!("--- Urgent & Critical Only ---");
        for event in &self.events {
            let urgency = event.urgency();
            if urgency == Urgency::Urgent || urgency == Urgency::Critical {
                println!("{} {}", urgency.prefix(), event.announce());
            }
        }
    }

    fn summary(&self) {
        let mut departures = 0;
        let mut arrivals = 0;
        let mut delays = 0;
        let mut cancellations = 0;
        let mut platform_changes = 0;

        for event in &self.events {
            match event {
                Event::Departure { .. } => departures += 1,
                Event::Arrival { .. } => arrivals += 1,
                Event::Delay { .. } => delays += 1,
                Event::Cancellation { .. } => cancellations += 1,
                Event::PlatformChange { .. } => platform_changes += 1,
            }
        }

        let total = self.events.len();
        println!("--- Event Summary ---");
        println!("Departures: {departures}");
        println!("Arrivals: {arrivals}");
        println!("Delays: {delays}");
        println!("Cancellations: {cancellations}");
        println!("Platform changes: {platform_changes}");
        println!("Total: {total}");
    }
}

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
