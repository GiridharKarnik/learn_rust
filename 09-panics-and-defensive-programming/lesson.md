# Lesson 09 — Panics & Defensive Programming

In Lesson 06 you learned to use `Option` and `Result` to handle errors gracefully. But
what happens when things go *really* wrong — when your code hits a state so bad that it
can't continue? That's a **panic**.

Think of it like an emergency brake on a train. Pulling the emergency brake stops the
entire train dead — passengers are jolted, schedules are ruined, and the whole system
grinds to a halt. You *have* it for genuine emergencies, but you wouldn't pull it just
because one passenger lost their ticket. That's what `Result` is for.

In this lesson, you'll learn what causes panics, when they're appropriate, and — most
importantly — how to write **defensive code** that avoids them entirely.

---

## 1. What Is a Panic?

A panic is an **unrecoverable error**. When Rust panics, it:

1. Prints an error message with the file and line number
2. **Unwinds the stack** — drops all local variables, cleans up resources
3. **Terminates the program**

That's it. Game over. No second chances.

```rust
panic!("Something went terribly wrong!");
// Program prints the message and exits immediately
```

### Why This Matters for Embedded (ESP32)

On a normal computer, a panic prints a message and the program exits. Annoying, but
manageable. On an ESP32 microcontroller:

- **Panic = reboot.** The entire device restarts.
- There's no screen to show the error message.
- Your departure board goes blank for several seconds while the chip reboots.
- If the panic happens in a loop, you get an infinite reboot cycle.

Every panic in embedded code is a **visible failure** to anyone watching. That's why we
learn to avoid them.

---

## 2. What Causes Panics?

Here are the most common panic triggers. Knowing them is the first step to avoiding them.

### `unwrap()` on `None`

```rust
let next_train: Option<&str> = None;
next_train.unwrap();
// 💥 thread 'main' panicked at 'called `Option::unwrap()` on a `None` value'
```

### `unwrap()` on `Err`

```rust
let platform: Result<i32, _> = "not_a_number".parse::<i32>();
platform.unwrap();
// 💥 thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ...'
```

### Index Out of Bounds

```rust
let platforms = vec![1, 2, 3];
let p = platforms[10];
// 💥 thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 10'
```

### Integer Overflow (Debug Mode)

```rust
let passengers: u8 = 255;
let more = passengers + 1;
// 💥 thread 'main' panicked at 'attempt to add with overflow'
// (In release mode this wraps to 0 instead — also bad, just silent!)
```

### Integer Division by Zero

```rust
let total_fare = 1000;
let num_passengers = 0;
let fare_per_person = total_fare / num_passengers;
// 💥 thread 'main' panicked at 'attempt to divide by zero'
```

### Explicit `panic!()` Macro

```rust
panic!("Derailment detected — emergency stop!");
// 💥 Always panics. That's its job.
```

### `todo!()` and `unimplemented!()`

These are placeholders that panic at runtime — useful during development, dangerous in
production:

```rust
fn calculate_dynamic_fare(base: f64, demand: f64) -> f64 {
    todo!() // 💥 panics with "not yet implemented"
}

fn legacy_ticketing_system() {
    unimplemented!() // 💥 panics with "not implemented"
}
```

### Summary Table

| Cause                      | Example                              | Panic Message                              |
|----------------------------|--------------------------------------|--------------------------------------------|
| `unwrap()` on `None`       | `None::<i32>.unwrap()`               | called `Option::unwrap()` on a `None` value |
| `unwrap()` on `Err`        | `"abc".parse::<i32>().unwrap()`      | called `Result::unwrap()` on an `Err` value |
| Index out of bounds        | `vec![1,2,3][10]`                    | index out of bounds                        |
| Integer overflow (debug)   | `255u8 + 1`                          | attempt to add with overflow               |
| Division by zero           | `1 / 0`                              | attempt to divide by zero                  |
| `panic!()`                 | `panic!("oh no")`                    | oh no                                      |
| `todo!()`                  | `todo!()`                            | not yet implemented                        |
| `unimplemented!()`         | `unimplemented!()`                   | not implemented                            |

---

## 3. The `panic!` Macro — When to Use It Intentionally

Despite everything above, there *are* valid reasons to panic. The key question is:
**Is this a bug in the code, or an expected runtime condition?**

### Truly Impossible States

