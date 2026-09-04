# Lesson 05 — Enums & Pattern Matching

Enums are one of Rust's **most powerful features**. If structs let you say "a Train
has these fields," enums let you say "a TrainStatus is one of these possibilities."
Combined with pattern matching, they give you a way to handle every possible state
in your program — and the compiler **proves** you didn't miss any.

If you've used TypeScript discriminated unions, you'll feel at home. But Rust enums
go further — each variant can carry its own data, and `match` forces you to handle
every case.

---

## 1. Basic Enums

An enum defines a type that can be **one of several variants**:

```rust
enum TrainStatus {
    OnTime,
    Delayed,
    Cancelled,
    Boarding,
}
```

This says: a `TrainStatus` is either `OnTime`, `Delayed`, `Cancelled`, or `Boarding`.
Nothing else. Ever.

### Naming convention

- Enum name → `PascalCase` (`TrainStatus`, `SignalColor`)
- Variant names → `PascalCase` (`OnTime`, `Delayed`) — **not** `SCREAMING_CASE`

### Creating and using enum values

```rust
let status = TrainStatus::OnTime;
let another = TrainStatus::Delayed;
```

Notice the `::` syntax — variants live inside their enum's namespace.

### Why not just use strings?

You might think: "Why not `let status = "on_time"`?" Here's why:

```rust
// With strings — no safety at all
fn announce(status: &str) {
    if status == "on_tme" {  // typo! No compiler error. Silent bug.
        println!("Train is on time");
    }
}

// With enums — the compiler has your back
fn announce(status: TrainStatus) {
    // If you try TrainStatus::OnTme — compiler error. Typo caught instantly.
}
```

Strings are just bytes. The compiler can't check that `"on_time"` is a valid status.
Enums are **types** — the compiler knows every valid variant and rejects anything else.

### Enums as function parameters and return types

```rust
fn get_platform(status: TrainStatus) -> u8 {
    // We'll use match here shortly
    1
}

fn current_status() -> TrainStatus {
    TrainStatus::OnTime
}

let status = current_status();
let platform = get_platform(status);
```

Enums work everywhere types work — function params, return types, struct fields,
you name it.

### Printing enums with `Debug`

Just like structs, add `#[derive(Debug)]` to print them:

```rust
#[derive(Debug)]
enum TrainStatus {
    OnTime,
    Delayed,
    Cancelled,
    Boarding,
}

let status = TrainStatus::Delayed;
println!("Status: {:?}", status);  // Status: Delayed
```

---

## 2. Enums with Data

This is the **killer feature**. In most languages, enums are just labels — glorified
integers. In Rust, each variant can carry its own data, and different variants can
carry **different types** of data:

```rust
#[derive(Debug)]
enum TrainEvent {
    Departed,                                           // no data
    Arrived { station: String },                        // named fields (struct-like)
    Delayed(u32),                                       // one unnamed field (tuple-like)
    SpeedChanged(f64, f64),                             // two unnamed fields (from, to)
    Cancelled { reason: String, refund_percent: u8 },   // multiple named fields
}
```

Let's break this down:

| Variant | Style | Data |
|---------|-------|------|
| `Departed` | Unit (no data) | Just the fact that it happened |
| `Arrived { station }` | Struct-like | Named fields, like a mini-struct |
| `Delayed(u32)` | Tuple-like | Positional data (minutes of delay) |
| `SpeedChanged(f64, f64)` | Tuple-like | Two values (old speed, new speed) |
| `Cancelled { reason, refund_percent }` | Struct-like | Multiple named fields |

### Creating variants with data

```rust
let e1 = TrainEvent::Departed;
let e2 = TrainEvent::Arrived { station: String::from("Chennai Central") };
let e3 = TrainEvent::Delayed(45);
let e4 = TrainEvent::SpeedChanged(80.0, 120.0);
let e5 = TrainEvent::Cancelled {
    reason: String::from("Track maintenance"),
    refund_percent: 100,
};
```

