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
fn get_platform(status: &TrainStatus) -> u32 {
    // Only accept valid TrainStatus values
    5
}

fn current_status() -> TrainStatus {
    TrainStatus::OnTime  // Must return a valid variant
}
```

### Printing enums with `Debug`

Enums don't implement `Display` by default, but you can derive `Debug`:

```rust
#[derive(Debug)]
enum TrainStatus {
    OnTime,
    Delayed,
    Cancelled,
    Boarding,
}

let status = TrainStatus::OnTime;
println!("{:?}", status);  // prints: OnTime
```

---

## 2. Enums with Data

Here's where Rust enums really pull ahead of most languages. Each variant can carry
**its own data** — and different variants can carry **different types**:

```rust
enum TrainEvent {
    Departed,                                         // unit variant — no data
    Arrived { station: String, platform: u8 },        // struct-like — named fields
    Delayed(u32),                                     // tuple-like — one value
    SpeedChanged(f64, f64),                           // tuple-like — two values (old, new)
    Cancelled { reason: String, refund_percent: u8 }, // struct-like — named fields
}
```

Three kinds of variants:

| Kind | Syntax | Example |
|------|--------|---------|
| **Unit** | `Departed` | No data — just the variant itself |
| **Tuple-like** | `Delayed(u32)` | Positional data, like a tuple |
| **Struct-like** | `Arrived { station: String, platform: u8 }` | Named fields, like a struct |

### Creating variants with data

```rust
let e1 = TrainEvent::Departed;
let e2 = TrainEvent::Delayed(15);
let e3 = TrainEvent::Arrived {
    station: String::from("Chennai Central"),
    platform: 3,
};
let e4 = TrainEvent::SpeedChanged(80.0, 120.0);
let e5 = TrainEvent::Cancelled {
    reason: String::from("Signal failure"),
    refund_percent: 100,
};
```

### Why this matters

Without enums, you'd model events with a struct full of optional fields:

```rust
struct TrainEvent {
    event_type: String,
    station: Option<String>,      // only for Arrived
    delay_minutes: Option<u32>,   // only for Delayed
    reason: Option<String>,       // only for Cancelled
    refund_percent: Option<u8>,   // only for Cancelled
    old_speed: Option<f64>,       // only for SpeedChanged
    new_speed: Option<f64>,       // only for SpeedChanged
}
```

That's a mess. Most fields are `None` most of the time. There's nothing stopping you
from setting `station` on a `Delayed` event — the types don't prevent invalid states.

With enums, **each variant carries exactly the data it needs**. A `Departed` event
can't accidentally have a `reason` field. The compiler won't let you.

### Comparison to TypeScript

In TypeScript, you'd model this with a discriminated union:

```typescript
type TrainEvent =
  | { type: "departed" }
  | { type: "arrived"; station: string; platform: number }
  | { type: "delayed"; minutes: number }
  | { type: "speedChanged"; oldSpeed: number; newSpeed: number }
  | { type: "cancelled"; reason: string; refundPercent: number };
