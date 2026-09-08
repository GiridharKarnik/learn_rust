// =============================================
// EXERCISE 05: Station Announcement System
// =============================================
//
// Build an automated announcement system for Chennai Central station.
// You write EVERYTHING from scratch — enums, structs, impl blocks, and main().
//
// Run with: cargo run
//
// =============================================
// EXPECTED OUTPUT:
// =============================================
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
// STEP 1: Define the `Urgency` enum + method
// =============================================
//
// Variants: Info, Minor, Urgent, Critical
//
// Derive: Debug, Clone, PartialEq
//
// Implement ONE method:
//
//   fn prefix(&self) -> &str
//     Returns a colored tag for each urgency level:
//       Info     → "🟢 [INFO]"
//       Minor    → "🟡 [MINOR]"
//       Urgent   → "🟠 [URGENT]"
//       Critical → "🔴 [CRITICAL]"

// =============================================
// STEP 2: Define the `Event` enum (with data!)
// =============================================
//
// Each variant carries different data:
//
//   Departure { train: String, platform: u8, destination: String }
//   Arrival { train: String, platform: u8, origin: String }
//   Delay { train: String, minutes: u32, reason: String }
//   Cancellation { train: String, reason: String }
//   PlatformChange { train: String, old_platform: u8, new_platform: u8 }
//
// Derive: Debug, Clone

// =============================================
// STEP 3: Implement `Event` methods
// =============================================
//
// Three methods:
//
// (a) fn urgency(&self) -> Urgency
//     Rules:
//       Cancellation                      → Critical
//       Delay where minutes > 30          → Urgent     (use a match guard!)
//       Delay where minutes <= 30         → Minor
//       PlatformChange                    → Urgent
//       Departure | Arrival               → Info
//
// (b) fn announce(&self) -> String
//     Format varies by variant:
//       Departure      → "{train} to {destination} departing from platform {platform}"
//       Arrival        → "{train} from {origin} arriving at platform {platform}"
//       Delay          → "{train} delayed by {minutes} min — {reason}"
//       Cancellation   → "CANCELLED: {train} — {reason}"
//       PlatformChange → "{train}: platform changed from {old_platform} to {new_platform}"
//     (— is an em dash, Unicode \u{2014})
//
// (c) fn train_name(&self) -> &str
//     Returns the train name from any variant.
//     Hint: Every variant has a `train` field. You can combine arms with |.

// =============================================
// STEP 4: Define `AnnouncementBoard` struct + methods
// =============================================
//
// Fields:
//   station: String
//   events: Vec<Event>
//
// Derive: Debug
//
// Methods:
//
//   fn new(station: &str) -> Self
//     Creates a board with an empty events Vec.
//
//   fn add(&mut self, event: Event)
//     Pushes an event onto the Vec.
//
//   fn display_all(&self)
//     Prints "--- All Announcements ---"
//     Then for each event: "{urgency_prefix} {announcement}"
//
//   fn display_urgent(&self)
//     Prints "--- Urgent & Critical Only ---"
//     Same format, but only events where urgency is Urgent or Critical.
//     Hint: compare with == (that's why Urgency derives PartialEq)
//
//   fn summary(&self)
//     Counts each event type using match, then prints:
//       --- Event Summary ---
//       Departures: {n}
//       Arrivals: {n}
//       Delays: {n}
//       Cancellations: {n}
//       Platform changes: {n}
//       Total: {n}

// =============================================
// STEP 5: Write main()
// =============================================
//
// Your main() should do the following:
//
// 1. Print "=== CHENNAI CENTRAL ANNOUNCEMENTS ==="
//
// 2. Create an AnnouncementBoard for "Chennai Central"
//
// 3. Add these 8 events (in order):
//      Departure:       "Rajdhani Express",  platform 3,  to "New Delhi"
//      Arrival:         "Shatabdi Express",  platform 7,  from "Bangalore"
//      Delay:           "Duronto Express",   45 min,      "Signal failure near Katpadi"
//      Cancellation:    "Nilgiri Express",                "Landslide on mountain track"
//      PlatformChange:  "Garib Rath",        from 5 to 2
//      Departure:       "Kovai Express",     platform 12, to "Coimbatore"
//      Cancellation:    "Island Express",                 "Flooding in Kerala"
//      Delay:           "Mumbai Mail",       10 min,      "Late arrival of incoming rake"
//
// 4. Print a blank line, then call display_all()
// 5. Print a blank line, then call display_urgent()
// 6. Print a blank line, then call summary()

fn main() {
    // Your code here
}