If your code reaches a state that should be logically impossible, a panic is correct —
it signals a **programmer error** that needs fixing:

```rust
enum TrainClass {
    FirstAC,
    SecondAC,
    ThirdAC,
    Sleeper,
    General,
}

fn seat_capacity(class: &TrainClass) -> u32 {
    match class {
        TrainClass::FirstAC  => 18,
        TrainClass::SecondAC => 46,
        TrainClass::ThirdAC  => 64,
        TrainClass::Sleeper  => 72,
        TrainClass::General  => 90,
        // If you add a new variant and forget to update this match,
        // the compiler will catch it. But in code that uses integer
        // conversions or FFI, you might need:
        // _ => panic!("Unknown train class — this is a bug"),
    }
}
```

### Assertions — Catching Bugs Early

Assertions are panics with a purpose: they verify assumptions during development.

```rust
fn set_platform(platform: u32) {
    assert!(platform > 0, "Platform numbers start at 1, got {}", platform);
    assert!(platform <= 24, "Station only has 24 platforms, got {}", platform);
    // ... assign train to platform
}

let speed_expected = 130;
let speed_actual = 130;
assert_eq!(speed_expected, speed_actual, "Speed mismatch!");

let departures = 10;
let arrivals = 8;
assert_ne!(departures, arrivals, "Departures and arrivals shouldn't be equal");
```

- `assert!()` — panics if the condition is `false`
- `assert_eq!()` — panics if two values aren't equal
- `assert_ne!()` — panics if two values *are* equal

All three accept an optional message as the last argument.

> **Note:** In tests, `assert!` macros are *expected* — they're how you verify behavior.
> In production code, use them sparingly, only for invariants that indicate bugs.

### `unreachable!()` — Code That Should Never Execute

```rust
fn direction_from_code(code: u8) -> &'static str {
    match code {
        0 => "Northbound",
        1 => "Southbound",
        _ => unreachable!("Direction code must be 0 or 1, got {}", code),
    }
}
```

`unreachable!()` panics with a message indicating the code path should never be hit.
Use it when you *know* the branch is impossible but the compiler can't prove it.

---

## 4. The Unwrap Ladder — From Dangerous to Safe

This is one of the most practical things you'll learn. Here's the same operation — looking
up a train in a schedule — done seven different ways, from most dangerous to safest:

```rust
use std::collections::HashMap;

let mut schedule: HashMap<&str, &str> = HashMap::new();
schedule.insert("12001", "Shatabdi Express — Platform 3");
schedule.insert("12007", "Duronto Express — Platform 1");
```

### Level 1: `.unwrap()` — 💥 Panics on `None`

```rust
let info = schedule.get("99999").unwrap();
// 💥 PANIC! No context about what went wrong.
```

The worst option. You get a generic panic message with no clue what you were looking for
or why it was missing.

### Level 2: `.expect("message")` — 💥 Panics with Context

```rust
let info = schedule.get("99999").expect("Train 99999 not in schedule");
// 💥 PANIC! But at least the message tells you what was missing.
```

Better for debugging, but still crashes the program. Use `.expect()` during prototyping
or when `None` genuinely means a bug in your code.

### Level 3: `.unwrap_or(default)` — ✅ Returns a Fallback

```rust
let info = schedule.get("99999").unwrap_or(&"No information available");
// Returns "No information available" — no panic
```

Simple and safe. The default is always evaluated, even if the value exists.

### Level 4: `.unwrap_or_else(|| ...)` — ✅ Computes Fallback Lazily

```rust
let info = schedule.get("99999").unwrap_or_else(|| {
    println!("Warning: Train 99999 not found, using fallback");
    &"Service information unavailable — check station display"
});
```

The closure only runs if the value is `None`. Use this when computing the default is
expensive or has side effects.

### Level 5: `.unwrap_or_default()` — ✅ Uses Type's Default

```rust
let delay_minutes: Option<i32> = None;
let delay = delay_minutes.unwrap_or_default();
// Returns 0 — the default for i32
```

Works on any type that implements `Default`: `0` for numbers, `""` for `String`,
`false` for `bool`, empty `Vec` for vectors.

### Level 6: `match` / `if let` — ✅ Full Explicit Control

```rust
// match — handle both cases explicitly
match schedule.get("12001") {
    Some(info) => println!("Train found: {}", info),
    None => println!("Train not in schedule — check departures board"),
}

// if let — when you only care about the Some case
if let Some(info) = schedule.get("12001") {
    println!("Departure info: {}", info);
}
```