Each of these is the same type: `TrainEvent`. You can put them all in one `Vec<TrainEvent>`,
pass them to the same function, etc. — but each carries different data.

### Why this matters

Think about a real railway system. A "departed" event has no extra info. A "delayed"
event needs the delay duration. A "cancelled" event needs a reason and refund info.
Without enums-with-data, you'd end up with:

```rust
// BAD: The "everything is optional" approach
struct TrainEvent {
    event_type: String,
    station: Option<String>,       // only for arrivals
    delay_minutes: Option<u32>,    // only for delays
    reason: Option<String>,        // only for cancellations
    refund_percent: Option<u8>,    // only for cancellations
    old_speed: Option<f64>,        // only for speed changes
    new_speed: Option<f64>,        // only for speed changes
}
```

That's messy and error-prone. With enums, each variant carries **exactly** the data
it needs — nothing more, nothing less.

### Comparison to TypeScript

This is the closest TypeScript equivalent:

```typescript
// TypeScript discriminated union
type TrainEvent =
  | { type: "departed" }
  | { type: "arrived"; station: string }
  | { type: "delayed"; minutes: number }
  | { type: "speedChanged"; from: number; to: number }
  | { type: "cancelled"; reason: string; refundPercent: number };
```

Rust enums are the same idea, but built into the language with better syntax and
the compiler enforcing exhaustive handling (you'll see this with `match`).

---

## 3. Pattern Matching with `match`

`match` is how you inspect an enum and act on each variant. Think of it as a `switch`
statement on steroids.

```rust
fn describe_event(event: &TrainEvent) {
    match event {
        TrainEvent::Departed => {
            println!("🚂 Train has departed");
        }
        TrainEvent::Arrived { station } => {
            println!("🛬 Arrived at {station}");
        }
        TrainEvent::Delayed(minutes) => {
            println!("⏰ Delayed by {minutes} minutes");
        }
        TrainEvent::SpeedChanged(from, to) => {
            println!("💨 Speed changed from {from} to {to} km/h");
        }
        TrainEvent::Cancelled { reason, refund_percent } => {
            println!("❌ Cancelled: {reason} (refund: {refund_percent}%)");
        }
    }
}
```

When you match a tuple-like variant (`Delayed(minutes)`), you **bind** the inner
data to a variable name of your choice. For struct-like variants, you use the field
names (`{ station }`), or rename them (`{ station: s }`).

### Exhaustive matching — the superpower

**You must handle every variant.** If you add a new variant to `TrainEvent` and
forget to update a `match`, the compiler **refuses to compile**:

```rust
enum TrainEvent {
    Departed,
    Arrived { station: String },
    Delayed(u32),
    SpeedChanged(f64, f64),
    Cancelled { reason: String, refund_percent: u8 },
    Rerouted(String),  // NEW variant
}

// Now every match on TrainEvent will fail to compile until you handle Rerouted.
// This is incredible for refactoring safety.
```

### The `_` catch-all pattern

Sometimes you don't need to handle every variant individually:

```rust
fn is_running(event: &TrainEvent) -> bool {
    match event {
        TrainEvent::Cancelled { .. } => false,
        _ => true,  // everything else means the train is still running
    }
}
```

`_` matches anything. `{ .. }` means "this variant has fields but I don't care
about them." Use these sparingly — exhaustive matching is usually what you want.

### Match guards with `if`

Add conditions to match arms:

```rust
fn describe_delay(event: &TrainEvent) {
    match event {
        TrainEvent::Delayed(mins) if *mins <= 5 => {
            println!("Slight delay — {mins} minutes");
        }
        TrainEvent::Delayed(mins) if *mins <= 30 => {
            println!("Moderate delay — {mins} minutes");
        }
        TrainEvent::Delayed(mins) => {
            println!("Severe delay — {mins} minutes!");
        }
        _ => {}
    }
}
```

The `if` condition is called a **match guard**. The final `Delayed(mins)` without
a guard acts as the catch-all for that variant.

### Binding with `@`

The `@` operator lets you bind a value to a name **while also** testing it against
a pattern:

```rust
fn check_speed(event: &TrainEvent) {
    match event {
        TrainEvent::SpeedChanged(_, to @ 0.0) => {
            println!("Train has stopped (speed: {to})");
        }
        TrainEvent::SpeedChanged(_, to @ 1.0..=60.0) => {
            println!("Moving slowly at {to} km/h");
        }
        TrainEvent::SpeedChanged(from, to) if to > from => {
            println!("Accelerating from {from} to {to} km/h");
        }
        TrainEvent::SpeedChanged(from, to) => {
            println!("Decelerating from {from} to {to} km/h");
        }
        _ => {}
    }
}
```

`to @ 1.0..=60.0` means "match a value in the range 1.0 to 60.0, and call it `to`."

### `match` returns a value

Just like `if`, `match` is an expression — it returns a value:

```rust
let announcement = match event {
    TrainEvent::Departed => String::from("The train has departed."),
    TrainEvent::Delayed(mins) => format!("Delayed by {} minutes.", mins),
    TrainEvent::Arrived { station } => format!("Now arriving at {}.", station),
    TrainEvent::Cancelled { reason, .. } => format!("Cancelled: {}.", reason),
    TrainEvent::SpeedChanged(_, to) => format!("Current speed: {} km/h.", to),
};

println!("{announcement}");
```

All arms must return the same type.

---

## 4. The `Option<T>` Enum — Rust's Null Replacement

This is **critical**. Understand this and you understand half of Rust's safety story.

Most languages have some concept of "no value": `null` in Java/C#, `nil` in Ruby/Go,
`None` in Python, `undefined`/`null` in JavaScript. These are the source of a
staggering number of bugs — Tony Hoare (who invented null) literally calls it his
["billion dollar mistake"](https://en.wikipedia.org/wiki/Tony_Hoare#Apologies_and_retractions).

**Rust has no null.** None. Zero. Nada.

Instead, Rust uses an enum called `Option<T>`:

```rust
// This is defined in the standard library. You don't write this yourself.
enum Option<T> {
    Some(T),    // there IS a value of type T
    None,       // there is NO value
}
```

`T` is a generic type parameter (we'll cover generics properly later). For now,
just read `Option<T>` as "maybe a T."

- `Option<String>` → maybe a String
- `Option<u32>` → maybe a u32
- `Option<Train>` → maybe a Train

### It's in the prelude — no imports needed

`Option`, `Some`, and `None` are used so often that Rust puts them in the prelude
(automatically imported into every file). You write `Some(42)` and `None` directly,
without `Option::Some(42)` or any `use` statement.

### Real examples

**Looking up a train by number:**

```rust
fn find_train(trains: &[Train], number: u32) -> Option<&Train> {
    for train in trains {
        if train.number == number {
            return Some(train);
        }
    }
    None  // no train found with that number
}
```

**Finding an available seat:**

```rust
fn find_available_seat(coach: &Coach) -> Option<u32> {
    for seat in &coach.seats {
        if !seat.is_occupied {
            return Some(seat.number);
        }
    }
    None
}
```

**Getting an element from a vector:**

```rust
let platforms = vec![1, 2, 3, 5, 8];
let third: Option<&i32> = platforms.get(2);    // Some(&3)
let tenth: Option<&i32> = platforms.get(9);    // None — no panic!
```

Notice: `platforms.get(9)` returns `None` instead of crashing. Compare this to
`platforms[9]` which **would** panic. The `Option` return type makes the possibility
of "not found" explicit in the type system.

### Why this is better than null

In other languages, **any** reference could be null, and you find out at runtime
when your program crashes. In Rust, the type tells you upfront:

```rust
fn get_train(number: u32) -> Train { ... }         // ALWAYS returns a Train. Guaranteed.
fn find_train(number: u32) -> Option<Train> { ... } // MIGHT return a Train. Handle both cases.
```

You can **never** accidentally use a "null" value because `Option<Train>` is a
different type from `Train`. The compiler forces you to check:

```rust
let maybe_train: Option<Train> = find_train(12001);

// This won't compile — maybe_train isn't a Train, it's an Option<Train>:
// println!("{}", maybe_train.name);  // ❌ ERROR

// You must unwrap it first:
match maybe_train {
    Some(train) => println!("Found: {}", train.name),
    None => println!("Train not found"),
}
```

### Working with `Option` — your toolbox

Matching every `Option` with a full `match` gets verbose. Rust provides a rich set
of methods:

#### `.unwrap()` — get the value or panic

```rust
let seat = find_available_seat(&coach).unwrap();
// If Some → returns the inner value
// If None → PANICS: "called unwrap() on a None value"
```

**Only use this in quick scripts, tests, or when you're 100% certain it's `Some`.**
In production code, prefer the alternatives below.

#### `.unwrap_or(default)` — provide a fallback

```rust
let platform = find_platform(&train).unwrap_or(0);
// If Some(5) → returns 5
// If None → returns 0
```

#### `.unwrap_or_else(|| closure)` — compute a fallback lazily

```rust
let seat = find_available_seat(&coach)
    .unwrap_or_else(|| assign_standing_room(&coach));
// The closure only runs if the value is None
```

#### `.is_some()` / `.is_none()` — check without extracting

```rust
if find_train(trains, 12001).is_some() {
    println!("Train exists!");
}
```

#### `.map()` — transform the inner value if `Some`

```rust
let train_name: Option<String> = find_train(trains, 12001)
    .map(|train| train.name.clone());
// If Some(train) → Some(train.name.clone())
// If None → None
```

`.map()` is incredibly useful for chaining transformations without unwrapping.

#### Chaining it all together

```rust
let announcement = find_train(trains, 12001)
    .map(|t| format!("Train {} is ready", t.name))
    .unwrap_or_else(|| String::from("Train not found"));

println!("{announcement}");
```

### Comparison to TypeScript

| TypeScript | Rust |
|------------|------|
| `string \| undefined` | `Option<String>` |
| `if (value !== undefined)` | `if let Some(value) = option` |
| `value ?? "default"` | `.unwrap_or("default")` |
| `value?.toUpperCase()` | `.map(\|v\| v.to_uppercase())` |
| Runtime crash: `Cannot read property of undefined` | Compile error: must handle `None` |

---

## 5. The `Result<T, E>` Enum — Error Handling Preview

`Option` says "maybe a value." `Result` says "either success or failure":

```rust
// Also defined in the standard library
enum Result<T, E> {
    Ok(T),     // success — here's the value
    Err(E),    // failure — here's the error
}
```

Like `Option`, `Result`, `Ok`, and `Err` are in the prelude — no imports needed.

### When to use `Result` vs `Option`

- `Option<T>` — "This might not exist" (no train found, no seat available)
- `Result<T, E>` — "This operation can fail" (parsing, file I/O, network calls)

### Real examples

**Parsing a train number from user input:**

```rust
let input = "12001";
let number: Result<u32, _> = input.parse();

match number {
    Ok(n) => println!("Train number: {n}"),
    Err(e) => println!("Invalid input: {e}"),
}
```

**A function that can fail:**

```rust
#[derive(Debug)]
enum BookingError {
    TrainNotFound,
    NoSeatsAvailable,
    InvalidPassenger(String),
}

fn book_ticket(train_number: u32, passenger: &str) -> Result<String, BookingError> {
    if passenger.is_empty() {
        return Err(BookingError::InvalidPassenger(
            String::from("Name cannot be empty"),
        ));
    }

    // Pretend we look up the train...
    if train_number == 99999 {
        return Err(BookingError::TrainNotFound);
    }

    // Pretend we check seats...
    Ok(format!("Booking confirmed for {passenger} on train {train_number}"))
}
```

**Using the result:**

```rust
match book_ticket(12001, "Priya") {
    Ok(confirmation) => println!("✅ {confirmation}"),
    Err(BookingError::TrainNotFound) => println!("❌ That train doesn't exist"),
    Err(BookingError::NoSeatsAvailable) => println!("❌ No seats left"),
    Err(BookingError::InvalidPassenger(msg)) => println!("❌ Bad input: {msg}"),
}
```

See what happened? The error type is an **enum**, so you can match on specific
failure modes. This is far more precise than throwing exceptions.

### Quick methods on Result

#### `.unwrap()` — get the value or panic on error

```rust
let n: u32 = "12001".parse().unwrap();
// If Ok(12001) → returns 12001
// If Err → panics with the error message
```

Same warning as `Option::unwrap()` — fine for quick scripts, dangerous in production.

#### `.expect("message")` — panic with a custom message

```rust
let n: u32 = "12001".parse().expect("Failed to parse train number");
// If Err → panics with: "Failed to parse train number: <error details>"
```

Better than `.unwrap()` because the panic message tells you **what** went wrong.

#### The `?` operator — propagate errors up

This is the **real** way to handle errors in Rust, and you'll see it everywhere:

```rust
fn parse_and_book(input: &str) -> Result<String, BookingError> {
    let number: u32 = input.parse()
        .map_err(|_| BookingError::InvalidPassenger(
            format!("'{input}' is not a valid train number"),
        ))?;  // ← the ? operator

    book_ticket(number, "Priya")
}
```

The `?` operator does this:
- If `Ok(value)` → unwrap and continue
- If `Err(e)` → **return the error immediately** from the current function

It's like an early return for errors, but cleaner than writing `match` every time.
We'll cover this in depth in Lesson 07 (Error Handling). For now, just know it exists.

### This is what you'll see ALL OVER ESP32 code

Embedded Rust uses `Result` constantly. Setting up a GPIO pin? `Result`. Configuring
WiFi? `Result`. Reading a sensor? `Result`. Hardware can always fail, so every
operation returns a `Result`:

```rust
// Typical ESP32 pattern (preview — don't worry about the types yet)
fn setup_display() -> Result<Display, EspError> {
    let i2c = I2cDriver::new(/* ... */)?;   // might fail
    let display = Display::new(i2c)?;       // might fail
    display.init()?;                        // might fail
    Ok(display)                             // all good!
}
```

Each `?` propagates errors up. If any step fails, the function returns early with
the error. No try/catch, no exception handling — just types and control flow.

---

## 6. `if let` — Concise Pattern Matching

Sometimes you only care about **one** variant. A full `match` is overkill:

```rust
// This works but is verbose for one case:
match find_train(trains, 12001) {
    Some(train) => println!("Found: {}", train.name),
    None => {}  // do nothing — feels like wasted code
}
```

`if let` is syntactic sugar for exactly this:

```rust
if let Some(train) = find_train(trains, 12001) {
    println!("Found: {}", train.name);
}
```

Read it as: "If `find_train(...)` matches `Some(train)`, run this block."

### `if let` with `else`

```rust
if let Some(train) = find_train(trains, 12001) {
    println!("Found: {}", train.name);
} else {
    println!("Train 12001 not found");
}
```

### Works with any enum, not just `Option`

```rust
if let TrainEvent::Delayed(mins) = &event {
    println!("Heads up — delayed by {mins} minutes");
}

if let TrainEvent::Cancelled { reason, .. } = &event {
    println!("Cancelled because: {reason}");
}
```

### When to use `if let` vs `match`

| Situation | Use |
|-----------|-----|
| Need to handle **every** variant | `match` |
| Care about **one** variant, ignore the rest | `if let` |
| Need a value from one variant with a fallback | `if let` + `else` |
| Multiple variants need different handling | `match` |

Rule of thumb: if you find yourself writing `_ => {}` in a `match`, consider `if let`.

---

## 7. `while let` — Loop Until Pattern Fails

`while let` runs a loop as long as a pattern matches. Perfect for draining a
collection:

```rust
let mut boarding_queue: Vec<String> = vec![
    String::from("Amit"),
    String::from("Priya"),
    String::from("Raj"),
];

while let Some(passenger) = boarding_queue.pop() {
    println!("Boarding: {passenger}");
}

println!("All passengers boarded!");
```

Output:
```
Boarding: Raj
Boarding: Priya
Boarding: Amit
All passengers boarded!
```

`Vec::pop()` returns `Option<T>` — it returns `Some(value)` while there are
elements, and `None` when the vector is empty. The `while let` keeps going until
it hits `None`.

Another example — processing departures from a schedule:

```rust
let mut departures: Vec<(u32, String)> = vec![
    (12001, String::from("Rajdhani Express")),
    (12002, String::from("Shatabdi Express")),
    (12003, String::from("Duronto Express")),
];

while let Some((number, name)) = departures.pop() {
    println!("Now departing: {name} (#{number})");
}
```

---

## 8. Methods on Enums

Enums can have methods, just like structs. Use `impl`:

```rust
#[derive(Debug)]
enum TrainStatus {
    OnTime,
    Delayed(u32),       // minutes
    Cancelled(String),  // reason
    Boarding,
}

impl TrainStatus {
    fn description(&self) -> String {
        match self {
            TrainStatus::OnTime => String::from("Running on time"),
            TrainStatus::Delayed(mins) => format!("Delayed by {mins} minutes"),
            TrainStatus::Cancelled(reason) => format!("Cancelled: {reason}"),
            TrainStatus::Boarding => String::from("Now boarding"),
        }
    }

    fn is_running(&self) -> bool {
        match self {
            TrainStatus::Cancelled(_) => false,
            _ => true,
        }
    }

    fn delay_minutes(&self) -> u32 {
        match self {
            TrainStatus::Delayed(mins) => *mins,
            _ => 0,
        }
    }
}
```

Usage:

```rust
let status = TrainStatus::Delayed(25);
println!("{}", status.description());  // "Delayed by 25 minutes"
println!("Running? {}", status.is_running());  // "Running? true"
println!("Delay: {} mins", status.delay_minutes());  // "Delay: 25 mins"
```

### Associated functions on enums (constructors)

```rust
impl TrainStatus {
    fn from_delay(minutes: u32) -> Self {
        if minutes == 0 {
            TrainStatus::OnTime
        } else {
            TrainStatus::Delayed(minutes)
        }
    }
}

let status = TrainStatus::from_delay(0);   // OnTime
let status = TrainStatus::from_delay(15);  // Delayed(15)
```

---

## 9. Enums + Structs Together

Enums and structs are meant to work together. This is where Rust's type system
really shines:

```rust
#[derive(Debug)]
enum TrainStatus {
    OnTime,
    Delayed(u32),
    Cancelled(String),
    Boarding,
}

#[derive(Debug)]
enum CoachClass {
    Sleeper,
    AC3Tier,
    AC2Tier,
    AC1stClass,
    General,
}

#[derive(Debug)]
struct Train {
    name: String,
    number: u32,
    status: TrainStatus,
    class: CoachClass,
    platform: Option<u8>,  // might not have a platform assigned yet
}

impl Train {
    fn new(name: &str, number: u32, class: CoachClass) -> Self {
        Self {
            name: String::from(name),
            number,
            status: TrainStatus::OnTime,
            class,
            platform: None,  // no platform assigned at creation
        }
    }

    fn announce(&self) {
        let platform_info = match self.platform {
            Some(p) => format!("Platform {p}"),
            None => String::from("Platform TBD"),
        };

        match &self.status {
            TrainStatus::OnTime => {
                println!("🟢 {} (#{}) — On Time — {}",
                    self.name, self.number, platform_info);
            }
            TrainStatus::Delayed(mins) => {
                println!("🟡 {} (#{}) — Delayed {mins} min — {}",
                    self.name, self.number, platform_info);
            }
            TrainStatus::Cancelled(reason) => {
                println!("🔴 {} (#{}) — CANCELLED: {}",
                    self.name, self.number, reason);
            }
            TrainStatus::Boarding => {
                println!("🔵 {} (#{}) — NOW BOARDING — {}",
                    self.name, self.number, platform_info);
            }
        }
    }

    fn delay(&mut self, minutes: u32) {
        self.status = TrainStatus::Delayed(minutes);
    }

    fn assign_platform(&mut self, platform: u8) {
        self.platform = Some(platform);
    }

    fn cancel(&mut self, reason: &str) {
        self.status = TrainStatus::Cancelled(String::from(reason));
        self.platform = None;  // release the platform
    }
}
```

Usage:

```rust
let mut rajdhani = Train::new("Rajdhani Express", 12001, CoachClass::AC2Tier);
rajdhani.assign_platform(3);
rajdhani.announce();
// 🟢 Rajdhani Express (#12001) — On Time — Platform 3

rajdhani.delay(20);
rajdhani.announce();
// 🟡 Rajdhani Express (#12001) — Delayed 20 min — Platform 3

rajdhani.cancel("Signal failure");
rajdhani.announce();
// 🔴 Rajdhani Express (#12001) — CANCELLED: Signal failure
```

### Pattern matching on struct fields

You can destructure structs inside a `match` too:

```rust
fn should_notify_passengers(train: &Train) -> bool {
    match (&train.status, &train.class) {
        (TrainStatus::Cancelled(_), _) => true,           // always notify for cancellations
        (TrainStatus::Delayed(mins), _) if *mins > 30 => true,  // long delays
        (_, CoachClass::AC1stClass) => true,               // always notify 1st class
        _ => false,
    }
}
```

Matching on **tuples of enums** like this is a powerful technique. You can combine
multiple pieces of state and handle specific combinations.

---

## 10. Comparison with TypeScript

| TypeScript | Rust |
|------------|------|
| `type Status = "onTime" \| "delayed" \| "cancelled"` | `enum Status { OnTime, Delayed, Cancelled }` |
| `string \| undefined` | `Option<String>` |
| `number \| null` | `Option<i32>` (or `Option<f64>`, etc.) |
| `throw new Error(...)` | `Result<T, E>` — errors are values, not exceptions |
| `try { } catch { }` | `match` on `Result` / the `?` operator |
| Discriminated unions with `type` field | Enums with data — same concept, cleaner syntax |
| `value ?? "default"` | `.unwrap_or("default")` |
| `value?.property` | `.map(\|v\| v.property)` |
| Runtime: `Cannot read property of undefined` | Compile time: "you must handle `None`" |
| `switch (status) { ... }` without exhaustive check | `match` — compiler enforces every variant |

The biggest difference: TypeScript's type checking happens at compile time but
**disappears at runtime**. Rust's enums are real types that exist at every stage.
There's no "just trust me, it'll be fine" — the compiler verifies it.

---

## Mental Model Summary

> **🧠 Key Concepts**
>
> 1. **Enums** define a type that is **one of several variants**. Each variant can
>    optionally carry data.
>
> 2. **`match`** is exhaustive pattern matching — the compiler **forces** you to
>    handle every variant. This is your main tool for working with enums.
>
> 3. **`Option<T>`** replaces null. `Some(value)` means "here's a value," `None`
>    means "no value." The compiler forces you to check before using the inner value.
>
> 4. **`Result<T, E>`** is for operations that can fail. `Ok(value)` means success,
>    `Err(error)` means failure. Use `?` to propagate errors up.
>
> 5. **`if let`** is shorthand for matching a single variant.
>    **`while let`** loops until a pattern stops matching.
>
> 6. **Enums can have methods** via `impl`, just like structs.
>
> 7. **Enums + structs compose** naturally — use enum fields in structs to model
>    states, and pattern match to handle them.
>
> 8. The core promise: **if it compiles, you handled every case.** No null pointer
>    exceptions, no unhandled error types, no forgotten switch cases.

---

## Up Next

Run the examples in the `examples/` directory to see these concepts in action, then
tackle the exercise to build your own railway system using enums and pattern matching! 🚂