```

Same idea, but Rust's syntax is cleaner — you don't need the `type` discriminant
field. The variant **is** the discriminant.

---

## 3. Pattern Matching with `match`

`match` is how you work with enums. You give it a value, and it runs the code for
whichever variant the value happens to be:

```rust
fn describe_event(event: &TrainEvent) -> String {
    match event {
        TrainEvent::Departed => {
            String::from("Train has departed")
        }
        TrainEvent::Arrived { station, platform } => {
            format!("Arrived at {} on platform {}", station, platform)
        }
        TrainEvent::Delayed(minutes) => {
            format!("Delayed by {} minutes", minutes)
        }
        TrainEvent::SpeedChanged(old, new) => {
            format!("Speed changed from {:.1} to {:.1} km/h", old, new)
        }
        TrainEvent::Cancelled { reason, refund_percent } => {
            format!("Cancelled ({}% refund): {}", refund_percent, reason)
        }
    }
}
```

Each arm **destructures** the variant, binding the inner data to variable names.
`station`, `platform`, `minutes`, `old`, `new`, `reason`, `refund_percent` — these
are all variables you can use in the arm's body.

### Exhaustive matching — the superpower

If you add a new variant to `TrainEvent`:

```rust
enum TrainEvent {
    Departed,
    Arrived { station: String, platform: u8 },
    Delayed(u32),
    SpeedChanged(f64, f64),
    Cancelled { reason: String, refund_percent: u8 },
    Rerouted(String),  // ← new variant!
}
```

Every `match` that handles `TrainEvent` will now **fail to compile** until you add a
`Rerouted` arm. The compiler doesn't let you forget. This is the killer feature —
when you add a state, the compiler tells you every place you need to handle it.

### The `_` catch-all pattern

Sometimes you only care about specific variants:

```rust
fn is_running(event: &TrainEvent) -> bool {
    match event {
        TrainEvent::Cancelled { .. } => false,
        _ => true,  // everything else means the train is still running
    }
}
```

`_` matches anything. Use `..` inside a variant to ignore its fields.

**Be careful:** `_` means "I've thought about this and don't need to handle it
specifically." If you add a new variant later, `_` silently catches it — you won't
get a compiler warning. Use `_` intentionally, not lazily.

### Match guards with `if`

Add conditions to match arms:

```rust
fn describe_delay(status: &TrainStatus) -> &str {
    match status {
        TrainStatus::Delayed(mins) if *mins < 10 => {
            "Minor delay, no action needed"
        }
        TrainStatus::Delayed(mins) if *mins < 30 => {
            "Moderate delay, inform passengers"
        }
        TrainStatus::Delayed(mins) if *mins < 60 => {
            "Major delay, offer refreshments"
        }
        TrainStatus::Delayed(_) => {
            "Severe delay, arrange alternate transport"
        }
        _ => "Not a delay",
    }
}
```

The guard (`if *mins < 10`) runs **after** the pattern matches. Notice `*mins` —
when matching on a reference, the bound variable is also a reference. `*` dereferences
it for comparison.

### Binding with `@`

Bind a value to a name while also testing it against a range:

```rust
fn check_speed(event: &TrainEvent) -> String {
    match event {
        TrainEvent::SpeedChanged(_, new) if *new > 200.0 => {
            format!("DANGER: Speed {:.0} exceeds limit!", new)
        }
        TrainEvent::SpeedChanged(old, new @ 100.0..=200.0) => {
            format!("Normal: {:.0} → {:.0} km/h", old, new)
        }
        TrainEvent::SpeedChanged(old, new) => {
            format!("Slow: {:.0} → {:.0} km/h", old, new)
        }
        _ => String::from("Not a speed event"),
    }
}
```

`new @ 100.0..=200.0` means "bind to `new`, but only if it's between 100.0 and
200.0 inclusive."

### `match` returns a value

`match` is an expression — it returns the value of the arm that matched:

```rust
let emoji = match status {
    TrainStatus::OnTime => "🟢",
    TrainStatus::Delayed(_) => "🟡",
    TrainStatus::Cancelled(_) => "🔴",
};
```

All arms must return the same type. This makes `match` great for assigning variables
based on an enum's variant.

---

## 4. Methods on Enums

Enums can have `impl` blocks, just like structs:

```rust
#[derive(Debug)]
enum TrainStatus {
    OnTime,
    Delayed(u32),
    Cancelled(String),
}

impl TrainStatus {
    fn description(&self) -> String {
        match self {
            TrainStatus::OnTime => String::from("On time"),
            TrainStatus::Delayed(mins) => format!("Delayed by {} minutes", mins),
            TrainStatus::Cancelled(reason) => format!("Cancelled: {}", reason),
        }
    }

    fn is_running(&self) -> bool {
        match self {
            TrainStatus::OnTime | TrainStatus::Delayed(_) => true,
            TrainStatus::Cancelled(_) => false,
        }
    }

    fn emoji(&self) -> &str {
        match self {
            TrainStatus::OnTime => "🟢",
            TrainStatus::Delayed(_) => "🟡",
            TrainStatus::Cancelled(_) => "🔴",
        }
    }
}
```

Now you can call methods on any `TrainStatus` value:

```rust
let status = TrainStatus::Delayed(20);
println!("{} {}", status.emoji(), status.description());
// Output: 🟡 Delayed by 20 minutes
```

This is the same `impl` pattern you learned with structs. The only difference is
that your methods will typically `match self` to handle each variant.

### Associated functions on enums (constructors)

You can also define associated functions (no `self` parameter):

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

let status = TrainStatus::from_delay(15);
// Returns TrainStatus::Delayed(15)
```