Maximum clarity. The reader sees exactly what happens in each case.

### Level 7: `?` Operator — ✅ Propagate to Caller

```rust
fn get_departure_info(
    schedule: &HashMap<&str, &str>,
    train_number: &str,
) -> Option<String> {
    let info = schedule.get(train_number)?;  // returns None to caller if missing
    Some(format!("DEPARTURE: {}", info))
}
```

The `?` on an `Option` returns `None` to the caller if the value is missing. Clean,
composable, and doesn't crash.

### The Ladder at a Glance

```
.unwrap()                      — 💥 panics, no context
.expect("msg")                 — 💥 panics with message
.unwrap_or(default)            — ✅ fallback value (always evaluated)
.unwrap_or_else(|| compute())  — ✅ fallback value (lazy)
.unwrap_or_default()           — ✅ type's default (0, "", false, vec![])
match / if let                 — ✅ handle explicitly
?                              — ✅ propagate to caller
```

**Rule of thumb:** In production code, never use `.unwrap()` except in tests or when you
can *prove* the value is `Some`/`Ok`. Reach for the lower rungs of the ladder instead.

---

## 5. Defensive Patterns — How to Write Panic-Free Code

Pattern by pattern, here's how to replace every common panic trigger with a safe
alternative.

### Safe Indexing: `vec[i]` → `vec.get(i)`

```rust
let platforms = vec!["Platform 1", "Platform 2", "Platform 3"];

// ❌ DANGEROUS — panics if index is out of bounds
// let p = platforms[10];

// ✅ SAFE — returns Option<&T>
match platforms.get(10) {
    Some(p) => println!("Platform: {}", p),
    None => println!("No platform at that index"),
}
```

`vec.get(i)` returns `Some(&value)` if the index is valid, `None` otherwise. **Never panics.**

### Safe HashMap Access: `map[key]` → `map.get(key)`

```rust
use std::collections::HashMap;

let mut gates: HashMap<&str, u32> = HashMap::new();
gates.insert("North Gate", 3);

// ❌ DANGEROUS — panics if key is missing
// let g = gates["South Gate"];

// ✅ SAFE — returns Option<&V>
let g = gates.get("South Gate").unwrap_or(&0);
println!("South Gate capacity: {}", g);
```

### Safe Parsing: Always Handle the `Result`

```rust
let input = "not_a_number";

// ❌ DANGEROUS
// let n: i32 = input.parse().unwrap();

// ✅ SAFE — match on the Result
let n: i32 = match input.parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Invalid input '{}', defaulting to 0", input);
        0
    }
};
```

### Safe Division: Check for Zero First

```rust
fn fare_per_person(total_fare: u32, passengers: u32) -> Option<u32> {
    if passengers == 0 {
        None  // No passengers — can't divide
    } else {
        Some(total_fare / passengers)
    }
}

match fare_per_person(5000, 0) {
    Some(fare) => println!("₹{} per person", fare),
    None => println!("No passengers to split the fare"),
}
```

### Safe Unwrap Alternative: `if let` Instead of `.unwrap()`

```rust
let train_name: Option<&str> = Some("Rajdhani Express");

// ❌ This works BUT will panic if the Option becomes None later
// println!("{}", train_name.unwrap());

// ✅ Pattern match — safe and explicit
if let Some(name) = train_name {
    println!("Next departure: {}", name);
} else {
    println!("No train scheduled");
}
```

### Validating Input Early — Return `Result`

```rust
#[derive(Debug)]
enum BookingError {
    InvalidTrainNumber,
    InvalidPassengerCount,
    TrainNotFound,
}

fn book_ticket(
    train_number: &str,
    passengers: u32,
) -> Result<String, BookingError> {
    // Validate early — before doing any real work
    if train_number.is_empty() {
        return Err(BookingError::InvalidTrainNumber);
    }
    if passengers == 0 || passengers > 6 {
        return Err(BookingError::InvalidPassengerCount);
    }

    // ... look up train, check availability ...
    Ok(format!("Booked {} seats on train {}", passengers, train_number))
}
```

The pattern: **validate first, fail fast with `Result`, never panic on bad input.**

---

## 6. `Result` vs `panic!` — When to Use Which

This is the decision you'll make dozens of times in every project. Here's the guide:

| Situation                        | Use            | Why                                    |
|----------------------------------|----------------|----------------------------------------|
| Invalid user input               | `Result`       | User can fix it                        |
| File not found                   | `Result`       | Expected failure                       |
| Network timeout                  | `Result`       | Caller can retry                       |
| Sensor read failure (ESP32)      | `Result`       | Use last known value                   |
| Train not found in schedule      | `Result`       | Missing data, not a bug                |
| Bug in your logic                | `panic!`       | Fix the code                           |
| Out of memory                    | `panic!`       | Can't recover                          |
| "This should never happen"       | `unreachable!` | Means there's a bug if it does         |
| Violated invariant in a `match`  | `panic!`       | Compiler couldn't prove exhaustiveness |

**The rule of thumb:** If the **caller** can reasonably handle the failure → `Result`.
If it's a **programmer error** that means the code is wrong → `panic!`.

Another way to think about it: Would you want your departure board to **reboot** because
a train number wasn't found? No — you'd want it to display "Information unavailable"
and keep running. That's a `Result`.

---

## 7. ESP32 Context — Why This Matters for Embedded

Everything in this lesson matters double for embedded programming. Here's what changes
when your code runs on a microcontroller instead of a laptop:

### No Screen, No User

On a desktop, a panic prints a nice error to the terminal. On an ESP32, the device simply
reboots. Nobody sees the error message (unless you're watching the serial console during
development).

### Panic = Reboot = Visible Failure

Your railway departure board goes blank for 3–5 seconds while the ESP32 restarts. If
the panic is in the main loop, it reboots *repeatedly* — an infinite restart cycle.

### Watchdog Timers

ESP32 has hardware watchdog timers that reset the device if your code hangs for too long.
A panic during stack unwinding might trigger the watchdog, compounding the problem.

### Defensive Patterns for Embedded

```rust
// ❌ On ESP32, this reboots the device
let temp = sensor.read_temperature().unwrap();

// ✅ Use last known value if sensor read fails
let temp = match sensor.read_temperature() {
    Ok(t) => {
        last_known_temp = t;
        t
    }
    Err(e) => {
        log::warn!("Sensor read failed: {:?}, using last known value", e);
        last_known_temp
    }
};
```

**Embedded rules of thumb:**

- **Never use `.unwrap()`** on anything that touches hardware (sensors, I/O, network)
- **Always return `Result`** from functions that can fail
- **Log errors** over serial before falling back to defaults
- **Defensive defaults**: if a sensor fails, use the last known good value
- **Graceful degradation**: show "---" on the display instead of crashing

---

## 8. Mental Model Summary

```
┌────────────────────────────────────────────────────────────┐
│                    Error Decision Tree                      │
│                                                            │
│  Can the caller handle this failure?                       │
│     YES ──→ Return Result<T, E>                            │
│              • Invalid input    • File not found            │
│              • Network timeout  • Sensor failure            │
│                                                            │
│     NO ──→ Is this a bug in the code?                      │
│              YES ──→ panic! / assert! / unreachable!        │
│              NO  ──→ Rethink your design — most things      │
│                      can be handled with Result             │
└────────────────────────────────────────────────────────────┘
```

**The Unwrap Ladder — commit this to memory:**

```
  DANGEROUS                                            SAFE
     │                                                   │
     ▼                                                   ▼
  .unwrap()  →  .expect()  →  .unwrap_or()  →  match  →  ?
     💥            💥             ✅              ✅       ✅
  no context   with message   with fallback    explicit  propagate
```

**Defensive replacements:**

```
vec[i]         →  vec.get(i)           returns Option
map[key]       →  map.get(key)         returns Option
s.parse().unwrap()  →  s.parse()?      returns Result
a / b          →  check b != 0 first   returns Option/Result
x.unwrap()     →  if let Some(x) = ... pattern match
```

In embedded systems, every avoided panic is a device that keeps running instead of
rebooting. Write defensive code. Use `Result`. Save the `panic!` for genuine bugs.

---

## Up Next

In the exercise, you'll build a **Railway Booking Validator** — a system that processes
ticket bookings defensively. You'll replace every `.unwrap()` with safe alternatives,
validate user input, handle missing data gracefully, and make sure your code never panics
no matter what input it receives.

`cd exercise && cargo run` — and remember: no panics allowed! 🚂