---

## 5. Enums + Structs Together

Enums and structs compose naturally. A struct field can be an enum, and you pattern
match on it when you need to branch on the state:

```rust
#[derive(Debug)]
enum TrainStatus {
    OnTime,
    Delayed(u32),
    Cancelled(String),
}

struct Train {
    name: String,
    number: u32,
    status: TrainStatus,
}

impl Train {
    fn new(name: &str, number: u32, status: TrainStatus) -> Self {
        Train {
            name: name.to_string(),
            number,
            status,
        }
    }

    fn announce(&self) {
        let msg = match &self.status {
            TrainStatus::OnTime => format!(
                "Train {} ({}) is on time.",
                self.name, self.number
            ),
            TrainStatus::Delayed(mins) => format!(
                "Train {} ({}) is delayed by {} minutes.",
                self.name, self.number, mins
            ),
            TrainStatus::Cancelled(reason) => format!(
                "Train {} ({}) has been cancelled: {}",
                self.name, self.number, reason
            ),
        };
        println!("{}", msg);
    }
}
```

Use it:

```rust
let train = Train::new("Rajdhani Express", 12301, TrainStatus::Delayed(15));
train.announce();
// Train Rajdhani Express (12301) is delayed by 15 minutes.
```

Notice `match &self.status` — we borrow the status rather than moving it. This is
important because `self` is `&Self`, so we can't move fields out of it.

### Pattern matching on struct fields

You can match on a struct's enum field to make decisions:

```rust
fn should_notify_passengers(train: &Train) -> bool {
    match &train.status {
        TrainStatus::Delayed(mins) if *mins > 15 => true,
        TrainStatus::Cancelled(_) => true,
        _ => false,
    }
}
```

This is a very common pattern in Rust: a struct holds state (via an enum field),
and functions use `match` on that state to determine behavior.

---

## 6. Comparison with TypeScript

| TypeScript | Rust |
|------------|------|
| `type Status = "onTime" \| "delayed" \| "cancelled"` | `enum Status { OnTime, Delayed, Cancelled }` |
| Discriminated unions with `type` field | Enums with data — same concept, cleaner syntax |
| `switch (status) { ... }` without exhaustive check | `match` — compiler enforces every variant |
| Extra fields may be present on wrong types | Each variant carries exactly its own data |
| Runtime: typo in string union goes uncaught | Compile time: typo in variant name is an error |
| `switch` fall-through is a footgun | `match` arms don't fall through — ever |
| `default:` catches new cases silently | `_` catch-all is explicit; without it, new variants cause compiler errors |

The biggest difference: TypeScript's type checking happens at compile time but
**disappears at runtime**. Rust's enums are real types that exist at every stage.
There's no "just trust me, it'll be fine" — the compiler verifies it.

---

## Mental Model Summary

> **🧠 Key Concepts**
>
> 1. **Enums** define a type that is **one of several variants**. Each variant can
>    optionally carry data (unit, tuple-like, or struct-like).
>
> 2. **`match`** is exhaustive pattern matching — the compiler **forces** you to
>    handle every variant. This is your main tool for working with enums.
>
> 3. **Match guards** (`if` conditions) let you add extra checks within a match arm.
>    The `_` catch-all pattern handles "everything else."
>
> 4. **Enums can have methods** via `impl`, just like structs. Methods typically
>    `match self` to handle each variant.
>
> 5. **Enums + structs compose** naturally — use enum fields in structs to model
>    states, and pattern match to handle them.
>
> 6. The core promise: **if it compiles, you handled every case.** No forgotten
>    switch cases, no invalid states, no silent bugs from typos in strings.

---

## Up Next

Run the examples in the `examples/` directory to see these concepts in action, then
tackle the exercise to build your own Train Status Dashboard using enums and pattern
matching! 🚂
